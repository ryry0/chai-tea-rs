# 🍵 chai-tea

> a minimal Elm-style architecture for [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) apps

**Status:** experimental — minimal working example only.
API may change rapidly as development continues.

---

**chai-tea** lets you write GUI apps in the same clean loop you’d use in The Elm
Architecture (TEA):

```rust
use chai_tea::*;
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
    chai_tea::run(init, update, view)
}
```

run it and you’ve got a fully working counter app.

`cargo run --example counter`

✨ features

- pure Elm-style loop (Model → Msg → update → view)
- no lifetimes, no borrowing hell — just ownership
- automatic mem::take pattern for smooth updates
- works on native and wasm targets (coming soon)
- tiny, dependency-light, and easy to extend

🫖 roadmap

- [ ] async / background command support
- [ ] fixed-timestep threaded simulation variant
- [ ] wasm runner (chai_tea::run_web)
- [ ] macro sugar: #[chai_app]
- [ ] theme system (chai-theme someday?)
