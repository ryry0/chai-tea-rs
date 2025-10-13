//! # 🍵 chai-tea
//!
//! A minimal Elm-style architecture for egui / eframe apps.
//!
//! # Example
//! ```no_run
//! use eframe::egui;
//!
//! #[derive(Default)]
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
//!     chai_tea::run("chai app", init, update, view)
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
    title: &str,
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
        title,
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

/// An alias for [`run`]. 🍵
///
/// # Example
/// ```no_run
/// # use eframe::egui;
/// # fn init() -> i32 { 1 }
/// # fn update(m: i32, msg: i32) -> i32 { 1 }
/// # fn view(ctx: &egui::Context, m: &i32, tx: &mut Vec<i32>) { }
/// chai_tea::brew("chai_app", init, update, view);
/// ```
///
/// Equivalent to:
/// ```no_run
/// # use eframe::egui;
/// # fn init() -> i32 { 1 }
/// # fn update(m: i32, msg: i32) -> i32 { 1 }
/// # fn view(ctx: &egui::Context, m: &i32, tx: &mut Vec<i32>) { }
/// chai_tea::run("chai_app", init, update, view);
/// ```
#[inline(always)]
pub fn brew<M, Msg, Finit, Fupdate, Fview>(
    title: &str,
    init: Finit,
    update: Fupdate,
    view: Fview,
) -> eframe::Result<()>
where
    M: Default + 'static,
    Msg: 'static,
    Finit: Fn() -> M + 'static,
    Fupdate: Fn(M, Msg) -> M + Copy + 'static,
    Fview: Fn(&egui::Context, &M, &mut Vec<Msg>) + Copy + 'static,
{
    run(title, init, update, view)
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

struct ChaiTeaAppAsync<M, S, Cmd, Msg, Fupdate, Fview, Fcmd> {
    model: M,
    sync_state: S,
    messages: Vec<Msg>,
    update: Fupdate,
    view: Fview,
    run_cmd: Fcmd,
    chai_tx: ChaiSender<Msg>,
    msg_rx: std::sync::mpsc::Receiver<Msg>,
    _phantom_cmd: std::marker::PhantomData<Cmd>,
}

pub struct ChaiSender<T> {
    tx: std::sync::mpsc::Sender<T>,
    ctx: Option<egui::Context>,
}

impl<T> ChaiSender<T> {
    pub fn new(tx: std::sync::mpsc::Sender<T>) -> Self {
        Self { tx, ctx: None }
    }

    pub fn set_ctx(&mut self, ctx: &egui::Context) {
        self.ctx = Some(ctx.clone());
    }

    pub fn send(&self, msg: T) -> Result<(), std::sync::mpsc::SendError<T>> {
        if let Some(ctx) = &self.ctx {
            ctx.request_repaint();
        }
        self.tx.send(msg)
    }

    pub fn send_repaintless(&self, msg: T) -> Result<(), std::sync::mpsc::SendError<T>> {
        self.tx.send(msg)
    }
}

impl<T> std::ops::Deref for ChaiSender<T> {
    type Target = std::sync::mpsc::Sender<T>;
    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

impl<T> Clone for ChaiSender<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            ctx: self.ctx.clone(),
        }
    }
}

/// An alias for [`run_async`]. 🍵
///
/// # Example
/// ```no_run
/// # use eframe::egui;
/// # fn init() -> i32 { 1 }
/// # fn sync_state_init() -> i32 { 1 }
/// # fn update(m: i32, msg: i32) -> (i32, Option<i32>) { (1, None) }
/// # fn view(ctx: &egui::Context, m: &i32, tx: &mut Vec<i32>) { }
/// # fn run_cmd(cmd: i32, sync: &mut i32, tx: chai_tea::ChaiSender<i32>) { }
/// chai_tea::brew_async("chai_app", init, sync_state_init, update, view, run_cmd);
/// ```
///
/// Equivalent to:
/// ```no_run
/// # use eframe::egui;
/// # fn init() -> i32 { 1 }
/// # fn sync_state_init() -> i32 { 1 }
/// # fn update(m: i32, msg: i32) -> (i32, Option<i32>) { (1, None) }
/// # fn view(ctx: &egui::Context, m: &i32, tx: &mut Vec<i32>) { }
/// # fn run_cmd(cmd: i32, sync: &mut i32, tx: chai_tea::ChaiSender<i32>) { }
/// chai_tea::run_async("chai_app", init, sync_state_init, update, view, run_cmd);
/// ```
#[inline(always)]
pub fn brew_async<M, S, Cmd, Msg, Finit, FsyncInit, Fupdate, Fview, Fcmd>(
    title: &str,
    init: Finit,
    sync_state_init: FsyncInit,
    update: Fupdate,
    view: Fview,
    run_cmd: Fcmd,
) -> eframe::Result<()>
where
    M: Default + 'static,
    S: 'static,
    Cmd: 'static,
    Finit: Fn() -> M + 'static,
    FsyncInit: Fn() -> S + 'static,
    Fupdate: Fn(M, Msg) -> (M, Option<Cmd>) + Copy + 'static,
    Fview: Fn(&egui::Context, &M, &mut Vec<Msg>) + Copy + 'static,
    Fcmd: Fn(Cmd, &mut S, ChaiSender<Msg>) + Copy + Send + Sync + 'static,
    Msg: 'static,
{
    run_async(title, init, sync_state_init, update, view, run_cmd)
}

/// Run an async chai-tea app with a model, update, and view and async run_cmd function.
///
/// This is the minimal entry point. It wires up eframe and drives your Elm-style loop.
pub fn run_async<M, S, Cmd, Msg, Finit, FsyncInit, Fupdate, Fview, Fcmd>(
    title: &str,
    init: Finit,
    sync_state_init: FsyncInit,
    update: Fupdate,
    view: Fview,
    run_cmd: Fcmd,
) -> eframe::Result<()>
where
    M: Default + 'static,
    S: 'static,
    Cmd: 'static,
    Finit: Fn() -> M + 'static,
    FsyncInit: Fn() -> S + 'static,
    Fupdate: Fn(M, Msg) -> (M, Option<Cmd>) + Copy + 'static,
    Fview: Fn(&egui::Context, &M, &mut Vec<Msg>) + Copy + 'static,
    Fcmd: Fn(Cmd, &mut S, ChaiSender<Msg>) + Copy + Send + Sync + 'static,
    Msg: 'static,
{
    let options = eframe::NativeOptions::default();
    let (msg_tx, msg_rx) = std::sync::mpsc::channel();

    let chai_tx = ChaiSender::new(msg_tx);

    eframe::run_native(
        title,
        options,
        Box::new(move |_cc| {
            Ok(Box::new(ChaiTeaAppAsync {
                model: init(),
                sync_state: sync_state_init(),
                messages: Vec::new(),
                update,
                view,
                run_cmd,
                chai_tx,
                msg_rx,
                _phantom_cmd: std::marker::PhantomData,
            }))
        }),
    )
}

impl<M, S, Cmd, Msg, Fupdate, Fview, Fcmd> eframe::App
    for ChaiTeaAppAsync<M, S, Cmd, Msg, Fupdate, Fview, Fcmd>
where
    M: Default + 'static,
    S: 'static,
    Cmd: 'static,
    Msg: 'static,
    Fupdate: Fn(M, Msg) -> (M, Option<Cmd>) + Copy + 'static,
    Fview: Fn(&egui::Context, &M, &mut Vec<Msg>) + Copy + 'static,
    Fcmd: Fn(Cmd, &mut S, ChaiSender<Msg>) + Copy + Send + Sync + 'static,
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        static ONCE: std::sync::Once = std::sync::Once::new();

        ONCE.call_once(|| {
            self.chai_tx.set_ctx(ctx);
        });

        //get view messages
        (self.view)(ctx, &self.model, &mut self.messages);
        let mut msgs: Vec<_> = self.messages.drain(..).collect();
        let mut cmds = Vec::<Cmd>::new();

        //get async messages
        while let Ok(msg) = self.msg_rx.try_recv() {
            msgs.push(msg);
        }

        //handle them all
        for msg in msgs {
            let old = std::mem::take(&mut self.model);
            let (new_model, cmd) = (self.update)(old, msg);
            self.model = new_model;
            if let Some(cmd) = cmd {
                cmds.push(cmd);
            }
        }

        //run async cmds
        for cmd in cmds {
            let tx = ChaiSender::clone(&self.chai_tx);
            (self.run_cmd)(cmd, &mut self.sync_state, tx);
        }
    }
}
