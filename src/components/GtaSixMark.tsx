import logo from "../assets/gta6-logo.png";

interface GtaSixMarkProps {
  /** Rendered height in pixels; the width follows the artwork's aspect ratio. */
  height: number;
}

export function GtaSixMark({ height }: GtaSixMarkProps) {
  return (
    <img
      className="gta-mark"
      src={logo}
      alt="Grand Theft Auto VI"
      height={height}
      draggable={false}
    />
  );
}
