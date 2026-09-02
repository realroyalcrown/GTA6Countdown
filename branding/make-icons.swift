// Builds the app icon and the menu bar template image from the logo artwork.
//
// Usage: swift make-icons.swift <logo.png> <silhouette.png> <out-dir>

import AppKit
import CoreGraphics
import Foundation

let arguments = CommandLine.arguments
guard arguments.count == 4 else {
    FileHandle.standardError.write(
        "usage: make-icons.swift <logo.png> <silhouette.png> <out-dir>\n".data(using: .utf8)!)
    exit(2)
}

let logoURL = URL(fileURLWithPath: arguments[1])
let silhouetteURL = URL(fileURLWithPath: arguments[2])
let outputDirectory = URL(fileURLWithPath: arguments[3])

func loadImage(_ url: URL) -> CGImage {
    guard
        let source = CGImageSourceCreateWithURL(url as CFURL, nil),
        let image = CGImageSourceCreateImageAtIndex(source, 0, nil)
    else {
        FileHandle.standardError.write("could not read \(url.path)\n".data(using: .utf8)!)
        exit(1)
    }
    return image
}

func makeContext(width: Int, height: Int) -> CGContext {
    guard
        let context = CGContext(
            data: nil,
            width: width,
            height: height,
            bitsPerComponent: 8,
            bytesPerRow: 0,
            space: CGColorSpaceCreateDeviceRGB(),
            bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
        )
    else {
        FileHandle.standardError.write("could not create context\n".data(using: .utf8)!)
        exit(1)
    }
    context.interpolationQuality = .high
    return context
}

func write(_ image: CGImage, to url: URL) {
    guard
        let destination = CGImageDestinationCreateWithURL(
            url as CFURL, "public.png" as CFString, 1, nil
        )
    else {
        FileHandle.standardError.write("could not encode \(url.path)\n".data(using: .utf8)!)
        exit(1)
    }
    CGImageDestinationAddImage(destination, image, nil)
    guard CGImageDestinationFinalize(destination) else {
        FileHandle.standardError.write("could not write \(url.path)\n".data(using: .utf8)!)
        exit(1)
    }
    print("wrote \(url.lastPathComponent) (\(image.width)x\(image.height))")
}

/// Fits a rect of `size` inside `bounds`, centred, without distortion.
func fit(_ size: CGSize, into bounds: CGRect) -> CGRect {
    let scale = min(bounds.width / size.width, bounds.height / size.height)
    let width = size.width * scale
    let height = size.height * scale
    return CGRect(
        x: bounds.midX - width / 2,
        y: bounds.midY - height / 2,
        width: width,
        height: height
    )
}

// ---------------------------------------------------------------- app icon

let logo = loadImage(logoURL)
let canvas = 1024
let context = makeContext(width: canvas, height: canvas)

// macOS icons are inset within their canvas, leaving room for the system shadow.
let plate = CGRect(x: 100, y: 100, width: 824, height: 824)
let platePath = CGPath(roundedRect: plate, cornerWidth: 184, cornerHeight: 184, transform: nil)

context.saveGState()
context.addPath(platePath)
context.clip()

let gradient = CGGradient(
    colorsSpace: CGColorSpaceCreateDeviceRGB(),
    colors: [
        CGColor(red: 0.16, green: 0.06, blue: 0.27, alpha: 1),
        CGColor(red: 0.36, green: 0.11, blue: 0.42, alpha: 1),
        CGColor(red: 0.07, green: 0.03, blue: 0.12, alpha: 1),
    ] as CFArray,
    locations: [0, 0.45, 1]
)!
context.drawLinearGradient(
    gradient,
    start: CGPoint(x: plate.minX, y: plate.maxY),
    end: CGPoint(x: plate.maxX, y: plate.minY),
    options: []
)
context.restoreGState()

// Hairline rim, the same touch AppKit gives its own icons.
context.saveGState()
context.addPath(platePath)
context.setStrokeColor(CGColor(red: 1, green: 1, blue: 1, alpha: 0.14))
context.setLineWidth(4)
context.strokePath()
context.restoreGState()

let logoBox = fit(
    CGSize(width: logo.width, height: logo.height),
    into: plate.insetBy(dx: 150, dy: 210)
)
context.setShadow(
    offset: CGSize(width: 0, height: -10),
    blur: 40,
    color: CGColor(red: 0, green: 0, blue: 0, alpha: 0.55)
)
context.draw(logo, in: logoBox)

write(context.makeImage()!, to: outputDirectory.appendingPathComponent("app-icon.png"))

// ------------------------------------------------------- menu bar template

let silhouette = loadImage(silhouetteURL)

// Menu bar glyphs do not fill the bar's full height. The mark is sized to sit
// against the cap height of the adjacent countdown text, and the padding is
// split unevenly — more above than below — so its baseline lines up with the
// text's rather than centring the glyph in the bar.
let trayHeight = 128
let trayTopPadding = 34
let trayBottomPadding = 22
let trayContentHeight = trayHeight - trayTopPadding - trayBottomPadding
let trayWidth =
    Int((Double(silhouette.width) / Double(silhouette.height) * Double(trayContentHeight)).rounded())

let trayContext = makeContext(width: trayWidth, height: trayHeight)
// The context's origin is bottom-left, so the bottom padding is the y offset.
trayContext.draw(
    silhouette,
    in: CGRect(x: 0, y: trayBottomPadding, width: trayWidth, height: trayContentHeight)
)

write(trayContext.makeImage()!, to: outputDirectory.appendingPathComponent("tray.png"))
