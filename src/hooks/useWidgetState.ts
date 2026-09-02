import { useCallback, useEffect, useState } from "react";

import {
  FALLBACK_STATE,
  fetchWidgetState,
  onStateChanged,
  requestSize,
  requestStacking,
} from "../lib/backend";
import type { StackingMode, WidgetSize, WidgetState } from "../types";

interface WidgetStateApi {
  state: WidgetState;
  setSize: (size: WidgetSize) => void;
  setStacking: (mode: StackingMode) => void;
}

/**
 * Rust owns the widget state because the tray menu can change it too; this
 * hook subscribes to that single source rather than keeping its own copy.
 */
export function useWidgetState(): WidgetStateApi {
  const [state, setState] = useState<WidgetState>(FALLBACK_STATE);

  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | undefined;

    void fetchWidgetState().then((initial) => {
      if (active) setState(initial);
    });

    void onStateChanged((next) => {
      if (active) setState(next);
    }).then((dispose) => {
      if (active) unlisten = dispose;
      else dispose();
    });

    return () => {
      active = false;
      unlisten?.();
    };
  }, []);

  const setSize = useCallback((size: WidgetSize) => {
    setState((current) => ({ ...current, size }));
    void requestSize(size);
  }, []);

  const setStacking = useCallback((mode: StackingMode) => {
    setState((current) => ({ ...current, stacking: mode }));
    void requestStacking(mode);
  }, []);

  return { state, setSize, setStacking };
}
