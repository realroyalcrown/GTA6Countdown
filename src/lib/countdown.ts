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

export function formatReleaseDate(target: ReleaseDateParts): string {
  return new Intl.DateTimeFormat("en-US", {
    month: "long",
    day: "numeric",
    year: "numeric",
  }).format(releaseDay(target));
}

/**
 * The offset in force on release day, which is not necessarily today's: the
 * release falls after the autumn daylight saving change in many regions.
 */
export function formatReleaseZone(target: ReleaseDateParts): string {
  const parts = new Intl.DateTimeFormat("en-US", {
    timeZoneName: "short",
  }).formatToParts(releaseDay(target));

  return parts.find((part) => part.type === "timeZoneName")?.value ?? "";
}

function releaseDay(target: ReleaseDateParts): Date {
  return new Date(target.year, target.month - 1, target.day);
}

interface ReleaseDateParts {
  year: number;
  month: number;
  day: number;
}
