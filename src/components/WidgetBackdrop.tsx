import backgroundImage from "../assets/background.jpg";

/**
 * The three layers behind the countdown, bottom to top: the artwork, a frosted
 * glass pane that blurs it, and a medium dark smoke wash that guarantees the
 * white numerals stay legible over any part of the image.
 */
export function WidgetBackdrop() {
  return (
    <div className="backdrop" aria-hidden="true">
      <img className="backdrop__image" src={backgroundImage} alt="" />
      <div className="backdrop__glass" />
      <div className="backdrop__smoke" />
    </div>
  );
}
