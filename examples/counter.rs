use eframe::egui;

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

impl Default for Model {
    fn default() -> Self {
        Self {
            counter: 1,
            label: String::from("hello world"),
        }
    }
}

fn init() -> Model {
    Model::default()
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
    chai_tea::run(init, update, view)
}
