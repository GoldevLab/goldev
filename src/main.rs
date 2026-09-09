//! Goldev — portfolio for Golfredo Pérez Fernández / GoldevLab.

mod cards;
mod faq;
mod offerings;
mod pages;
mod process;
mod security;
mod site;
mod stack;
mod work;

use pages::PagesRegistry;
use resuma::prelude::*;
use resuma::SeoKit;
use serde_json::json;

fn view_transition_name(path: &str) -> String {
    let slug = path.trim_matches('/');
    if slug.is_empty() {
        "home".into()
    } else {
        slug.replace('/', "-")
    }
}

fn chrome(body: View) -> View {
    let vt = view_transition_name(
        &current_request()
            .map(|r| r.path)
            .unwrap_or_else(|| "/".into()),
    );
    view! {
        <div class="app">
            <a class="skip-link" href="#main">"Skip to content"</a>
            <div class="liquid-orbs" aria-hidden="true">
                <div class="liquid-blob liquid-blob-a"></div>
                <div class="liquid-blob liquid-blob-b"></div>
                <div class="liquid-blob liquid-blob-c"></div>
                <div class="liquid-spark"></div>
            </div>
            <header class="site-header">
                <div class="header-inner">
                    <NavLink href="/" class="brand" activeClass="is-active" exact=true>
                        <img class="brand-logo" src="/icon.svg" width="36" height="36" alt="" />
                        <span class="brand-name">"Goldev"</span>
                    </NavLink>
                    <button
                        type="button"
                        class="nav-toggle"
                        data-nav-toggle=""
                        aria-expanded="false"
                        aria-controls="site-menu"
                        aria-label="Open menu"
                    >
                        <span class="nav-toggle-bars" aria-hidden="true">
                            <span></span><span></span><span></span>
                        </span>
                    </button>
                    <div class="nav-backdrop" data-nav-backdrop="" hidden=""></div>
                    <nav class="site-nav" id="site-menu" data-site-nav="" aria-label="Primary">
                        <p class="nav-drawer-kicker" aria-hidden="true">"Menu"</p>
                        <NavLink href="/work" activeClass="is-active">"Work"</NavLink>
                        <NavLink href="/about" activeClass="is-active">"About"</NavLink>
                        <a href="/#faq">"FAQ"</a>
                        <NavLink href="/contact" activeClass="is-active">"Contact"</NavLink>
                        <div class="nav-drawer-meta">
                            <a href={crate::site::GITHUB} rel="noopener noreferrer">"GitHub"</a>
                            <a href={format!("mailto:{}", crate::site::EMAIL)}>"Email"</a>
                        </div>
                    </nav>
                    <span class="nav-progress" aria-hidden="true"></span>
                </div>
            </header>
            {with_view_transition(vt, vec![Child::View(body)])}
            <footer class="site-footer">
                <div class="footer-inner">
                    <p>
                        <strong>"Goldev"</strong>
                        " — "
                        {crate::site::NAME}
                        " · "
                        {crate::site::ORG}
                    </p>
                    <div class="footer-links">
                        <NavLink href="/work">"Work"</NavLink>
                        <a href="/#faq">"FAQ"</a>
                        <NavLink href="/contact">"Contact"</NavLink>
                        <a href={crate::site::GITHUB} rel="noopener noreferrer">"GitHub"</a>
                        <a href={crate::site::LINKEDIN} rel="noopener noreferrer">"LinkedIn"</a>
                        <a href={format!("mailto:{}", crate::site::EMAIL)}>{crate::site::EMAIL}</a>
                    </div>
                </div>
            </footer>
        </div>
    }
}

#[layout("/")]
fn RootLayout() -> View {
    visible_task!(
        r##"
        async (_state, __resuma) => {
            const onNav = () => {
                document.documentElement.classList.remove("is-navigating");
                const heading = document.querySelector("[data-r-vt] h1");
                if (!heading) return;
                heading.setAttribute("tabindex", "-1");
                heading.focus({ preventScroll: true });
            };
            const onClick = (e) => {
                const el = e.target instanceof Element ? e.target.closest("a[data-r-nav]") : null;
                if (el?.getAttribute("href")) {
                    document.documentElement.classList.add("is-navigating");
                }
            };
            document.addEventListener("resuma:navigate", onNav);
            document.addEventListener("click", onClick, true);
            if (!window.__goldevVtGuard) {
                window.__goldevVtGuard = true;
                window.addEventListener("unhandledrejection", (e) => {
                    const err = e.reason;
                    const name = err && err.name;
                    const msg = (err && (err.message || String(err))) || "";
                    if (name === "AbortError" && /Transition was skipped|ViewTransition/i.test(msg)) {
                        e.preventDefault();
                    }
                });
            }
            return () => {
                document.removeEventListener("resuma:navigate", onNav);
                document.removeEventListener("click", onClick, true);
            };
        }
        "##
    );
    chrome(view! { <Slot /> })
}

fn not_found_page() -> View {
    chrome(view! {
        <main class="page">
            <p class="page-kicker reveal">"404"</p>
            <h1 class="reveal">"Page not found"</h1>
            <p class="page-lead reveal">"That path is not part of Goldev."</p>
            <p class="reveal">
                <NavLink href="/" class="btn btn-primary">"Back home"</NavLink>
            </p>
        </main>
    })
}

fn seo_kit() -> SeoKit {
    let origin = crate::site::public_origin();
    let mut kit = SeoKit::new(crate::site::BRAND, &origin)
        .with_locale("en_US")
        .with_keywords(
            "Golfredo Pérez Fernández, Goldev, GoldevLab, Resuma, portfolio, backend, Web3, YouTubeForge, UnderKb",
        )
        .with_llms_summary(
            "Goldev is the portfolio of Golfredo Pérez Fernández (GoldevLab): client builds, \
             microsaas tools, Web3, and Resuma SSR products.",
        )
        .with_default_json_ld()
        .push_json_ld(json!({
            "@context": "https://schema.org",
            "@type": "Person",
            "name": crate::site::NAME,
            "url": origin,
            "email": crate::site::EMAIL,
            "sameAs": [crate::site::GITHUB, crate::site::LINKEDIN],
            "jobTitle": "Software engineer",
            "worksFor": {
                "@type": "Organization",
                "name": crate::site::ORG,
                "url": crate::site::GITHUB
            }
        }))
        .push_json_ld(json!({
            "@context": "https://schema.org",
            "@type": "WebSite",
            "name": crate::site::BRAND,
            "url": origin,
            "description": crate::site::TAGLINE
        }))
        .push_json_ld(crate::faq::faq_json_ld());
    kit.theme_color = Some("#0a0c10".into());
    kit.author = crate::site::NAME.into();
    kit.llms_sections = vec![
        (
            "Work".into(),
            format!("{origin}/work — selected products and client builds."),
        ),
        (
            "FAQ".into(),
            format!("{origin}/#faq — common questions about Goldev and the stack."),
        ),
        (
            "Contact".into(),
            format!("{origin}/contact — email, GitHub, LinkedIn."),
        ),
    ];
    kit
}

const FONTS_CSS: &str = "https://fonts.googleapis.com/css2?family=Figtree:wght@400;500;600;700&family=Syne:wght@600;700;800&display=swap";

fn head_html() -> String {
    // Non-blocking Google Fonts (print→all) + low-priority app JS — same TBT
    // posture as forgeyt (eager LCP CSS, third-party / chrome JS deferred or low).
    format!(
        r##"<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link rel="preload" as="style" href="{fonts}" />
<link rel="stylesheet" href="{fonts}" media="print" onload="this.media='all'" />
<noscript><link rel="stylesheet" href="{fonts}" /></noscript>
<link rel="preload" href="/themes.css" as="style" />
<link rel="preload" href="/css/goldev.css?v=17" as="style" />
<link rel="icon" href="/icon.svg" type="image/svg+xml" />
<link rel="icon" href="/icons/favicon-32.png" type="image/png" sizes="32x32" />
<link rel="apple-touch-icon" href="/icons/apple-touch-icon.png" sizes="180x180" />
<meta name="theme-color" content="#0a0c10" />
<script type="module" src="/js/goldev.js?v=7" fetchpriority="low"></script>
"##,
        fonts = FONTS_CSS
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    security::install();

    let public = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public");
    const ICON: &[u8] = include_bytes!("icon.svg");

    let contact = std::env::var("CONTACT_EMAIL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| s.contains('@') && !s.contains(' '));

    FlowApp::new()
        .with_title("Goldev — Golfredo Pérez Fernández")
        .with_description(crate::site::TAGLINE)
        .with_site_url(crate::site::public_origin())
        .with_og_image("/og.png")
        .with_head(head_html())
        .with_seo_kit(seo_kit())
        .with_html_theme(
            HtmlTheme::new(["argent"])
                .dark(["argent"])
                .cookie("goldev_theme")
                .storage_key("goldev-theme"),
        )
        .with_stylesheet("/css/goldev.css?v=17")
        // public/*.svg → octet-stream + nosniff blanks <img>; serve via static_asset.
        .static_asset("/icon.svg", ICON, "image/svg+xml")
        .with_security_txt(crate::site::public_origin(), contact.as_deref())
        .with_public_dir(public)
        .with_pwa(FlowPwaConfig {
            name: "Goldev".into(),
            short_name: "Goldev".into(),
            description: crate::site::TAGLINE.into(),
            theme_color: "#0a0c10".into(),
            background_color: "#0a0c10".into(),
            start_url: "/".into(),
            scope: "/".into(),
            cache_version: "goldev-18".into(),
            display: "standalone".into(),
            orientation: "any".into(),
            lang: "en".into(),
            icon_char: Some("G".into()),
            precache_paths: vec![
                "/themes.css".into(),
                "/css/goldev.css?v=17".into(),
                "/js/goldev.js?v=7".into(),
                "/icon.svg".into(),
                "/icons/icon-192.png".into(),
                "/icons/icon-512.png".into(),
                "/icons/apple-touch-icon.png".into(),
            ],
            shortcuts: vec![
                PwaShortcut {
                    name: "Work".into(),
                    short_name: "Work".into(),
                    url: "/work".into(),
                },
                PwaShortcut {
                    name: "Contact".into(),
                    short_name: "Contact".into(),
                    url: "/contact".into(),
                },
            ],
            offline_title: "You're offline".into(),
            offline_message: "Goldev needs a connection. Reconnect and open again.".into(),
            manifest_icons: Vec::new(),
        })
        .not_found(|| not_found_page())
        .auto_pages(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"),
            PagesRegistry,
        )
        .serve(FlowServeOptions {
            security: SecurityConfig::from_env(),
            ..FlowServeOptions::default()
        })
        .await
}
