(() => {
  let stopHero = null;

  const mountHeroParticles = () => {
    if (typeof stopHero === "function") {
      stopHero();
      stopHero = null;
    }
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    const root = document.querySelector("[data-hero-particles]");
    if (!root) return;

    const canvas = document.createElement("canvas");
    canvas.className = "hero-particles-canvas";
    canvas.setAttribute("aria-hidden", "true");
    root.replaceChildren(canvas);
    const ctx = canvas.getContext("2d", { alpha: true });
    if (!ctx) return;

    const COUNT = window.matchMedia("(max-width: 640px)").matches ? 32 : 56;
    const items = Array.from({ length: COUNT }, () => {
      const roll = Math.random();
      const kind = roll < 0.55 ? "foil" : roll < 0.82 ? "spark" : "ring";
      return {
        kind,
        x: Math.random() * 1.2 - 0.1,
        y: 0.1 + Math.random() * 0.78,
        z: 0.28 + Math.random() * 0.72,
        len: kind === "foil" ? 0.05 + Math.random() * 0.14 : 0,
        vx: -(0.01 + Math.random() * 0.024),
        vy: (Math.random() - 0.5) * 0.006,
      };
    });

    let w = 0;
    let h = 0;
    let pointerX = 0;
    let raf = 0;
    let last = 0;
    let accent = "#c9b896";
    let ink = "#d4dae6";

    const readColors = () => {
      const cs = getComputedStyle(document.documentElement);
      accent = (cs.getPropertyValue("--accent") || "#c9b896").trim() || "#c9b896";
      ink = (cs.getPropertyValue("--primary") || "#d4dae6").trim() || "#d4dae6";
    };

    const resize = () => {
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      w = root.clientWidth;
      h = root.clientHeight;
      if (!w || !h) return;
      canvas.width = Math.floor(w * dpr);
      canvas.height = Math.floor(h * dpr);
      canvas.style.width = w + "px";
      canvas.style.height = h + "px";
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    };

    const onMove = (e) => {
      const r = root.getBoundingClientRect();
      if (!r.width) return;
      pointerX = (e.clientX - r.left) / r.width - 0.5;
    };

    const roundChip = (x, y, bw, bh) => {
      const r = Math.min(bh / 2, 4);
      if (ctx.roundRect) {
        ctx.beginPath();
        ctx.roundRect(x, y, bw, bh, r);
        ctx.fill();
      } else {
        ctx.fillRect(x, y, bw, bh);
      }
    };

    const tick = (t) => {
      if (!root.isConnected) {
        if (typeof stopHero === "function") stopHero();
        return;
      }
      if (document.hidden) {
        raf = 0;
        last = 0;
        return;
      }
      const dt = Math.min(0.032, last ? (t - last) / 1000 : 0.016);
      last = t;
      ctx.clearRect(0, 0, w, h);

      ctx.globalAlpha = 0.16;
      ctx.fillStyle = accent;
      ctx.fillRect((0.55 + pointerX * 0.45) * w, h * 0.1, 1.4, h * 0.78);

      for (const d of items) {
        d.x += (d.vx + pointerX * 0.035 * d.z) * dt * 14;
        d.y += d.vy * dt * 10;
        if (d.x < -0.22) {
          d.x = 1.12;
          d.y = 0.1 + Math.random() * 0.78;
        } else if (d.x > 1.18) {
          d.x = -0.1;
        }
        if (d.y < 0.05 || d.y > 0.95) d.vy *= -1;

        const x = d.x * w;
        const y = d.y * h;
        ctx.globalAlpha = 0.18 + d.z * 0.45;

        if (d.kind === "foil") {
          ctx.fillStyle = ink;
          roundChip(x, y, d.len * w, 4 + d.z * 3.5);
        } else if (d.kind === "spark") {
          ctx.fillStyle = accent;
          ctx.fillRect(x, y - 4 - d.z * 5, 1.15, 8 + d.z * 9);
        } else {
          const s = 3.5 + d.z * 3.5;
          ctx.strokeStyle = accent;
          ctx.lineWidth = 1.2;
          ctx.beginPath();
          ctx.arc(x, y, s, 0, Math.PI * 2);
          ctx.stroke();
        }
      }
      raf = requestAnimationFrame(tick);
    };

    const onVis = () => {
      if (!document.hidden && !raf && root.isConnected) raf = requestAnimationFrame(tick);
    };

    const themeWatch = new MutationObserver(readColors);
    themeWatch.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-theme"],
    });

    readColors();
    resize();
    window.addEventListener("resize", resize, { passive: true });
    window.addEventListener("pointermove", onMove, { passive: true });
    document.addEventListener("visibilitychange", onVis);
    raf = requestAnimationFrame(tick);

    stopHero = () => {
      cancelAnimationFrame(raf);
      raf = 0;
      last = 0;
      themeWatch.disconnect();
      window.removeEventListener("resize", resize);
      window.removeEventListener("pointermove", onMove);
      document.removeEventListener("visibilitychange", onVis);
      canvas.remove();
      stopHero = null;
    };
  };

  let stopCarousel = null;
  const INTERVAL = 4500;

  const mountHeroCarousel = () => {
    if (typeof stopCarousel === "function") {
      stopCarousel();
      stopCarousel = null;
    }
    const root = document.querySelector("[data-hero-carousel]");
    if (!root) return;
    const track = root.querySelector("[data-carousel-track]");
    const slides = [...root.querySelectorAll("[data-carousel-slide]")];
    const dots = [...root.querySelectorAll("[data-carousel-dot]")];
    const prev = root.querySelector("[data-carousel-prev]");
    const next = root.querySelector("[data-carousel-next]");
    const progress = root.querySelector("[data-carousel-progress]");
    if (!track || slides.length < 2) return;

    root.style.setProperty("--carousel-ms", INTERVAL + "ms");
    let index = 0;
    let timer = 0;
    let paused = false;
    const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    const nearestIndex = () => {
      const left = track.scrollLeft;
      let best = 0;
      let dist = Infinity;
      slides.forEach((slide, i) => {
        const d = Math.abs(slide.offsetLeft - left);
        if (d < dist) {
          dist = d;
          best = i;
        }
      });
      return best;
    };

    const restartProgress = () => {
      if (!progress || reduce || paused) {
        if (progress) progress.classList.remove("is-running");
        return;
      }
      progress.classList.remove("is-running");
      void progress.offsetWidth;
      progress.classList.add("is-running");
    };

    const setActive = (i, { scroll = true, smooth = true } = {}) => {
      index = ((i % slides.length) + slides.length) % slides.length;
      slides.forEach((slide, n) => {
        const on = n === index;
        slide.classList.toggle("is-active", on);
        slide.setAttribute("aria-hidden", on ? "false" : "true");
        slide.querySelectorAll("[data-carousel-link]").forEach((link) => {
          if (on) link.removeAttribute("tabindex");
          else link.setAttribute("tabindex", "-1");
        });
      });
      dots.forEach((dot, n) => {
        const on = n === index;
        dot.classList.toggle("is-active", on);
        dot.setAttribute("aria-current", on ? "true" : "false");
      });
      if (scroll) {
        const behavior = smooth && !reduce ? "smooth" : "instant";
        slides[index].scrollIntoView({
          behavior: behavior === "instant" ? "auto" : behavior,
          inline: "start",
          block: "nearest",
        });
      }
      restartProgress();
    };

    const clearTimer = () => {
      if (timer) {
        window.clearInterval(timer);
        timer = 0;
      }
    };

    const armTimer = () => {
      clearTimer();
      if (reduce || paused || document.hidden) {
        if (progress) progress.classList.remove("is-running");
        return;
      }
      restartProgress();
      timer = window.setInterval(() => setActive(index + 1), INTERVAL);
    };

    const pause = () => {
      paused = true;
      clearTimer();
      if (progress) progress.classList.remove("is-running");
    };

    const resume = () => {
      paused = false;
      armTimer();
    };

    const onScroll = () => {
      const i = nearestIndex();
      if (i !== index) {
        index = i;
        setActive(i, { scroll: false });
      }
    };

    const onKey = (e) => {
      if (e.key === "ArrowRight") {
        e.preventDefault();
        setActive(index + 1);
        armTimer();
      } else if (e.key === "ArrowLeft") {
        e.preventDefault();
        setActive(index - 1);
        armTimer();
      }
    };

    prev?.addEventListener("click", () => {
      setActive(index - 1);
      armTimer();
    });
    next?.addEventListener("click", () => {
      setActive(index + 1);
      armTimer();
    });
    dots.forEach((dot) => {
      dot.addEventListener("click", () => {
        const i = Number(dot.getAttribute("data-carousel-dot") || "0");
        setActive(i);
        armTimer();
      });
    });

    track.addEventListener("scroll", onScroll, { passive: true });
    track.addEventListener("keydown", onKey);
    root.addEventListener("pointerenter", pause);
    root.addEventListener("pointerleave", resume);
    root.addEventListener("focusin", pause);
    root.addEventListener("focusout", (e) => {
      if (!root.contains(e.relatedTarget)) resume();
    });
    const onVis = () => (document.hidden ? pause() : resume());
    document.addEventListener("visibilitychange", onVis);

    setActive(0, { scroll: true, smooth: false });
    armTimer();

    stopCarousel = () => {
      clearTimer();
      track.removeEventListener("scroll", onScroll);
      track.removeEventListener("keydown", onKey);
      root.removeEventListener("pointerenter", pause);
      root.removeEventListener("pointerleave", resume);
      root.removeEventListener("focusin", pause);
      document.removeEventListener("visibilitychange", onVis);
      if (progress) progress.classList.remove("is-running");
      stopCarousel = null;
    };
  };

  const tryHero = () => {
    mountHeroParticles();
    mountHeroCarousel();
  };

  let navDrawerReady = false;
  const mountNavDrawer = () => {
    if (navDrawerReady) return;
    const toggle = document.querySelector("[data-nav-toggle]");
    const nav = document.querySelector("[data-site-nav]");
    const backdrop = document.querySelector("[data-nav-backdrop]");
    if (!toggle || !nav) return;
    navDrawerReady = true;

    const setOpen = (open) => {
      document.documentElement.classList.toggle("nav-open", open);
      const btn = document.querySelector("[data-nav-toggle]");
      const sheet = document.querySelector("[data-site-nav]");
      const veil = document.querySelector("[data-nav-backdrop]");
      if (btn) {
        btn.setAttribute("aria-expanded", open ? "true" : "false");
        btn.setAttribute("aria-label", open ? "Close menu" : "Open menu");
      }
      if (veil) {
        if (open) veil.removeAttribute("hidden");
        else veil.setAttribute("hidden", "");
      }
      if (open) sheet?.querySelector("a")?.focus({ preventScroll: true });
    };

    const close = () => setOpen(false);
    const isOpen = () => document.documentElement.classList.contains("nav-open");

    document.addEventListener("click", (e) => {
      const t = e.target instanceof Element ? e.target : null;
      if (!t) return;
      if (t.closest("[data-nav-toggle]")) {
        e.preventDefault();
        isOpen() ? close() : setOpen(true);
        return;
      }
      if (t.closest("[data-nav-backdrop]")) {
        close();
        return;
      }
      if (isOpen() && t.closest("[data-site-nav] a")) close();
    });
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && isOpen()) {
        close();
        document.querySelector("[data-nav-toggle]")?.focus({ preventScroll: true });
      }
    });
    document.addEventListener("resuma:navigate", close);
    window.addEventListener(
      "resize",
      () => {
        if (window.matchMedia("(min-width: 721px)").matches) close();
      },
      { passive: true },
    );
  };

  const boot = () => {
    tryHero();
    mountNavDrawer();
  };
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", boot, { once: true });
  } else {
    boot();
  }
  document.addEventListener("resuma:navigate", () => requestAnimationFrame(tryHero));
})();
