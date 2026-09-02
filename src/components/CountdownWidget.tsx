import { useCountdown } from "../hooks/useCountdown";
import { useGtaTypeface } from "../hooks/useGtaTypeface";
import { useReleaseTarget } from "../hooks/useReleaseTarget";
import { useWidgetState } from "../hooks/useWidgetState";
import { formatReleaseMoment } from "../lib/countdown";
import { CountdownGrid } from "./CountdownGrid";
import { GtaSixMark } from "./GtaSixMark";
import { WidgetBackdrop } from "./WidgetBackdrop";
import { WidgetControls } from "./WidgetControls";
import { WidgetFooter } from "./WidgetFooter";

const MARK_HEIGHT = { small: 26, medium: 30, large: 40 } as const;

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
      /* "deep" makes the whole surface draggable; Tauri still excludes buttons
         and links, so the controls and the footer link keep working. */
      data-tauri-drag-region="deep"
    >
      <WidgetBackdrop />

      <div className="widget__content">
        <header className="widget__head">
          <GtaSixMark height={MARK_HEIGHT[state.size]} />
          <h1 className="widget__title">Grand Theft Auto 6</h1>
          <p className="widget__subtitle">Release Countdown</p>
          <p className="widget__release">
            {remaining.released
              ? "Out now"
              : formatReleaseMoment(target.epochMs, state.size === "small" ? "short" : "long")}
          </p>
        </header>

        {remaining.released ? (
          <p className="widget__released">OUT NOW</p>
        ) : (
          <CountdownGrid remaining={remaining} />
        )}

        <WidgetFooter />
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
