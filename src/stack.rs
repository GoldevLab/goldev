//! Development stack shown on Goldev.

use resuma::prelude::*;

#[derive(Clone, Copy)]
pub struct Tech {
    pub name: &'static str,
    pub blurb: &'static str,
    pub group: &'static str,
    /// Official docs / reference.
    pub docs: &'static str,
}

pub const STACK: &[Tech] = &[
    Tech {
        name: "Rust",
        blurb: "Systems language for Resuma apps, APIs, and Fly binaries.",
        group: "Core",
        docs: "https://doc.rust-lang.org/book/",
    },
    Tech {
        name: "TypeScript",
        blurb: "Typed frontends for Qwik products — ACUPATAS, MOA, Koolinart.",
        group: "Core",
        docs: "https://www.typescriptlang.org/docs/",
    },
    Tech {
        name: "Resuma",
        blurb: "Resumable Rust SSR — FlowApp, islands, HtmlTheme, View Transitions.",
        group: "Framework",
        docs: "https://resuma-docs.fly.dev/",
    },
    Tech {
        name: "Qwik",
        blurb: "Resumable JS SSR for large client apps and PWAs.",
        group: "Framework",
        docs: "https://qwik.dev/docs/",
    },
    Tech {
        name: "Axum + Tokio",
        blurb: "Async HTTP and middleware next to Resuma routes.",
        group: "Backend",
        docs: "https://docs.rs/axum/latest/axum/",
    },
    Tech {
        name: "Node + Vite",
        blurb: "Tooling for Qwik City builds and client bundles.",
        group: "Backend",
        docs: "https://vite.dev/guide/",
    },
    Tech {
        name: "PostgreSQL",
        blurb: "Relational data for product apps that need durable state.",
        group: "Data",
        docs: "https://www.postgresql.org/docs/current/",
    },
    Tech {
        name: "SQLite",
        blurb: "Local and single-node tools — job bots, light services.",
        group: "Data",
        docs: "https://www.sqlite.org/docs.html",
    },
    Tech {
        name: "Web3 / EVM",
        blurb: "Tokenized assets, wallets, and on-chain product surfaces.",
        group: "Domain",
        docs: "https://ethereum.org/developers/docs/",
    },
    Tech {
        name: "PWA",
        blurb: "Installable apps — manifest, service worker, offline shell.",
        group: "Platform",
        docs: "https://web.dev/explore/progressive-web-apps",
    },
    Tech {
        name: "Fly.io",
        blurb: "Production deploys for Resuma and Qwik services.",
        group: "Platform",
        docs: "https://fly.io/docs/",
    },
    Tech {
        name: "Docker",
        blurb: "Reproducible images for Fly and CI pipelines.",
        group: "Platform",
        docs: "https://docs.docker.com/",
    },
];

pub fn stack_card(t: &Tech) -> View {
    view! {
        <li>
            <a
                class="stack-item"
                href={t.docs}
                rel="noopener noreferrer"
                target="_blank"
                aria-label={format!("{} documentation", t.name)}
            >
                <span class="stack-group">{t.group}</span>
                <strong class="stack-name">
                    {t.name}
                    <span class="stack-docs" aria-hidden="true">"Docs ↗"</span>
                </strong>
                <span class="stack-blurb">{t.blurb}</span>
            </a>
        </li>
    }
}
