use egui::{Color32, TextStyle, WidgetText};
use quiz::Quiz;
use egui::*;

mod quiz;


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    quiz: Quiz,
    answered: bool
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self { 
            quiz: Quiz::dummy_test(),
            answered: false
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            let question = self.quiz.question();
            ui.set_enabled(!self.answered);

            ui.heading(&question.question);

            for answer in question.answers.iter_mut() {
                let text_color = if self.answered && !answer.is_answered_correct() && answer.checked {
                    Color32::from_rgb(255, 0, 0)
                } else if self.answered && answer.checked {
                    Color32::from_rgb(0, 255, 0)   
                }else {
                    Color32::from_rgb(120, 120, 120)
                };
                ui.checkbox(&mut answer.checked, RichText::new(&answer.answer).color(text_color));
            }
        });

        egui::TopBottomPanel::bottom("questions_move").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let prev_button = egui::Button::new("<<<<");
                if ui.add_enabled(
                    self.quiz.is_there_prev_question() || self.answered, 
                    prev_button
                ).clicked() {
                    if self.answered {
                        self.answered = false;
                    } else {
                        self.quiz.prev_question();
                    }
                };

                ui.label(format!("{question_number} / {total_number_of_questions}",
                    question_number = self.quiz.current_question_number() + 1,
                    total_number_of_questions = self.quiz.number_of_questions()));
                
                let next_button = egui::Button::new(">>>>");
                 if ui.add(next_button)
                    .clicked() {
                        if self.answered {
                            self.quiz.next_question();
                        }
                        self.answered = !self.answered;
                 }

            })


        });
   }
}