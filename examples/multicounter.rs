use eframe::egui;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
struct Model {
    counter1: i64,
    counter2: i64,
    counter1_up: bool,
    counter2_up: bool,

    counter1_enabled: bool,
    counter2_enabled: bool,
}

enum Msg {
    Count1(i64),
    Count2(i64),
    Start1,
    Stop1,
    Start2,
    Stop2,
    CountDir1(bool),
    CountDir2(bool),
}

fn init() -> Model {
    Model {
        counter1_up: true,
        counter2_up: true,
        ..Default::default()
    }
}

fn update(model: Model, msg: Msg) -> (Model, Option<Cmd>) {
    match msg {
        Msg::Count1(val) => (
            Model {
                counter1: val,
                ..model
            },
            None,
        ),
        Msg::Count2(val) => (
            Model {
                counter2: val,
                ..model
            },
            None,
        ),
        Msg::Start1 => (
            Model {
                counter1_enabled: true,
                ..model
            },
            Some(Cmd::Start1),
        ),
        Msg::Start2 => (
            Model {
                counter2_enabled: true,
                ..model
            },
            Some(Cmd::Start2),
        ),
        Msg::Stop1 => (
            Model {
                counter1: 0,
                counter1_enabled: false,
                ..model
            },
            Some(Cmd::Stop1),
        ),
        Msg::Stop2 => (
            Model {
                counter2: 0,
                counter2_enabled: false,
                ..model
            },
            Some(Cmd::Stop2),
        ),
        Msg::CountDir1(val) => (
            Model {
                counter1_up: val,
                ..model
            },
            Some(Cmd::CountDir1(val)),
        ),
        Msg::CountDir2(val) => (
            Model {
                counter2_up: val,
                ..model
            },
            Some(Cmd::CountDir2(val)),
        ),
    }
}

fn view(ctx: &egui::Context, model: &Model, tx: &mut Vec<Msg>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Chai Double Counter");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("{}", model.counter1));
                if model.counter1_enabled {
                    if ui.button("stop").clicked() {
                        tx.push(Msg::Stop1);
                    }
                } else if ui.button("start").clicked() {
                    tx.push(Msg::Start1);
                }

                let mut up = model.counter1_up;
                if ui.checkbox(&mut up, "count up").changed() {
                    tx.push(Msg::CountDir1(up));
                }
            });

            ui.vertical(|ui| {
                ui.label(format!("{}", model.counter2));
                if model.counter2_enabled {
                    if ui.button("stop").clicked() {
                        tx.push(Msg::Stop2);
                    }
                } else if ui.button("start").clicked() {
                    tx.push(Msg::Start2);
                }
                let mut up = model.counter2_up;
                if ui.checkbox(&mut up, "count up").changed() {
                    tx.push(Msg::CountDir2(up));
                }
            });
        });
    });
}

struct SyncState {
    count_up_flag1: Arc<AtomicBool>,
    count_up_flag2: Arc<AtomicBool>,
    stop_flag1: Arc<AtomicBool>,
    stop_flag2: Arc<AtomicBool>,
}

enum Cmd {
    Start1,
    Start2,
    Stop1,
    Stop2,
    CountDir1(bool),
    CountDir2(bool),
}

fn sync_state_init() -> SyncState {
    SyncState {
        count_up_flag1: Arc::new(AtomicBool::new(true)),
        count_up_flag2: Arc::new(AtomicBool::new(true)),
        stop_flag1: Arc::new(AtomicBool::new(false)),
        stop_flag2: Arc::new(AtomicBool::new(false)),
    }
}

fn run_cmd(cmd: Cmd, sync_state: &mut SyncState, tx: chai_tea::ChaiSender<Msg>) {
    match cmd {
        Cmd::Start1 => {
            sync_state.stop_flag1.store(false, Ordering::SeqCst);
            let flag = sync_state.stop_flag1.clone();
            let count_up_flag = sync_state.count_up_flag1.clone();

            std::thread::spawn(move || {
                let mut counter: i64 = 0;
                loop {
                    if count_up_flag.load(Ordering::SeqCst) {
                        counter += 1;
                    } else {
                        counter -= 1;
                    }

                    if !interruptible_sleep(std::time::Duration::from_millis(300), &flag) {
                        return;
                    }

                    if tx.send(Msg::Count1(counter)).is_err() {
                        return;
                    }
                }
            });
        }

        Cmd::Start2 => {
            sync_state.stop_flag2.store(false, Ordering::SeqCst);
            let flag = sync_state.stop_flag2.clone();
            let count_up_flag = sync_state.count_up_flag2.clone();

            std::thread::spawn(move || {
                let mut counter: i64 = 0;
                loop {
                    if count_up_flag.load(Ordering::SeqCst) {
                        counter += 1;
                    } else {
                        counter -= 1;
                    }

                    if !interruptible_sleep(std::time::Duration::from_millis(1000), &flag) {
                        return;
                    }

                    if tx.send(Msg::Count2(counter)).is_err() {
                        return;
                    }
                }
            });
        }

        Cmd::Stop1 => sync_state.stop_flag1.store(true, Ordering::SeqCst),
        Cmd::Stop2 => sync_state.stop_flag2.store(true, Ordering::SeqCst),

        Cmd::CountDir1(val) => sync_state.count_up_flag1.store(val, Ordering::SeqCst),
        Cmd::CountDir2(val) => sync_state.count_up_flag2.store(val, Ordering::SeqCst),
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
    chai_tea::brew_async(
        "chai_counters",
        init,
        sync_state_init,
        update,
        view,
        run_cmd,
    )
}
