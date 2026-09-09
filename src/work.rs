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
    &PIECES[8], // Resuma
];

/// One deep narrative case on the home page (challenge → solution → outcomes).
#[derive(Clone, Copy)]
pub struct CaseStudy {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub subject: &'static str,
    pub challenge: &'static str,
    pub solution: &'static str,
    pub outcomes: &'static [&'static str],
    pub tech: &'static str,
    pub href: &'static str,
    pub preview: &'static str,
    pub cta: &'static str,
}

pub const FEATURED_CASE: CaseStudy = CaseStudy {
    eyebrow: "Case study",
    title: "Captions → product workflow",
    subject: "YouTubeForge — forgeyt.com",
    challenge:
        "Creators and teams needed transcripts, chapters, and translation from public captions — without a heavy desktop suite or opaque pricing.",
    solution:
        "A focused Resuma product: pull public captions, forge SRT/audio recaps, translate, and ship chapter-ready output from a silver-glass UI that stays fast on the phone.",
    outcomes: &[
        "Live at forgeyt.com",
        "Transcript · SRT · chapters · translation",
        "Resuma SSR — ship without a SPA tax",
    ],
    tech: "Rust · Resuma · Fly.io",
    href: "https://forgeyt.com",
    preview: "/previews/youtubeforge.webp",
    cta: "Open YouTubeForge",
};
