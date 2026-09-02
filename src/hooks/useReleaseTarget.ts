import { useEffect, useState } from "react";

import { FALLBACK_TARGET, fetchReleaseTarget } from "../lib/backend";
import type { ReleaseTarget } from "../types";
import { useTimeZone } from "./useTimeZone";

/**
 * Resolves the release instant, re-resolving it whenever the machine's time
 * zone changes so the countdown always targets local midnight where you are.
 */
export function useReleaseTarget(): ReleaseTarget {
  const [target, setTarget] = useState<ReleaseTarget>(FALLBACK_TARGET);
  const zone = useTimeZone();

  useEffect(() => {
    let active = true;

    void fetchReleaseTarget().then((resolved) => {
      if (active) setTarget(resolved);
    });

    return () => {
      active = false;
    };
  }, [zone]);

  return target;
}
