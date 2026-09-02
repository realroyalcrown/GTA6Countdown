import type { Remaining } from "../types";

const MS_PER_SECOND = 1_000;
const SECONDS_PER_MINUTE = 60;
const SECONDS_PER_HOUR = 60 * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY = 24 * SECONDS_PER_HOUR;

export const RELEASED: Remaining = {
  days: 0,
  hours: 0,
  minutes: 0,
  seconds: 0,
  released: true,
};

export function remainingUntil(targetMs: number, nowMs: number): Remaining {
  const totalSeconds = Math.floor((targetMs - nowMs) / MS_PER_SECOND);

  if (totalSeconds <= 0) {
    return RELEASED;
  }

  return {
    days: Math.floor(totalSeconds / SECONDS_PER_DAY),
    hours: Math.floor((totalSeconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR),
    minutes: Math.floor((totalSeconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE),
    seconds: totalSeconds % SECONDS_PER_MINUTE,
    released: false,
  };
}

/** Days get as many digits as they need; clock units are always padded. */
export function formatUnit(value: number, pad: number): string {
  return String(value).padStart(pad, "0");
}

/**
 * The release moment as it falls in the machine's own time zone, with the
 * offset spelled out — the offset in force on release day, which is not
 * necessarily today's, since the release lands after the autumn daylight
 * saving change in many regions.
 *
 * Formatting from the absolute instant rather than from calendar parts is what
 * makes the conversion correct: the widget targets local midnight, so a Mac set
 * to another zone shows that zone's wall time for the same instant.
 */
export function formatReleaseMoment(epochMs: number, monthStyle: MonthStyle): string {
  const moment = new Date(epochMs);

  const date = new Intl.DateTimeFormat("en-US", {
    month: monthStyle,
    day: "numeric",
    year: "numeric",
  }).format(moment);

  const clock = new Intl.DateTimeFormat("en-GB", {
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  }).format(moment);

  const zone = new Intl.DateTimeFormat("en-US", { timeZoneName: "short" })
    .formatToParts(moment)
    .find((part) => part.type === "timeZoneName")?.value;

  return zone ? `${date} · ${clock} ${zone}` : `${date} · ${clock}`;
}

type MonthStyle = "long" | "short";
