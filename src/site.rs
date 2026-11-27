//! Public origin and shared portfolio copy.

pub fn public_origin() -> String {
    std::env::var("SITE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000".into())
        .trim_end_matches('/')
        .to_string()
}

pub fn canonical_url(path: &str) -> String {
    let origin = public_origin();
    if path.is_empty() || path == "/" {
        format!("{origin}/")
    } else {
        format!("{origin}{}", if path.starts_with('/') { path.to_string() } else { format!("/{path}") })
    }
}

pub const NAME: &str = "Golfredo Pérez Fernández";
pub const BRAND: &str = "Goldev";
pub const ORG: &str = "GoldevLab";
pub const EMAIL: &str = "golfredo.pf@gmail.com";
pub const GITHUB: &str = "https://github.com/GoldevLab";
pub const LINKEDIN: &str = "https://linkedin.com/in/golfredo-perez-fernandez";
pub const TAGLINE: &str = "Backend, Web3, and resumable product craft — silver glass tools that ship.";
/// Search title and snippet for the home page (role keywords that match the work shown).
pub const SEO_TITLE_HOME: &str = "Golfredo Pérez Fernández — Rust & Web3 Developer | Goldev";
pub const SEO_DESCRIPTION_HOME: &str = "Golfredo Pérez Fernández, Rust and Web3 developer from Venezuela. Backend APIs, on-chain product UIs and Resuma microsaas. Remote and async-first.";
pub const ROLE: &str = "Rust & Web3 developer";

/// Optional head markup from env: Search Console meta verification and GA4.
///
/// - `GSC_VERIFICATION`: the `content` value of Google's `google-site-verification` meta tag.
/// - `GA4_ID`: measurement ID (`G-XXXXXXXXXX`); gtag loads on the first real interaction
///   so bots and prerender hits do not inflate the numbers.
///
/// Unset or malformed values emit nothing.
pub fn head_extras() -> String {
    let mut out = String::new();
    if let Ok(v) = std::env::var("GSC_VERIFICATION") {
        let v = v.trim();
        if !v.is_empty()
            && v.len() < 120
            && v.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            out.push_str(&format!(
                r#"<meta name="google-site-verification" content="{v}" />
"#
            ));
        }
    }
    if let Ok(id) = std::env::var("GA4_ID") {
        let id = id.trim();
        if id.starts_with("G-")
            && id.len() < 20
            && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            out.push_str(&format!(
                r#"<link rel="dns-prefetch" href="https://www.googletagmanager.com" />
<script>
(function(){{
  var id={id:?};
  window.dataLayer=window.dataLayer||[];
  function gtag(){{dataLayer.push(arguments);}}
  window.gtag=gtag;
  var loaded=false;
  function load(){{
    if(loaded)return;
    loaded=true;
    var s=document.createElement('script');
    s.async=true;
    s.fetchPriority='low';
    s.src='https://www.googletagmanager.com/gtag/js?id='+id;
    s.onload=function(){{
      gtag('js',new Date());
      gtag('config',id,{{send_page_view:true}});
    }};
    document.head.appendChild(s);
  }}
  ['pointerdown','keydown','touchstart','scroll'].forEach(function(e){{
    window.addEventListener(e,load,{{once:true,passive:true}});
  }});
}})();
</script>
"#
            ));
        }
    }
    out
}
