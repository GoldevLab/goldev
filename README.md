# Goldev

Portfolio for **Golfredo Pérez Fernández** / **GoldevLab** — Resuma Flow app with the `argent` silver glass theme. Domain: **https://golfredo.dev**

## Dev

```bash
cd atelier
RESUMA_SKIP_UPDATE=1 resuma dev --addr 127.0.0.1:3000
```

## Stack

- Resuma (GoldevLab git pin in `Cargo.toml` — currently dogfood `5fd63d1+`: nonce-free JSON-LD, `with_security_txt`)
- Brand **Goldev** · `HtmlTheme::argent` · PWA
- Pages: `/` `/work` `/about` `/contact`
- Fly edge: `pristine`, security headers, `min_machines_running = 1`
- Footer links live sister tools: forgeyt, under200kb, pdfforge, placaqr

## Deploy

Canonical domain: **https://golfredo.dev**

Prefer GitHub Actions / `fly deploy`. Secrets:

```bash
fly secrets set SITE_URL=https://golfredo.dev
# optional
# fly secrets set CONTACT_EMAIL=golfredo.pf@gmail.com
```

Point DNS at the Fly app (A/AAAA or CNAME as Fly shows), then add the cert for `golfredo.dev`.
