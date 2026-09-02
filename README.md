# GTA VI Countdown

A macOS desktop widget and menu bar countdown to the release of **Grand Theft
Auto VI on November 19, 2026**.

The widget sits on the desktop behind your windows like a native macOS widget,
layering a frosted glass pane and a dark smoke wash over your own artwork, with
the remaining days, hours, minutes and seconds in white on top. The same
countdown ticks in the menu bar, where the size and behaviour controls live.

## Requirements

- macOS 13 Ventura or newer (built and verified on macOS 26)
- [Node.js](https://nodejs.org) 20+
- [Rust](https://www.rust-lang.org) 1.82+ — `brew install rust`

## Getting started

```bash
npm install
npm run widget:dev     # run with hot reload
npm run widget:build   # produce a signed-for-local-use .app and .dmg
```

The finished app lands in `src-tauri/target/release/bundle/macos/`. Drag it to
`/Applications`.

To start it at login, either add it under **System Settings → General → Login
Items**, or install the launch agent in `packaging/`, which does the same thing
from the command line:

```bash
cp packaging/com.realroyalcrown.gta6countdown.plist ~/Library/LaunchAgents/
launchctl bootstrap gui/$UID ~/Library/LaunchAgents/com.realroyalcrown.gta6countdown.plist
```

`KeepAlive` is deliberately off, so quitting from the menu bar keeps it closed
until the next login rather than having launchd bring it straight back. Remove
it again with `launchctl bootout gui/$UID/com.realroyalcrown.gta6countdown`.

## Using the widget

The widget has no window chrome. Drag it anywhere by its background; it stays
where you put it, on every Space, across restarts.

Hovering reveals the controls, sizes on the left and window behaviour on the
right:

| Control | Position | Effect |
| --- | --- | --- |
| `S` `M` `L` | Top left | Switch between the three widget sizes |
| Pin | Top right | Toggle between sitting on the desktop and floating above windows |
| `×` | Top right | Hide the widget; bring it back from the menu bar |

The menu bar icon carries the same options plus **Move to Screen Center**, a
link to the author's site and **Quit**. Centring targets the usable area —
between the menu bar and the Dock, on the display the widget is currently on —
rather than the raw screen rectangle, so the result looks centred instead of
sitting high.

Everything in the widget is a single centred column: logo, title, `Release
Countdown`, the release moment with its offset, the countdown, then the footer.

### Sizes

| Preset | Points | Countdown |
| --- | --- | --- |
| Small | 240 × 300 | Two-by-two grid, abbreviated month |
| Medium | 400 × 240 | Single row |
| Large | 540 × 320 | Single row |

The heights follow the stacked layout rather than round numbers. Small is a
portrait tile because the two-by-two grid needs the vertical room; it keeps the
same numeral size as medium, since each of its cells is about as wide as
medium's.

### Desktop vs. floating

By default the window sits at `kCGDesktopIconWindowLevel + 1`: above the
desktop picture and its icons, below every application window. That is what
makes it a desktop widget rather than a floating panel — you see it when you
clear your windows away. Flip the pin control if you would rather have it
always visible.

## Customising

**Background.** Replace `src/assets/background.jpg` with any image and rebuild.
It is drawn with `object-fit: cover`, so a landscape image around 3:2 fits all
three presets without awkward cropping.

**Glass and smoke.** Both layers are plain CSS in
`src/styles/widget.css` — `.backdrop__glass` controls the blur and sheen,
`.backdrop__smoke` the darkening that keeps the numerals readable.

## About the typeface

Every GTA logo since GTA III uses **Pricedown** by Ray Larabie
([Typodermic Fonts](https://typodermicfonts.com/pricedown/)). Pricedown Black is
[a free download](https://www.dafont.com/pricedown.font), and its licence covers
installing it on your Mac — but explicitly *not* embedding it in an application.

So the widget ships no font. At launch the backend looks for a `Pricedown*`
file in `~/Library/Fonts` and `/Library/Fonts` and, if you have installed it
yourself, hands that copy to the interface at runtime. Without it the widget
uses SF Pro and still looks at home on macOS.

To get the GTA look: download Pricedown, double-click the `.otf`, click
**Install Font**, and restart the widget.

## Artwork

The `VI` mark comes from `branding/gta6-logo-source.jpg`. That file is matted on
black rather than transparent, which would show as a black rectangle over the
glass, so two small Swift tools prepare it:

```bash
swift branding/dematte.swift branding/gta6-logo-source.jpg src/assets/gta6-logo.png
swift branding/dematte.swift branding/gta6-logo-source.jpg branding/gta6-logo-silhouette.png --silhouette
swift branding/make-icons.swift src/assets/gta6-logo.png branding/gta6-logo-silhouette.png branding/
npx tauri icon branding/app-icon.png
cp branding/tray.png src-tauri/icons/tray.png
```

`dematte.swift` ramps alpha over a narrow band just above black, so the
backdrop disappears while the logo's dark purple outline and its antialiased
edges survive, then crops to the opaque bounds. `make-icons.swift` composites
the app icon and pads the silhouette to menu bar proportions.

Swap in different artwork by replacing the source image and re-running those
commands.

## Release date

November 19, 2026, as
[announced by Take-Two](https://taketwointeractivesoftwareinc.gcs-web.com/news-releases/news-release-details/rockstar-games-announces-pre-orders-grand-theft-auto-vi)
and listed on [rockstargames.com/VI](https://www.rockstargames.com/VI), for
PlayStation 5 and Xbox Series X|S. The game has slipped twice before, so if
Rockstar moves it again, change the three constants at the top of
`src-tauri/src/countdown.rs`; everything else follows from there.

Rockstar publishes launch times as local midnight, so the countdown targets
00:00 on November 19 **in your own time zone**.

That target is resolved to an absolute instant, which keeps it correct across
the autumn daylight saving change: the release falls after it, and the offset
in force on release day — not today's — is what the maths uses. Moving the Mac
to another time zone changes which instant local midnight means, so the widget
re-resolves the target whenever the zone changes.

Every size prints that instant as wall time with its offset, formatted from the
instant rather than from calendar parts. A Mac in another zone therefore shows
that zone's time for the same moment.

## How it is put together

Tauri v2 with a React 19 frontend, chosen over Electron for a desktop widget
that runs all day: it uses the system WKWebView instead of bundling Chromium,
which keeps the app around 15 MB and its memory footprint a fraction of the
Electron equivalent.

```
src/                      React frontend, function components only
  components/             Widget UI
  hooks/                  Countdown clock, widget state, time zone, typeface
  lib/                    Countdown maths and the Rust bridge
src-tauri/src/
  countdown.rs            Release target and remaining-time maths
  desktop_window.rs       NSWindow level and collection behaviour
  menu_bar.rs             Matches the tray text to the system clock
  placement.rs            Centring and restoring the window position
  tray.rs                 Menu bar icon, live title, menu
  settings.rs             Size, stacking and position persistence
  typeface.rs             Locates an installed Pricedown
  widget_size.rs          The three size presets
branding/                 Logo source and the tools that derive the icons
tools/                    Development helpers
```

Rust owns the state that both surfaces show — release date, size, stacking
mode — so the widget and the menu bar can never disagree. The frontend
subscribes to a `widget://state-changed` event rather than keeping a second
copy.

Two details worth knowing if you modify it:

- The countdown ticks on the wall clock, not on a 1000 ms interval, so it never
  drifts and resynchronises the moment the Mac wakes from sleep.
- `macOSPrivateApi` is enabled because the transparent window the glass layers
  need requires it. That rules out Mac App Store distribution, which does not
  matter for a personal widget.
- The drag region is marked `deep`. Tauri's default only starts a drag on a
  direct hit, which never happens here because the backdrop and content cover
  the window; `deep` accepts the whole subtree while still excluding buttons.
- `menu_bar.rs` reaches the underlying `NSStatusBarButton` to set the font,
  because Tauri exposes the tray title only as a string. It takes the menu
  bar's own point size from AppKit and switches to monospaced digits, so the
  countdown matches the clock beside it without the digits shifting each
  second.

Run the backend tests with `cd src-tauri && cargo test`. To inspect the live
window's position and layer — useful because a desktop-level window hides
behind everything — run `swift tools/window-probe.swift`.

## Licence and trademarks

Personal project, not affiliated with or endorsed by Rockstar Games or
Take-Two Interactive. *Grand Theft Auto* and the `VI` logo are their
trademarks; the logo is included here for personal use only. Pricedown is
Ray Larabie's and is never redistributed by this app. Whatever you put in
`src/assets/background.jpg` is your own business.
