// Turns the black backdrop of a logo image into transparency.
//
// The source artwork is a JPEG matted on black. Rather than keying out a flat
// colour, alpha is ramped over a narrow luminance band just above black: the
// backdrop disappears, the logo's dark purple outline survives, and antialiased
// edges keep a soft falloff instead of a hard jagged cut.
//
// The result is cropped to its opaque bounds so the logo can be positioned by
// its actual edges. Pass --silhouette for a flat black cutout, which is what a
// macOS menu bar template image needs.
//
// Usage: swift dematte.swift <input> <output.png> [--silhouette]

import AppKit
import CoreGraphics
import Foundation

let arguments = CommandLine.arguments
guard arguments.count == 3 || arguments.count == 4 else {
    FileHandle.standardError.write(
        "usage: dematte.swift <input> <output.png> [--silhouette]\n".data(using: .utf8)!)
    exit(2)
}

let inputURL = URL(fileURLWithPath: arguments[1])
let outputURL = URL(fileURLWithPath: arguments[2])
let wantsSilhouette = arguments.count == 4 && arguments[3] == "--silhouette"

guard
    let source = CGImageSourceCreateWithURL(inputURL as CFURL, nil),
    let image = CGImageSourceCreateImageAtIndex(source, 0, nil)
else {
    FileHandle.standardError.write("could not read \(inputURL.path)\n".data(using: .utf8)!)
    exit(1)
}

let width = image.width
let height = image.height
let bytesPerRow = width * 4
var pixels = [UInt8](repeating: 0, count: bytesPerRow * height)

guard
    let context = CGContext(
        data: &pixels,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: bytesPerRow,
        space: CGColorSpaceCreateDeviceRGB(),
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    )
else {
    FileHandle.standardError.write("could not create drawing context\n".data(using: .utf8)!)
    exit(1)
}

context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))

// Fully transparent at or below `floor`, fully opaque at or above `ceiling`.
let floorLevel: Double = 6
let ceilingLevel: Double = 34

for index in stride(from: 0, to: pixels.count, by: 4) {
    let red = Double(pixels[index])
    let green = Double(pixels[index + 1])
    let blue = Double(pixels[index + 2])
    let brightest = max(red, green, blue)

    let alpha: Double
    if brightest <= floorLevel {
        alpha = 0
    } else if brightest >= ceilingLevel {
        alpha = 1
    } else {
        alpha = (brightest - floorLevel) / (ceilingLevel - floorLevel)
    }

    // The context is premultiplied, so scale the colour with the new alpha.
    // A silhouette drops the colour entirely and keeps only the coverage.
    let source = wantsSilhouette ? (0.0, 0.0, 0.0) : (red, green, blue)
    pixels[index] = UInt8((source.0 * alpha).rounded())
    pixels[index + 1] = UInt8((source.1 * alpha).rounded())
    pixels[index + 2] = UInt8((source.2 * alpha).rounded())
    pixels[index + 3] = UInt8((alpha * 255).rounded())
}

guard let full = context.makeImage() else {
    FileHandle.standardError.write("could not encode output\n".data(using: .utf8)!)
    exit(1)
}

/// Bounding box of every pixel that is not fully transparent.
func opaqueBounds() -> CGRect {
    var minX = width, minY = height, maxX = -1, maxY = -1

    for y in 0..<height {
        for x in 0..<width where pixels[(y * bytesPerRow) + (x * 4) + 3] > 0 {
            minX = min(minX, x)
            maxX = max(maxX, x)
            minY = min(minY, y)
            maxY = max(maxY, y)
        }
    }

    guard maxX >= minX, maxY >= minY else {
        return CGRect(x: 0, y: 0, width: width, height: height)
    }
    return CGRect(x: minX, y: minY, width: maxX - minX + 1, height: maxY - minY + 1)
}

let bounds = opaqueBounds()
let output = full.cropping(to: bounds) ?? full

guard
    let destination = CGImageDestinationCreateWithURL(
        outputURL as CFURL, "public.png" as CFString, 1, nil
    )
else {
    FileHandle.standardError.write("could not encode output\n".data(using: .utf8)!)
    exit(1)
}

CGImageDestinationAddImage(destination, output, nil)
guard CGImageDestinationFinalize(destination) else {
    FileHandle.standardError.write("could not write \(outputURL.path)\n".data(using: .utf8)!)
    exit(1)
}

print("wrote \(outputURL.path) (\(output.width)x\(output.height))")
