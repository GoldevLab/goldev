//! Selected work — products and client builds.

#[derive(Clone, Copy)]
pub struct Piece {
    pub name: &'static str,
    pub href: &'static str,
    pub blurb: &'static str,
    pub kind: &'static str,
    /// WebP under `/previews/` (960×540).
    pub preview: &'static str,
}

pub const PIECES: &[Piece] = &[
    Piece {
        name: "YouTubeForge",
        href: "https://forgeyt.com",
        blurb: "Transcripts, audio, SRT, translation, and chapter recaps from public captions.",
        kind: "Product",
        preview: "/previews/youtubeforge.webp",
    },
    Piece {
        name: "UnderKb",
        href: "https://under200kb.com",
        blurb: "Compress images to a real kilobyte budget — JPG, WebP, PNG.",
        kind: "Product",
        preview: "/previews/underkb.webp",
    },
    Piece {
        name: "ACUPATAS",
        href: "https://acupatas.com/",
        blurb: "Pet care marketplace in Venezuela — verified caregivers, biometrics, audited payments, and secure chat.",
        kind: "Product",
        preview: "/previews/acupatas.webp",
    },
    Piece {
        name: "Koolinart",
        href: "https://koolinart.fly.dev/",
        blurb: "KNRT Property — tokenize, trade, and manage real-world assets on the Koolinart Chain.",
        kind: "Web3",
        preview: "/previews/koolinart.webp",
    },
    Piece {
        name: "PDFForge",
        href: "https://pdfforge.fly.dev",
        blurb: "Merge, split, compress PDFs. JPG ↔ PDF and extract text.",
        kind: "Product",
        preview: "/previews/pdfforge.webp",
    },
    Piece {
        name: "Moa Academy",
        href: "https://alice-moa.fly.dev/",
        blurb: "Learning platform for language courses — community, instructors, and practice toward fluency.",
        kind: "Product",
        preview: "/previews/moa-academy.webp",
    },
    Piece {
        name: "MOA Education",
        href: "https://moaeducation.com/",
        blurb: "Language company site — programs for people, schools, and businesses, in person and live online.",
        kind: "Client",
        preview: "/previews/moa-education.webp",
    },
    Piece {
        name: "PlacaQR",
        href: "https://placaqr.fly.dev",
        blurb: "3D-printable QR — stand, tile, keychain, or plaque.",
        kind: "Product",
        preview: "/previews/placaqr.webp",
    },
    Piece {
        name: "Billloom",
        href: "https://billloom.fly.dev",
        blurb: "Invoice, quote, and receipt PDFs. No account, no watermark.",
        kind: "Product",
        preview: "/previews/billloom.webp",
    },
    Piece {
        name: "Linkprobe",
        href: "https://linkprobe.fly.dev",
        blurb: "Paste a URL. See which links work and which 404.",
        kind: "Product",
        preview: "/previews/linkprobe.webp",
    },
    Piece {
        name: "Svgsport",
        href: "https://svgsport.fly.dev",
        blurb: "Paste a page. Download every SVG as a zip.",
        kind: "Product",
        preview: "/previews/svgsport.webp",
    },
    Piece {
        name: "Resuma",
        href: "https://resuma-docs.fly.dev",
        blurb: "Resumable SSR in Rust — islands, Flow routing, zero hydration by default.",
        kind: "Framework",
        preview: "/previews/resuma.webp",
    },
];

/// Flagship apps for the home hero carousel (keeps the fold sharp).
pub const HERO_PIECES: &[&Piece] = &[
    &PIECES[0], // YouTubeForge
    &PIECES[1], // UnderKb
    &PIECES[2], // ACUPATAS
    &PIECES[3], // Koolinart
    &PIECES[4], // PDFForge
    &PIECES[5], // Moa Academy
    &PIECES[7], // PlacaQR
    &PIECES[11], // Resuma
];
