import { formatUnit } from "../lib/countdown";
import type { Remaining } from "../types";
import { TimeUnit } from "./TimeUnit";

interface CountdownGridProps {
  remaining: Remaining;
}

export function CountdownGrid({ remaining }: CountdownGridProps) {
  return (
    <div className="grid">
      <TimeUnit value={String(remaining.days)} label="Days" />
      <TimeUnit value={formatUnit(remaining.hours, 2)} label="Hours" />
      <TimeUnit value={formatUnit(remaining.minutes, 2)} label="Minutes" />
      <TimeUnit value={formatUnit(remaining.seconds, 2)} label="Seconds" />
    </div>
  );
}
