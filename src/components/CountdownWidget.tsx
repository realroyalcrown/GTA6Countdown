import { useCountdown } from "../hooks/useCountdown";
import { useGtaTypeface } from "../hooks/useGtaTypeface";
import { useReleaseTarget } from "../hooks/useReleaseTarget";
import { useWidgetState } from "../hooks/useWidgetState";
import { formatReleaseDate, formatReleaseZone } from "../lib/countdown";
import type { ReleaseTarget, WidgetSize } from "../types";
import { CountdownGrid } from "./CountdownGrid";
import { GtaSixMark } from "./GtaSixMark";
import { WidgetBackdrop } from "./WidgetBackdrop";
import { WidgetControls } from "./WidgetControls";

const MARK_HEIGHT = { small: 22, medium: 26, large: 34 } as const;

export function CountdownWidget() {
  const target = useReleaseTarget();
  const remaining = useCountdown(target.epochMs);
  const { state, setSize, setStacking } = useWidgetState();
  const typeface = useGtaTypeface();

  return (
    <main
      className="widget"
      data-size={state.size}
      data-typeface={typeface ? "gta" : "system"}
      style={typeface ? ({ "--display-font": `"${typeface}"` } as React.CSSProperties) : undefined}
      /* "deep" makes the whole surface draggable; Tauri still excludes buttons,
         so the hover controls keep working. */
      data-tauri-drag-region="deep"
    >
      <WidgetBackdrop />

      <div className="widget__content">
        <header className="widget__header">
          <GtaSixMark height={MARK_HEIGHT[state.size]} />
          <div className="widget__titles">
            <p className="widget__eyebrow">Grand Theft Auto</p>
            <p className="widget__caption">
              {remaining.released ? "Out now" : releaseCaption(target, state.size)}
            </p>
          </div>
        </header>

        {remaining.released ? (
          <p className="widget__released">OUT NOW</p>
        ) : (
          <CountdownGrid remaining={remaining} />
        )}

        {state.size === "large" && (
          <p className="widget__footer">PlayStation 5 &middot; Xbox Series X|S</p>
        )}
      </div>

      <WidgetControls
        size={state.size}
        stacking={state.stacking}
        onSizeChange={setSize}
        onStackingChange={setStacking}
      />
    </main>
  );
}

/** Only the large layout has room to spell out which midnight is meant. */
function releaseCaption(target: ReleaseTarget, size: WidgetSize): string {
  const date = formatReleaseDate(target);
  if (size !== "large") return date;

  const zone = formatReleaseZone(target);
  return zone ? `${date} · ${zone}` : date;
}
