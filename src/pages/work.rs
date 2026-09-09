use resuma::prelude::*;

use crate::cards::work_card;
use crate::site::canonical_url;
use crate::work::PIECES;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Work | Goldev");
    set_page_description(
        "Selected work: ACUPATAS, MOA Education, Moa Academy, Koolinart, YouTubeForge, and more.",
    );
    set_page_canonical(canonical_url("/work"));

    let items: Vec<View> = PIECES.iter().map(work_card).collect();

    view! {
        <main class="page">
            <p class="page-kicker reveal">"Portfolio"</p>
            <h1 class="reveal">"Work"</h1>
            <p class="page-lead reveal">
                "Client builds, learning platforms, Web3, and microsaas tools."
            </p>
            <ul class="work-grid">
                {items}
            </ul>
        </main>
    }
}
