//! Client-facing offerings (not the tech stack).

use resuma::prelude::*;

#[derive(Clone, Copy)]
pub struct Offering {
    pub title: &'static str,
    pub blurb: &'static str,
}

pub const OFFERINGS: &[Offering] = &[
    Offering {
        title: "Backend & APIs",
        blurb: "Solid services, auth, data, and integrations that stay quiet in production.",
    },
    Offering {
        title: "Web3 & on-chain",
        blurb: "Contracts, token flows, and product UI that make chain work usable.",
    },
    Offering {
        title: "Product web apps",
        blurb: "Resuma and Qwik PWAs — fast first paint, real features, shippable scope.",
    },
    Offering {
        title: "Tooling & microsaas",
        blurb: "Focused tools with a clear job: compress, forge, extract, ship.",
    },
];

pub fn offering_items() -> Vec<View> {
    OFFERINGS
        .iter()
        .map(|o| {
            view! {
                <li class="offer-item reveal">
                    <h3 class="offer-title">{o.title}</h3>
                    <p class="offer-blurb">{o.blurb}</p>
                </li>
            }
        })
        .collect()
}
