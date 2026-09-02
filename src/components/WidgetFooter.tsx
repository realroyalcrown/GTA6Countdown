import { openExternal } from "../lib/backend";

export const AUTHOR_URL = "https://realroyalcrown.eu";

export function WidgetFooter() {
  return (
    <footer className="widget__footer">
      <span>© 2026</span>
      <span className="widget__footer-dot" aria-hidden="true">
        ·
      </span>
      <a
        className="widget__link"
        href={AUTHOR_URL}
        /* A webview has no browser to hand the navigation to, so the click goes
           through the opener plugin instead of following the href. */
        onClick={(event) => {
          event.preventDefault();
          void openExternal(AUTHOR_URL);
        }}
      >
        Created by RealRoyalCrown.eu
      </a>
    </footer>
  );
}
