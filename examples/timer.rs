use eframe::egui;
//use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;

struct Model {
    total_time: u64,
    time_elapsed: u64,
    time_input: String,
    state: State,
}
enum State {
    Running,
    Paused,
    Stopped,
}

enum Msg {
    NewTime(String),
    Tick(u64),
    Start,
    Stop,
    Pause,
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

        Msg::Pause => (
            Model {
                state: State::Paused,
                ..model
            },
            None,
        ),

        Msg::Stop => (
            Model {
                time_elapsed: 0,
                state: State::Stopped,
                ..model
            },
            None,
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
    if matches!(model.state, State::Running) {
        ctx.request_repaint();
    }

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

        ui.horizontal(|ui| {
            match model.state {
                State::Running => {
                    if ui.button("pause").clicked() {
                        tx.push(Msg::Pause);
                    }
                }
                _ => {
                    if ui.button("start").clicked() {
                        tx.push(Msg::Start);
                    }
                }
            }

            if ui.button("stop").clicked() {
                tx.push(Msg::Stop);
            }
        });

        ui.horizontal(|ui| {
            let label = match model.state {
                State::Paused => "Paused",
                State::Running => "Running",
                State::Stopped => "Stopped",
            };
            ui.label(label);
        });
    });
}

struct SyncState {
    timer_lock: bool,
}

enum Cmd {
    Start(u64),
    Stop,
    Pause,
    Reset,
}

fn sync_state_init() -> SyncState {
    SyncState { timer_lock: false }
}

fn run_cmd(cmd: Cmd, sync_state: &mut SyncState, tx: Sender<Msg>) {
    match cmd {
        Cmd::Start(total_time) => {
            std::thread::spawn(move || {
                let start = std::time::Instant::now();
                let mut tick = 0;

                loop {
                    tick += 1;

                    let next = start + std::time::Duration::from_secs(tick);
                    let now = std::time::Instant::now();

                    let remaining = next.saturating_duration_since(now);
                    std::thread::sleep(remaining);

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

        Cmd::Stop => (),
        Cmd::Pause => (),
        Cmd::Reset => (),
    }
}

fn main() -> Result<(), eframe::Error> {
    chai_tea::brew_async("chai_timer", init, sync_state_init, update, view, run_cmd)
}
