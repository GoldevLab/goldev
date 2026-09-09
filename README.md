# Goldev

Portfolio for **Golfredo Pérez Fernández** / **GoldevLab** — Resuma Flow app with the `argent` silver glass theme. Domain: **https://golfredo.dev**

## Dev

```bash
cd atelier
RESUMA_SKIP_UPDATE=1 resuma dev --addr 127.0.0.1:3000
```

## Stack

- Resuma `1.3.1` (GoldevLab git)
- Brand **Goldev** · `HtmlTheme::argent` · PWA (`manifest` + SW `goldev-3`)
- Pages: `/` `/work` `/about` `/contact`
- Home shows **Stack**: Rust, TypeScript, Resuma, Qwik, Axum, Fly, …
## Deploy

Canonical domain: **https://golfredo.dev**

```bash
fly launch   # or reuse fly.toml
fly secrets set SITE_URL=https://golfredo.dev
fly deploy
```

Point DNS at the Fly app (A/AAAA or CNAME as Fly shows), then add the cert for `golfredo.dev`.
