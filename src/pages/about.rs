use resuma::prelude::*;

use crate::site::{canonical_url, NAME, ORG};
use crate::stack::STACK;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("About | Goldev");
    set_page_description(
        "Golfredo Pérez Fernández — Rust, TypeScript, Resuma, and Qwik. Backend, Web3, and product craft.",
    );
    set_page_canonical(canonical_url("/about"));

    let stack: Vec<View> = STACK.iter().map(crate::stack::stack_card).collect();

    view! {
        <main class="page">
            <p class="page-kicker reveal">"About"</p>
            <h1 class="reveal">{NAME}</h1>
            <p class="page-lead reveal">
                "Software engineer from Venezuela. I ship under "
                {ORG}
                " with two main lanes: "
                <strong>"Rust + Resuma"</strong>
                " for microsaas, and "
                <strong>"TypeScript + Qwik"</strong>
                " for larger product PWAs — plus Web3 when the job is on-chain."
            </p>
            <div class="glass-panel prose reveal">
                <p>
                    "ACUPATAS, MOA Education, Moa Academy, and Koolinart run on Qwik/TypeScript. YouTubeForge, UnderKb, PDFForge, and the sister tools run on Resuma/Rust. Same craft, different runtime."
                </p>
                <p>
                    "Goldev is the front door: selected work, stack, and how to reach me."
                </p>
            </div>
            <h2 class="reveal" style="margin-top:2.5rem">"Technologies"</h2>
            <ul class="stack-grid" style="margin-top:1rem">
                {stack}
            </ul>
        </main>
    }
}
