# Habilis-β Project Page

Project page for **Habilis-β: A Fast-Motion and Long-Lasting On-Device Vision-Language-Action Model**.

Built with [Leptos](https://leptos.dev/) + [Axum](https://github.com/tokio-rs/axum) (SSR + Islands architecture) and [Tailwind CSS](https://tailwindcss.com/).

## Prerequisites

- **Rust nightly** with `wasm32-unknown-unknown` target (handled by `rust-toolchain.toml`)
- **cargo-leptos**: `cargo install cargo-leptos`
- **Tailwind CSS CLI**: `npm install -g tailwindcss` (or cargo-leptos auto-detects it)

## Development

```bash
cargo leptos watch
```

Serves the site at [http://localhost:3000](http://localhost:3000) with hot-reload.

## Production Build

```bash
cargo leptos build --release
```

The output goes to `target/site/`. Run the server binary:

```bash
./target/release/habilis-beta
```

## Project Structure

```
src/
  main.rs               # Axum server entry point
  lib.rs                # WASM hydration entry point
  app.rs                # Root App component + HTML shell
  pages/
    home.rs             # Full page assembling all sections
  components/
    hero.rs             # Title, authors, paper links
    teaser.rs           # Teaser demo video
    abstract_section.rs # Abstract text
    rethinking_eval.rs  # PRP figure + evaluation protocol
    system_overview.rs  # Model architecture diagram
    simulation_perf.rs  # Simulation charts + video tabs
    real_world_perf.rs  # Real-world results + failure cases
    bibtex.rs           # Citation block
    video_card.rs       # Reusable video card
    section_wrapper.rs  # Reusable section container
  islands/
    task_tabs.rs        # Interactive tab switcher (WASM)
    copy_button.rs      # Copy-to-clipboard button (WASM)
style/
  tailwind.css          # Tailwind directives + custom utilities
public/
  images/               # SVG/PNG assets
  videos/               # MP4 demo videos
```
