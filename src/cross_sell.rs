//! Live GoldevLab microsaas links (same FAMILY as forgeyt / underkb).

use resuma::prelude::*;

const FAMILY: &[(&str, &str, &str)] = &[
    (
        "YouTubeForge",
        "YouTube transcript, MP3, SRT, and translation.",
        "https://forgeyt.com",
    ),
    (
        "UnderKb",
        "Compress images to a real KB target. JPG, WebP, PNG.",
        "https://under200kb.com",
    ),
    (
        "PDFForge",
        "Merge, split, compress PDFs. JPG ↔ PDF and extract text.",
        "https://pdfforge.fly.dev",
    ),
    (
        "PlacaQR",
        "3D-printable QR — stand, tile, keychain, or plaque.",
        "https://placaqr.fly.dev",
    ),
];

/// Compact footer strip pointing at live sister tools.
pub fn sister_apps_links() -> View {
    let items = FAMILY
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (name, _, href))| {
            let name = name.to_string();
            let href = href.to_string();
            if i == 0 {
                view! { <a href={href} rel="noopener noreferrer">{name}</a> }
            } else {
                view! {
                    <span aria-hidden="true">" · "</span>
                    <a href={href} rel="noopener noreferrer">{name}</a>
                }
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="sister-apps-links" aria-label="Also from us">
            <span>"Also from us:"</span>
            " "
            {items}
        </nav>
    }
}
