use eframe::egui::{__run_test_ui, CentralPanel, Context};
use eframe::Frame;

pub struct QuizApp{

}

impl Default for QuizApp{
    fn default() -> Self {
        Self{

        }
    }
}
impl eframe::App for QuizApp{
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
       CentralPanel::default().show(ctx, |ui|{
            ui.heading("Yanni i will kill you")
       });


    }
}