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
