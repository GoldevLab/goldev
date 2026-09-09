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
        q: "What kinds of projects do you take?",
        a: "Backend and APIs, Web3 product surfaces, and full web apps — especially Resuma microsaas and Qwik PWAs. If the problem is clear and the scope can ship in slices, we can talk.",
    },
    FaqItem {
        q: "Do you work remotely?",
        a: "Yes. Async-first with calls when they unblock decisions. Time zones are flexible as long as feedback loops stay short.",
    },
    FaqItem {
        q: "Fixed packs or custom scope?",
        a: "Custom scope. You get a written plan with milestones — not a generic pack. Small tools can be fixed-price; larger products usually run in phases.",
    },
    FaqItem {
        q: "How long does a project take?",
        a: "A focused tool can land in weeks. A marketplace or multi-surface product is phased — first useful release first, then deepen. Timelines live in the proposal, not as a slogan.",
    },
    FaqItem {
        q: "What support do you offer after launch?",
        a: "Handover with deploy notes, then optional support for fixes and the next slice. Email stays open for questions that keep the product healthy.",
    },
    FaqItem {
        q: "Rust or TypeScript — which do you use?",
        a: "Both, on purpose. Resuma + Rust for microsaas that ship fast. Qwik + TypeScript for larger product PWAs (ACUPATAS, MOA, Koolinart).",
    },
    FaqItem {
        q: "How do I get in touch?",
        a: "Email golfredo.pf@gmail.com with context, timeline, and links if you have them. Usually replies within 24 hours. GitHub and LinkedIn are linked in Contact.",
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
