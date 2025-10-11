use eframe::egui;

#[derive(Default)]
struct App {
    model: Model,
    messages: Vec<Msg>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        view(ctx, &self.model, &mut self.messages);
        let msgs: Vec<_> = self.messages.drain(..).collect();
        for msg in msgs {
            let old = std::mem::take(&mut self.model);
            self.model = update(old, msg);
        }
    }
}

struct Model {
    counter: i32,
    label: String,
}

enum Msg {
    Increment,
    Decrement,
    Set(i32),
    NewLabel(String),
}

impl Model {
    pub fn init() -> Self {
        Self {
            counter: 0,
            label: String::from(""),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::init()
    }
}

fn update(model: Model, msg: Msg) -> Model {
    match msg {
        Msg::Increment => Model {
            counter: model.counter + 1,
            ..model
        },

        Msg::Decrement => Model {
            counter: model.counter - 1,
            ..model
        },

        Msg::Set(x) => Model {
            counter: x,
            ..model
        },

        Msg::NewLabel(string) => Model {
            label: string,
            ..model
        },
    }
}

fn view(ctx: &egui::Context, model: &Model, tx: &mut Vec<Msg>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Chai Tea");
        ui.horizontal(|ui| {
            let mut label = model.label.clone();
            ui.label("Write something: ");
            ui.text_edit_singleline(&mut label);
            tx.push(Msg::NewLabel(label));
        });

        ui.label(format!("label: {}, value: {}", model.label, model.counter));
        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                tx.push(Msg::Increment);
            }
            if ui.button("-").clicked() {
                tx.push(Msg::Decrement);
            }

            if ui.button("Set").clicked() {
                tx.push(Msg::Set(10));
            }
        });
    });
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Demo",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
