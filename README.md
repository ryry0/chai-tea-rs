# 🍵 chai-tea

> a minimal Elm-style architecture for [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/main/crates/eframe) apps

**Status:** early-stage but functional — now with async / background task support.
API may change rapidly as development continues.

---

**chai-tea** lets you write GUI apps in the same clean loop you’d use in The Elm Architecture (TEA):

```rust
use eframe::egui;

#[derive(Default)]
struct Model { counter: i32 }
enum Msg { Inc, Dec }

fn init() -> Model { Model { counter: 0 } }

fn update(m: Model, msg: Msg) -> Model {
    match msg {
        Msg::Inc => Model { counter: m.counter + 1, ..m },
        Msg::Dec => Model { counter: m.counter - 1, ..m },
    }
}

fn view(ctx: &egui::Context, m: &Model, tx: &mut Vec<Msg>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("+").clicked() { tx.push(Msg::Inc); }
        if ui.button("–").clicked() { tx.push(Msg::Dec); }
        ui.label(m.counter.to_string());
    });
}

fn main() -> eframe::Result<()> {
    chai_tea::brew("chai_app", init, update, view)
}
```

add eframe to your dependencies, run it and you’ve got a fully working counter app.

`cargo run --example counter`

## 🧩 example: async counters

A minimal demonstration of concurrent background workers, shared atomic state, and repaint-on-message behavior is included under

`cargo run --example multicounter`

Each counter runs in its own thread and reports back through ChaiSender, automatically triggering redraws.

## 🌐 Async example

Using tokio + reqwest + scraper, chai-tea cleanly handles real async I/O: 

`cargo run --example scraper`

Fetches a live web page, parses HTML, and updates the UI — all while keeping a pure Elm-style architecture.


## ✨ features

- 🍃 Pure Elm-style loop — deterministic, functional, and testable
- 🧵 Async commands via `brew_async`, `SyncState`, and `ChaiSender`
    - spawn background threads or async tasks
    - send messages back safely
    - UI automatically repaints on message arrival
- 🌐 Native + (soon) WASM support
- 🪶 Tiny, dependency-light core
- ☕ Ergonomic aliases — `brew` = `run`, `brew_async` = `run_async`

## 🫖 possible roadmap

- [x] async / background command support
- [ ] fixed-timestep threaded simulation variant
- [ ] wasm runner (chai_tea::run_web) (in progress)
- [ ] macro sugar: #[chai_app]
- [ ] theme system (chai-latte someday?)
- [ ] time travel debugger?
- [ ] winit + wgpu + egui version (in progress)
