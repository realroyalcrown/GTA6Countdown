import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import type {
  ReleaseTarget,
  StackingMode,
  TypefaceAsset,
  WidgetSize,
  WidgetState,
} from "../types";

export const STATE_EVENT = "widget://state-changed";

/**
 * The release date lives in Rust so the widget and the menu bar can never show
 * two different countdowns. This mirror is only used if the bridge is
 * unavailable, which in practice means running the UI in a plain browser.
 */
export const FALLBACK_TARGET: ReleaseTarget = {
  epochMs: new Date(2026, 10, 19, 0, 0, 0, 0).getTime(),
  iso: new Date(2026, 10, 19, 0, 0, 0, 0).toISOString(),
  year: 2026,
  month: 11,
  day: 19,
};

export const FALLBACK_STATE: WidgetState = {
  size: "medium",
  stacking: "desktop",
  visible: true,
};

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function fetchReleaseTarget(): Promise<ReleaseTarget> {
  if (!isTauri()) return FALLBACK_TARGET;
  return invoke<ReleaseTarget>("get_release_target");
}

export async function fetchDisplayTypeface(): Promise<TypefaceAsset | null> {
  if (!isTauri()) return null;
  return invoke<TypefaceAsset | null>("get_display_typeface");
}

export async function fetchWidgetState(): Promise<WidgetState> {
  if (!isTauri()) return FALLBACK_STATE;
  return invoke<WidgetState>("get_widget_state");
}

export async function requestSize(size: WidgetSize): Promise<void> {
  if (!isTauri()) return;
  await invoke("set_widget_size", { size });
}

export async function requestStacking(mode: StackingMode): Promise<void> {
  if (!isTauri()) return;
  await invoke("set_stacking_mode", { mode });
}

export async function requestHide(): Promise<void> {
  if (!isTauri()) return;
  await invoke("hide_widget");
}

export async function requestCenter(): Promise<void> {
  if (!isTauri()) return;
  await invoke("center_window");
}

/** Hands a URL to the default browser; in a plain browser, opens a tab. */
export async function openExternal(url: string): Promise<void> {
  if (!isTauri()) {
    window.open(url, "_blank", "noopener");
    return;
  }
  const { openUrl } = await import("@tauri-apps/plugin-opener");
  await openUrl(url);
}

export async function onStateChanged(
  handler: (state: WidgetState) => void,
): Promise<() => void> {
  if (!isTauri()) return () => {};
  return listen<WidgetState>(STATE_EVENT, (event) => handler(event.payload));
}
