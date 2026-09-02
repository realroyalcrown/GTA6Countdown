import { useEffect, useState } from "react";

import { fetchDisplayTypeface } from "../lib/backend";

/**
 * Pricedown is the typeface behind every GTA logo since GTA III. Its free
 * licence covers installing it on a Mac but not shipping it inside an app, so
 * the widget loads the copy the user installed themselves and otherwise falls
 * back to the system UI font.
 *
 * The lookup happens in Rust because WKWebView hides locally installed fonts
 * from web content, which rules out detecting them from the page.
 */
export function useGtaTypeface(): string | null {
  const [family, setFamily] = useState<string | null>(null);

  useEffect(() => {
    let active = true;

    void (async () => {
      const asset = await fetchDisplayTypeface();
      if (!asset || !active) return;

      try {
        const face = new FontFace(asset.family, `url(${asset.source})`);
        await face.load();
        if (!active) return;

        document.fonts.add(face);
        setFamily(asset.family);
      } catch {
        // A corrupt or unsupported font file just means the system font wins.
      }
    })();

    return () => {
      active = false;
    };
  }, []);

  return family;
}
