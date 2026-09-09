//! Frequently asked questions for the portfolio.

use resuma::prelude::*;
use serde_json::{json, Value};

#[derive(Clone, Copy)]
pub struct FaqItem {
    pub q: &'static str,
    pub a: &'static str,
}

pub const FAQ: &[FaqItem] = &[
    FaqItem {
        q: "What is Goldev?",
        a: "Goldev is the personal brand and portfolio of Golfredo Pérez Fernández / GoldevLab — selected products, client builds, and the stack behind them.",
    },
    FaqItem {
        q: "Rust or TypeScript — which do you use?",
        a: "Both, on purpose. Resuma + Rust for microsaas tools that ship fast (YouTubeForge, UnderKb, PDFForge, and sisters). Qwik + TypeScript for larger product PWAs (ACUPATAS, MOA, Koolinart).",
    },
    FaqItem {
        q: "Are you available for hire or collaboration?",
        a: "Yes — backend, Web3, and product craft. Email is the best channel; include context, timeline, and links if you have them.",
    },
    FaqItem {
        q: "What is Resuma?",
        a: "A resumable Rust SSR framework (FlowApp, islands, HtmlTheme, View Transitions) used to build the GoldevLab microsaas suite. Docs live at resuma-docs.fly.dev.",
    },
    FaqItem {
        q: "Where can I try the tools?",
        a: "Use the Live apps carousel on the home page, or Selected work / Work — each card opens the production URL (forgeyt.com, under200kb.com, and Fly-hosted tools).",
    },
    FaqItem {
        q: "How do I get in touch?",
        a: "Email golfredo.pf@gmail.com, or reach GoldevLab on GitHub and LinkedIn. The Contact section below has direct links.",
    },
];

pub fn faq_cards() -> Vec<View> {
    FAQ.iter()
        .map(|item| {
            view! {
                <details class="faq-item reveal">
                    <summary>{item.q}</summary>
                    <p>{item.a}</p>
                </details>
            }
        })
        .collect()
}

pub fn faq_json_ld() -> Value {
    json!({
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": FAQ.iter().map(|item| json!({
            "@type": "Question",
            "name": item.q,
            "acceptedAnswer": {
                "@type": "Answer",
                "text": item.a
            }
        })).collect::<Vec<_>>()
    })
}
