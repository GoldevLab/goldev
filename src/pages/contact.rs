use resuma::prelude::*;

use crate::site::{canonical_url, EMAIL, GITHUB, LINKEDIN, NAME};

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Contact Golfredo Pérez Fernández — Rust & Web3 Developer | Goldev");
    set_page_description(
        "Hire Golfredo Pérez Fernández for Rust backend, Web3 and product web apps. Email, GitHub and LinkedIn.",
    );
    set_page_canonical(canonical_url("/contact"));

    view! {
        <main class="page">
            <p class="page-kicker reveal">"Contact"</p>
            <h1 class="reveal">"Say hello"</h1>
            <p class="page-lead reveal">
                "Projects, collaborations, or a quiet note — "
                {NAME}
                " replies by email."
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
                    <NavLink href="/work" class="btn btn-ghost">"View work"</NavLink>
                </div>
            </div>
            <p class="hint reveal" style="margin-top:1.5rem">
                "Also on the home page: "
                <NavLink href="/#faq">"FAQ"</NavLink>
                " and "
                <NavLink href="/#contact">"Contact"</NavLink>
                "."
            </p>
        </main>
    }
}
