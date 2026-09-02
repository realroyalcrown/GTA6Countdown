// Reports the widget's live window geometry and layering.
//
// Verifying the widget by eye is awkward: in desktop mode it sits behind every
// other window, so a screenshot proves nothing. This reads the window server
// directly, which needs no accessibility permission, and answers the questions
// that actually matter — is the window on screen, where is it, and at which
// layer.
//
// Usage: swift tools/window-probe.swift [owner-substring]

import CoreGraphics
import Foundation

let needle = (CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : "gta6countdown")
    .lowercased()

let raw = CGWindowListCopyWindowInfo(
    CGWindowListOption(rawValue: 0), kCGNullWindowID)
let windows = (raw as NSArray?) as? [[String: Any]] ?? []

if windows.isEmpty {
    print("window server returned no windows (screen recording permission?)")
}

var matches = 0

for window in windows {
    let owner = window[kCGWindowOwnerName as String] as? String ?? "?"
    // Swift's `contains("")` is false, so an empty needle means "list everything".
    guard needle.isEmpty || owner.lowercased().contains(needle) else { continue }
    matches += 1

    let name = window[kCGWindowName as String] as? String ?? ""
    let layer = window[kCGWindowLayer as String] as? Int ?? 0
    let alpha = window[kCGWindowAlpha as String] as? Double ?? 0
    let onScreen = window[kCGWindowIsOnscreen as String] as? Bool ?? false

    var bounds = CGRect.zero
    if let raw = window[kCGWindowBounds as String] as? [String: Any] {
        CGRectMakeWithDictionaryRepresentation(raw as CFDictionary, &bounds)
    }

    print(
        """
        \(owner) "\(name)"
          layer:    \(layer)
          onscreen: \(onScreen)
          alpha:    \(alpha)
          frame:    \(Int(bounds.origin.x)),\(Int(bounds.origin.y)) \
        \(Int(bounds.width))x\(Int(bounds.height))
        """
    )
}

if matches == 0 {
    print("no windows owned by a process matching \"\(needle)\"")
}
