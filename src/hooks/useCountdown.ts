import { useEffect, useState } from "react";

import { remainingUntil } from "../lib/countdown";
import type { Remaining } from "../types";

/**
 * Ticks once per wall-clock second rather than every 1000ms, so the display
 * never drifts and recovers immediately after the Mac wakes from sleep.
 */
export function useCountdown(targetMs: number): Remaining {
  const [remaining, setRemaining] = useState<Remaining>(() =>
    remainingUntil(targetMs, Date.now()),
  );

  useEffect(() => {
    let timeout: number | undefined;

    const tick = () => {
      const now = Date.now();
      setRemaining(remainingUntil(targetMs, now));
      timeout = window.setTimeout(tick, 1_000 - (now % 1_000));
    };

    const resync = () => {
      window.clearTimeout(timeout);
      tick();
    };

    tick();
    document.addEventListener("visibilitychange", resync);
    window.addEventListener("focus", resync);

    return () => {
      window.clearTimeout(timeout);
      document.removeEventListener("visibilitychange", resync);
      window.removeEventListener("focus", resync);
    };
  }, [targetMs]);

  return remaining;
}
