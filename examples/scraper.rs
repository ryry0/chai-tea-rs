use eframe::egui;
use scraper::{Html, Selector};

#[derive(Default)]
struct Model {
    countries: Vec<String>,
}

enum Msg {
    GetCountries,
    CountryList(Vec<String>),
}

fn init() -> Model {
    Model::default()
}

fn update(_model: Model, msg: Msg) -> (Model, Option<Cmd>) {
    match msg {
        Msg::GetCountries => (
            Model {
                countries: vec!["Loading...".into()],
            },
            Some(Cmd::GetCountries),
        ),

        Msg::CountryList(list) => (Model { countries: list }, None),
    }
}

fn view(ctx: &egui::Context, model: &Model, tx: &mut Vec<Msg>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Chai Tea Countries");
        ui.vertical(|ui| {
            if ui.button("Get Countries").clicked() {
                tx.push(Msg::GetCountries);
            }

            ui.vertical(|ui| {
                model.countries.iter().take(10).for_each(|country| {
                    ui.label(country.to_string());
                });
            });
        });
    });
}

struct SyncState {}

enum Cmd {
    GetCountries,
}

fn sync_state_init() -> SyncState {
    SyncState {}
}

fn run_cmd(cmd: Cmd, _sync_state: &mut SyncState, tx: chai_tea::ChaiSender<Msg>) {
    match cmd {
        Cmd::GetCountries => {
            tokio::spawn(async move {
                let url = "https://www.scrapethissite.com/pages/simple/";
                let response = reqwest::get(url).await.unwrap().text().await.unwrap();
                let document = Html::parse_document(&response);
                let selector = Selector::parse("h3").unwrap();

                let elements: Vec<_> = document
                    .select(&selector)
                    .map(|x| {
                        x.text()
                            .collect::<Vec<_>>()
                            .concat()
                            .chars()
                            .filter(|c| *c != '\n')
                            .collect::<String>()
                    })
                    .map(|x| x.trim().to_string())
                    .collect();

                tx.send(Msg::CountryList(elements)).ok();
            });
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    chai_tea::brew_async("chai_scraper", init, sync_state_init, update, view, run_cmd)
}
