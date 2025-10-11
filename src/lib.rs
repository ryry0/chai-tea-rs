use eframe::egui;

#[derive(Default)]
struct ChaiTeaApp<M, Msg, Fupdate, Fview> {
    model: M,
    messages: Vec<Msg>,
    update: Fupdate,
    view: Fview,
}

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
