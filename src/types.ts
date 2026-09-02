export type WidgetSize = "small" | "medium" | "large";

export type StackingMode = "desktop" | "alwaysOnTop";

export interface WidgetState {
  size: WidgetSize;
  stacking: StackingMode;
  visible: boolean;
}

export interface ReleaseTarget {
  epochMs: number;
  iso: string;
  year: number;
  month: number;
  day: number;
}

export interface TypefaceAsset {
  family: string;
  /** The font file as a `data:` URL. */
  source: string;
  /** Path the font was loaded from, for troubleshooting. */
  origin: string;
}

export interface Remaining {
  days: number;
  hours: number;
  minutes: number;
  seconds: number;
  released: boolean;
}
