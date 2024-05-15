use test::Test;

mod test;


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    quiz: Test
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self { quiz: Test::dummy_test() }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            let question = self.quiz.question();

            ui.heading(&question.question);

            for answer in question.answers.iter_mut() {
                ui.checkbox(&mut answer.checked, &answer.answer);
            }
        });

        egui::TopBottomPanel::bottom("questions_move").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let prev_button = egui::Button::new("<<<<");
                if ui.add_enabled(
                    self.quiz.is_there_prev_question(), 
                    prev_button
                ).clicked() {
                    self.quiz.prev_question();
                };

                ui.label(format!("{question_number} / {total_number_of_questions}",
                    question_number = self.quiz.current_question_number() + 1,
                    total_number_of_questions = self.quiz.number_of_questions()));
                
                let next_button = egui::Button::new(">>>>");
                 if ui.add_enabled(
                    self.quiz.is_there_next_question(), 
                    next_button)
                    .clicked() {
                    self.quiz.next_question();
                 }

                 if !self.quiz.is_there_next_question() {
                    if ui.button("Finsih").clicked() {
                        
                    }
                 }
            })


        });
   }
}