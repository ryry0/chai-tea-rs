//! # 🍵 chai-tea
//!
//! A minimal Elm-style architecture for egui / eframe apps.
//!
//! # Example
//! ```no_run
//! use chai_tea::*;
//! use eframe::egui;
//!
//! struct Model { counter: i32 }
//! enum Msg { Inc, Dec }
//!
//! fn init() -> Model { Model { counter: 0 } }
//!
//! fn update(m: Model, msg: Msg) -> Model {
//!     match msg {
//!         Msg::Inc => Model { counter: m.counter + 1, ..m },
//!         Msg::Dec => Model { counter: m.counter - 1, ..m },
//!     }
//! }
//!
//! fn view(ctx: &egui::Context, m: &Model, tx: &mut Vec<Msg>) {
//!     egui::CentralPanel::default().show(ctx, |ui| {
//!         if ui.button("+").clicked() { tx.push(Msg::Inc); }
//!         if ui.button("–").clicked() { tx.push(Msg::Dec); }
//!         ui.label(m.counter.to_string());
//!     });
//! }
//!
//! fn main() -> eframe::Result<()> {
//!     chai_tea::run(init, update, view)
//! }
//! ```

use eframe::egui;

#[derive(Default)]
struct ChaiTeaApp<M, Msg, Fupdate, Fview> {
    model: M,
    messages: Vec<Msg>,
    update: Fupdate,
    view: Fview,
}

/// Run a chai-tea app with a model, update, and view function.
///
/// This is the minimal entry point. It wires up eframe and drives your Elm-style loop.
pub fn run<M, Msg, Finit, Fupdate, Fview>(
    init: Finit,
    update: Fupdate,
    view: Fview,
) -> eframe::Result<()>
where
    M: Default + 'static,
    Finit: Fn() -> M + 'static,
    Fupdate: Fn(M, Msg) -> M + Copy + 'static,
    Fview: Fn(&egui::Context, &M, &mut Vec<Msg>) + Copy + 'static,
    Msg: 'static,
{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "chai_tea",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(ChaiTeaApp {
                model: init(),
                messages: Vec::new(),
                update,
                view,
            }))
        }),
    )
}

impl<M, Msg, Fupdate, Fview> eframe::App for ChaiTeaApp<M, Msg, Fupdate, Fview>
where
    M: Default + 'static,
    Msg: 'static,
    Fupdate: Fn(M, Msg) -> M + Copy + 'static,
    Fview: Fn(&egui::Context, &M, &mut Vec<Msg>) + Copy + 'static,
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        (self.view)(ctx, &self.model, &mut self.messages);
        let msgs: Vec<_> = self.messages.drain(..).collect();
        for msg in msgs {
            let old = std::mem::take(&mut self.model);
            self.model = (self.update)(old, msg);
        }
    }
}
