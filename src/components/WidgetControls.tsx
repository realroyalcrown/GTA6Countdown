import { requestHide } from "../lib/backend";
import type { StackingMode, WidgetSize } from "../types";

interface WidgetControlsProps {
  size: WidgetSize;
  stacking: StackingMode;
  onSizeChange: (size: WidgetSize) => void;
  onStackingChange: (mode: StackingMode) => void;
}

const SIZES: ReadonlyArray<{ id: WidgetSize; short: string; label: string }> = [
  { id: "small", short: "S", label: "Small widget" },
  { id: "medium", short: "M", label: "Medium widget" },
  { id: "large", short: "L", label: "Large widget" },
];

/** Revealed on hover so the widget stays clean while idle. */
export function WidgetControls({
  size,
  stacking,
  onSizeChange,
  onStackingChange,
}: WidgetControlsProps) {
  const floating = stacking === "alwaysOnTop";

  return (
    <div className="controls">
      <div className="controls__group" role="group" aria-label="Widget size">
        {SIZES.map((option) => (
          <button
            key={option.id}
            type="button"
            className="controls__button"
            aria-label={option.label}
            aria-pressed={option.id === size}
            onClick={() => onSizeChange(option.id)}
          >
            {option.short}
          </button>
        ))}
      </div>

      <div className="controls__group">
        <button
          type="button"
          className="controls__button"
          aria-label={floating ? "Pin back to desktop" : "Float above windows"}
          aria-pressed={floating}
          onClick={() => onStackingChange(floating ? "desktop" : "alwaysOnTop")}
        >
          <PinIcon filled={floating} />
        </button>
        <button
          type="button"
          className="controls__button"
          aria-label="Hide widget"
          onClick={() => void requestHide()}
        >
          <CloseIcon />
        </button>
      </div>
    </div>
  );
}

function PinIcon({ filled }: { filled: boolean }) {
  return (
    <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true">
      <path
        d="M9.6 1.4 14.6 6.4 12.9 8.1 12 7.9 9 10.9 9.3 13.4 8.1 14.6 5.2 11.7 1.9 15 1 14.1 4.3 10.8 1.4 7.9 2.6 6.7 5.1 7 8.1 4 7.9 3.1Z"
        fill={filled ? "currentColor" : "none"}
        stroke="currentColor"
        strokeWidth="1.3"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function CloseIcon() {
  return (
    <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true">
      <path
        d="M4 4 12 12 M12 4 4 12"
        stroke="currentColor"
        strokeWidth="1.6"
        strokeLinecap="round"
      />
    </svg>
  );
}
