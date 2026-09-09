//! Shared work-card markup.

use resuma::prelude::*;

use crate::work::Piece;

pub fn work_card(p: &Piece) -> View {
    view! {
        <li>
            <a class="work-link" href={p.href} rel="noopener noreferrer">
                <span class="work-media">
                    <img
                        src={p.preview}
                        alt=""
                        width="960"
                        height="540"
                        loading="lazy"
                        decoding="async"
                    />
                </span>
                <span class="work-body">
                    <span class="work-kind">{p.kind}</span>
                    <strong>{p.name}</strong>
                    <span class="work-blurb">{p.blurb}</span>
                </span>
            </a>
        </li>
    }
}
