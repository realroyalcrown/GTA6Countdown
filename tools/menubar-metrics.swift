// Measures the vertical extents of the glyphs in a menu bar screenshot.
//
// Aligning the tray icon with the countdown text by eye is unreliable at menu
// bar scale — one pixel of error is a tenth of the cap height. This reports the
// bounding box of every run of columns containing glyph pixels, so the icon's
// top and bottom edges can be compared with the text's directly.
//
// The menu bar composites its contents over the wallpaper, so glyphs are
// neither pure white nor a predictable brightness. Instead of an absolute
// threshold, each column is compared against its own background, sampled from
// the rows above and below the glyph band.
//
// Usage: swift tools/menubar-metrics.swift <screenshot.png> [contrast]

import CoreGraphics
import Foundation
import ImageIO

guard (2...3).contains(CommandLine.arguments.count) else {
    FileHandle.standardError.write(
        "usage: menubar-metrics.swift <screenshot.png> [contrast]\n".data(using: .utf8)!)
    exit(2)
}

let url = URL(fileURLWithPath: CommandLine.arguments[1])
let contrast = CommandLine.arguments.count == 3
    ? Double(CommandLine.arguments[2]) ?? 18
    : 18

guard
    let source = CGImageSourceCreateWithURL(url as CFURL, nil),
    let image = CGImageSourceCreateImageAtIndex(source, 0, nil)
else {
    FileHandle.standardError.write("could not read \(url.path)\n".data(using: .utf8)!)
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
    FileHandle.standardError.write("could not create context\n".data(using: .utf8)!)
    exit(1)
}

context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))

func luminance(x: Int, y: Int) -> Double {
    let offset = (y * bytesPerRow) + (x * 4)
    return 0.2126 * Double(pixels[offset])
        + 0.7152 * Double(pixels[offset + 1])
        + 0.0722 * Double(pixels[offset + 2])
}

/// Rows at the very top and bottom of the menu bar are never covered by glyphs,
/// so they stand in for what the wallpaper is doing behind this column.
let marginRows = max(2, height / 10)
var columnBackground = [Double](repeating: 0, count: width)
for x in 0..<width {
    var samples: [Double] = []
    for y in 0..<marginRows { samples.append(luminance(x: x, y: y)) }
    for y in (height - marginRows)..<height { samples.append(luminance(x: x, y: y)) }
    samples.sort()
    columnBackground[x] = samples[samples.count / 2]
}

func isGlyph(x: Int, y: Int) -> Bool {
    luminance(x: x, y: y) >= columnBackground[x] + contrast
}

struct Span {
    var firstColumn: Int
    var lastColumn: Int
    var top = Int.max
    var bottom = -1
}

var spans: [Span] = []
var current: Span?

for x in 0..<width {
    var top = Int.max
    var bottom = -1

    for y in 0..<height where isGlyph(x: x, y: y) {
        top = min(top, y)
        bottom = max(bottom, y)
    }

    if bottom >= 0 {
        if var open = current {
            open.lastColumn = x
            open.top = min(open.top, top)
            open.bottom = max(open.bottom, bottom)
            current = open
        } else {
            current = Span(firstColumn: x, lastColumn: x, top: top, bottom: bottom)
        }
    } else if let open = current {
        spans.append(open)
        current = nil
    }
}
if let open = current { spans.append(open) }

print("image \(width)x\(height), contrast \(contrast), \(spans.count) spans (y from top)")
for span in spans {
    let columns = span.lastColumn - span.firstColumn + 1
    let rows = span.bottom - span.top + 1
    print(
        "  x \(span.firstColumn)-\(span.lastColumn) (w \(columns))   "
            + "y \(span.top)-\(span.bottom) (h \(rows))"
    )
}
