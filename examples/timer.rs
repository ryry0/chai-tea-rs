use eframe::egui;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct Model {
    total_time: u64,
    time_elapsed: u64,
    time_input: String,
    state: State,
}
enum State {
    Running,
    Stopped,
}

enum Msg {
    NewTime(String),
    Tick(u64),
    Start,
    Stop,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            total_time: 10,
            time_elapsed: 0,
            time_input: String::from("10"),
            state: State::Stopped,
        }
    }
}

fn init() -> Model {
    Model::default()
}

fn update(model: Model, msg: Msg) -> (Model, Option<Cmd>) {
    match msg {
        Msg::NewTime(time) => match time.parse() {
            Ok(total_time) => (
                Model {
                    total_time,
                    time_input: time,
                    ..model
                },
                None,
            ),
            _ => (model, None),
        },

        Msg::Stop => (
            Model {
                time_elapsed: 0,
                state: State::Stopped,
                ..model
            },
            Some(Cmd::Stop),
        ),

        Msg::Start => (
            Model {
                state: State::Running,
                ..model
            },
            Some(Cmd::Start(model.total_time)),
        ),

        Msg::Tick(secs) => (
            Model {
                time_elapsed: secs,
                ..model
            },
            None,
        ),
    }
}

fn view(ctx: &egui::Context, model: &Model, tx: &mut Vec<Msg>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Chai Tea Timer");
        ui.horizontal(|ui| match model.state {
            State::Stopped => {
                ui.label("Input time (s):");
                let mut label = model.time_input.clone();
                if ui.text_edit_singleline(&mut label).changed() {
                    tx.push(Msg::NewTime(label));
                }
            }
            _ => {
                ui.label(format!(
                    "Total: {}s, Elapsed: {}s",
                    model.total_time, model.time_elapsed
                ));
            }
        });

        ui.horizontal(|ui| match model.state {
            State::Running => {
                if ui.button("stop").clicked() {
                    tx.push(Msg::Stop);
                }
            }
            _ => {
                if ui.button("start").clicked() {
                    tx.push(Msg::Start);
                }
            }
        });
    });
}

struct SyncState {
    stop_flag: Arc<AtomicBool>,
}

enum Cmd {
    Start(u64),
    Stop,
}

fn sync_state_init() -> SyncState {
    SyncState {
        stop_flag: Arc::new(AtomicBool::new(false)),
    }
}

fn run_cmd(cmd: Cmd, sync_state: &mut SyncState, tx: chai_tea::ChaiSender<Msg>) {
    match cmd {
        Cmd::Start(total_time) => {
            sync_state.stop_flag.store(false, Ordering::SeqCst);
            let flag = sync_state.stop_flag.clone();

            std::thread::spawn(move || {
                let start = std::time::Instant::now();
                let mut tick = 0;

                loop {
                    tick += 1;

                    let next = start + std::time::Duration::from_secs(tick);
                    let now = std::time::Instant::now();

                    let remaining = next.saturating_duration_since(now);

                    if !interruptible_sleep(remaining, &flag) {
                        break;
                    }

                    if tx.send(Msg::Tick(tick)).is_err() {
                        return;
                    }

                    if tick >= total_time {
                        if tx.send(Msg::Stop).is_err() {
                            return;
                        }
                        break;
                    }
                }
            });
        }

        Cmd::Stop => sync_state.stop_flag.store(true, Ordering::SeqCst),
    }
}

fn interruptible_sleep(total: std::time::Duration, flag: &AtomicBool) -> bool {
    let step = std::time::Duration::from_millis(10);
    let start = std::time::Instant::now();
    while std::time::Instant::now() - start < total {
        if flag.load(Ordering::SeqCst) {
            return false; // interrupted
        }
        std::thread::sleep(step);
    }
    true // completed
}

fn main() -> Result<(), eframe::Error> {
    chai_tea::brew_async("chai_timer", init, sync_state_init, update, view, run_cmd)
}
