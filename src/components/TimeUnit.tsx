interface TimeUnitProps {
  value: string;
  label: string;
}

export function TimeUnit({ value, label }: TimeUnitProps) {
  return (
    <div className="unit">
      <span className="unit__value">
        {/* Pricedown has no tabular figures, so each digit gets a fixed
            advance to stop the seconds from shuffling every tick. */}
        {[...value].map((digit, index) => (
          <span key={index} className="unit__digit">
            {digit}
          </span>
        ))}
      </span>
      <span className="unit__label">{label}</span>
    </div>
  );
}
