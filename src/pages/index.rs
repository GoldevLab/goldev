use resuma::prelude::*;

use crate::cards::work_card;
use crate::faq::faq_cards;
use crate::offerings::offering_items;
use crate::process::process_items;
use crate::site::{canonical_url, EMAIL, GITHUB, LINKEDIN, NAME, TAGLINE};
use crate::stack::STACK;
use crate::work::{FEATURED_CASE, HERO_PIECES, PIECES};

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Goldev — Golfredo Pérez Fernández");
    set_page_description(TAGLINE);
    set_page_canonical(canonical_url("/"));

    // Skip the case-study product so YouTubeForge is not shown three times on home.
    let featured: Vec<View> = PIECES
        .iter()
        .filter(|p| p.href != FEATURED_CASE.href)
        .take(6)
        .map(work_card)
        .collect();

    let slides: Vec<View> = HERO_PIECES
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let eager = i == 0;
            let hidden = if i == 0 { "false" } else { "true" };
            view! {
                <article
                    class={if i == 0 {
                        "hero-slide is-active"
                    } else {
                        "hero-slide"
                    }}
                    data-carousel-slide=""
                    aria-hidden={hidden}
                >
                    <a
                        class="hero-slide-hit"
                        href={p.href}
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label={format!("Open {} — {}", p.name, p.href)}
                        tabindex={if i == 0 { "0" } else { "-1" }}
                        data-carousel-link=""
                    >
                        <span class="hero-slide-media">
                            <img
                                src={p.preview}
                                alt=""
                                width="960"
                                height="540"
                                decoding="async"
                                loading={if eager { "eager" } else { "lazy" }}
                                fetchpriority={if eager { "high" } else { "low" }}
                            />
                            <span class="hero-slide-open" aria-hidden="true">
                                "Open app ↗"
                            </span>
                        </span>
                    </a>
                    <div class="hero-slide-meta">
                        <span class="work-kind">{p.kind}</span>
                        <a
                            class="hero-slide-title"
                            href={p.href}
                            target="_blank"
                            rel="noopener noreferrer"
                            tabindex={if i == 0 { "0" } else { "-1" }}
                            data-carousel-link=""
                        >
                            {p.name}
                        </a>
                        <span class="hero-panel-blurb">{p.blurb}</span>
                    </div>
                </article>
            }
        })
        .collect();

    let dots: Vec<View> = HERO_PIECES
        .iter()
        .enumerate()
        .map(|(i, p)| {
            view! {
                <button
                    type="button"
                    class={if i == 0 {
                        "hero-carousel-dot is-active"
                    } else {
                        "hero-carousel-dot"
                    }}
                    data-carousel-dot={i.to_string()}
                    aria-label={format!("Show {}", p.name)}
                    aria-current={if i == 0 { "true" } else { "false" }}
                ></button>
            }
        })
        .collect();

    let offerings = offering_items();
    let process = process_items();
    let case_outcomes: Vec<View> = FEATURED_CASE
        .outcomes
        .iter()
        .map(|o| {
            view! {
                <li>{*o}</li>
            }
        })
        .collect();
    let stack: Vec<View> = STACK.iter().map(crate::stack::stack_card).collect();
    let faq = faq_cards();

    view! {
        <main id="main">
            <div class="hero-wrap">
                <div class="hero-particles" data-hero-particles="" aria-hidden="true"></div>
                <section class="hero">
                    <div class="hero-copy">
                        <p class="eyebrow">"Software engineer · GoldevLab"</p>
                        <h1 class="hero-brand">{NAME}</h1>
                        <p class="hero-lead">{TAGLINE}</p>
                        <div class="hero-cta">
                            <a class="btn btn-primary" href="#contact">"Let's talk"</a>
                            <a class="btn btn-ghost" href="#work">"View work"</a>
                        </div>
                        <p class="hero-trust">"Usually replies within 24 hours · email is enough to start"</p>
                    </div>
                    <div class="hero-panel" data-hero-carousel="">
                        <div class="hero-panel-chrome" aria-hidden="true">
                            <span></span><span></span><span></span>
                            <span class="hero-panel-chrome-label">"Live apps"</span>
                        </div>
                        <div class="hero-carousel">
                            <div
                                class="hero-carousel-track"
                                data-carousel-track=""
                                tabindex="0"
                                role="region"
                                aria-roledescription="carousel"
                                aria-label="Live apps"
                            >
                                {slides}
                            </div>
                            <div class="hero-carousel-ui">
                                <button
                                    type="button"
                                    class="hero-carousel-btn"
                                    data-carousel-prev=""
                                    aria-label="Previous app"
                                >
                                    <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
                                        <path
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M15 6l-6 6 6 6"
                                        />
                                    </svg>
                                </button>
                                <div class="hero-carousel-dots" role="tablist" aria-label="Apps">
                                    {dots}
                                </div>
                                <button
                                    type="button"
                                    class="hero-carousel-btn"
                                    data-carousel-next=""
                                    aria-label="Next app"
                                >
                                    <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
                                        <path
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M9 6l6 6-6 6"
                                        />
                                    </svg>
                                </button>
                            </div>
                            <div class="hero-carousel-progress" aria-hidden="true">
                                <span data-carousel-progress=""></span>
                            </div>
                        </div>
                    </div>
                </section>
            </div>

            <section class="section offerings" id="offerings" aria-labelledby="offerings-title">
                <h2 id="offerings-title" class="reveal">"What I take on"</h2>
                <p class="hint reveal">
                    "Four lanes — pick the one that matches the problem, not a pack of buzzwords."
                </p>
                <ul class="offer-grid">
                    {offerings}
                </ul>
            </section>

            <section class="section case-section" id="case" aria-labelledby="case-title">
                <p class="case-eyebrow reveal">{FEATURED_CASE.eyebrow}</p>
                <h2 id="case-title" class="reveal">{FEATURED_CASE.title}</h2>
                <p class="hint reveal">{FEATURED_CASE.subject}</p>
                <div class="case-layout">
                    <a
                        class="case-media reveal"
                        href={FEATURED_CASE.href}
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label={FEATURED_CASE.cta}
                    >
                        <img
                            src={FEATURED_CASE.preview}
                            alt=""
                            width="960"
                            height="540"
                            loading="lazy"
                            decoding="async"
                        />
                    </a>
                    <div class="case-body reveal">
                        <div class="case-block">
                            <h3>"Challenge"</h3>
                            <p>{FEATURED_CASE.challenge}</p>
                        </div>
                        <div class="case-block">
                            <h3>"Solution"</h3>
                            <p>{FEATURED_CASE.solution}</p>
                        </div>
                        <ul class="case-outcomes">
                            {case_outcomes}
                        </ul>
                        <p class="case-tech">{FEATURED_CASE.tech}</p>
                        <p class="case-actions">
                            <a
                                class="btn btn-primary"
                                href={FEATURED_CASE.href}
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                {FEATURED_CASE.cta}
                            </a>
                            <a class="btn btn-ghost" href="#work">"More work"</a>
                        </p>
                    </div>
                </div>
            </section>

            <section class="section" id="work" aria-labelledby="selected-work">
                <h2 id="selected-work" class="reveal">"Selected work"</h2>
                <p class="hint reveal">
                    "Products and client builds — open any card for the live site."
                </p>
                <ul class="work-grid">
                    {featured}
                </ul>
                <p class="reveal" style="margin-top:1.25rem">
                    <NavLink href="/work" class="btn btn-ghost">"All projects"</NavLink>
                </p>
            </section>

            <section class="section process-section" id="process" aria-labelledby="process-title">
                <h2 id="process-title" class="reveal">"How we work"</h2>
                <p class="hint reveal">
                    "A simple path from first note to something live — then optional support."
                </p>
                <ol class="process-list">
                    {process}
                </ol>
            </section>

            <section class="section" aria-labelledby="stack-title">
                <h2 id="stack-title" class="reveal">"Stack"</h2>
                <p class="hint reveal">
                    "Rust and TypeScript in production — Resuma for microsaas, Qwik for large PWAs. Open Docs for the official reference."
                </p>
                <ul class="stack-grid">
                    {stack}
                </ul>
            </section>

            <section class="section faq" id="faq" aria-labelledby="faq-title">
                <h2 id="faq-title" class="reveal">"FAQ"</h2>
                <p class="hint reveal">
                    "Scope, remote work, timelines, and how to start."
                </p>
                <div class="faq-list">
                    {faq}
                </div>
            </section>

            <section class="section contact-section" id="contact" aria-labelledby="contact-title">
                <h2 id="contact-title" class="reveal">"Contact"</h2>
                <p class="hint reveal">
                    "Projects, collaborations, or a quiet note — email is the fastest path. Usually replies within 24 hours."
                </p>
                <div class="contact-panel glass-panel reveal">
                    <ul class="contact-list">
                        <li>
                            <span class="contact-label">"Email"</span>
                            <a href={format!("mailto:{EMAIL}")}>{EMAIL}</a>
                        </li>
                        <li>
                            <span class="contact-label">"GitHub"</span>
                            <a href={GITHUB} rel="noopener noreferrer">"github.com/GoldevLab"</a>
                        </li>
                        <li>
                            <span class="contact-label">"LinkedIn"</span>
                            <a href={LINKEDIN} rel="noopener noreferrer">"Golfredo Pérez Fernández"</a>
                        </li>
                    </ul>
                    <div class="contact-actions">
                        <a class="btn btn-primary" href={format!("mailto:{EMAIL}")}>"Email me"</a>
                        <a class="btn btn-ghost" href="#work">"View work"</a>
                    </div>
                </div>
            </section>
        </main>
    }
}
