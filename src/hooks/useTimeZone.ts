import { useEffect, useState } from "react";

/**
 * Tracks the machine's IANA time zone and reports when it changes.
 *
 * The release is a local-midnight launch, so the instant being counted down to
 * depends on where the Mac is. Travel across a boundary — or a manual change in
 * System Settings — has to move the target with it. Daylight saving needs no
 * special handling: the target is resolved to an absolute instant that already
 * accounts for the offset in force on the release date.
 */
export function useTimeZone(): string {
  const [zone, setZone] = useState(currentZone);

  useEffect(() => {
    const check = () => setZone(currentZone);

    // Nothing notifies a page of a time zone change, so this polls slowly and
    // also checks at the moments a change is most likely to surface.
    const interval = window.setInterval(check, 30_000);
    document.addEventListener("visibilitychange", check);
    window.addEventListener("focus", check);

    return () => {
      window.clearInterval(interval);
      document.removeEventListener("visibilitychange", check);
      window.removeEventListener("focus", check);
    };
  }, []);

  return zone;
}

function currentZone(): string {
  return Intl.DateTimeFormat().resolvedOptions().timeZone;
}
