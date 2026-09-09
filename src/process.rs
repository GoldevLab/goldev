//! How engagement typically runs.

use resuma::prelude::*;

#[derive(Clone, Copy)]
pub struct ProcessStep {
    pub n: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
}

pub const PROCESS: &[ProcessStep] = &[
    ProcessStep {
        n: "01",
        title: "Discovery",
        blurb: "A short call or thread. Goals, constraints, and what success looks like.",
    },
    ProcessStep {
        n: "02",
        title: "Scope",
        blurb: "A clear proposal — milestones, stack, and what ships first. Plain language.",
    },
    ProcessStep {
        n: "03",
        title: "Build",
        blurb: "Iterate in the open. Previews early, decisions documented, no black boxes.",
    },
    ProcessStep {
        n: "04",
        title: "Ship",
        blurb: "Deploy, harden, and hand over something you can run and extend.",
    },
    ProcessStep {
        n: "05",
        title: "Support",
        blurb: "Optional follow-through after launch — fixes, polish, and the next slice.",
    },
];

pub fn process_items() -> Vec<View> {
    PROCESS
        .iter()
        .map(|s| {
            view! {
                <li class="process-item reveal">
                    <span class="process-n" aria-hidden="true">{s.n}</span>
                    <h3 class="process-title">{s.title}</h3>
                    <p class="process-blurb">{s.blurb}</p>
                </li>
            }
        })
        .collect()
}
