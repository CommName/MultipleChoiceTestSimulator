use quiz::Quiz;
use egui::*;

mod quiz;


fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(QuizApp::new(cc))));
}

struct  QuizApp {
    quiz: Quiz,
    answered: bool
}

impl QuizApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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

impl eframe::App for QuizApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            let question = self.quiz.question();
            ui.set_enabled(!self.answered);

            ui.heading(
                RichText::new(&question.question)
                    .color(if self.answered {
                        if question.is_answered_correct() {
                            Color32::from_rgb(0, 255, 0)
                        } else {
                            Color32::from_rgb(255, 0, 0)
                        }
                    } else {
                        Color32::from_rgb(125, 125, 125)
                    })
            );

            for answer in question.answers.iter_mut() {
                let text_color = if self.answered && !answer.is_answered_correct() && answer.checked {
                    Color32::from_rgb(255, 0, 0)
                } else if self.answered && answer.checked {
                    Color32::from_rgb(0, 255, 0)   
                }else {
                    Color32::from_rgb(125, 125, 125)
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

                if ui.button("Restart quiz").clicked() {
                    self.quiz.randomize_quiz();
                    self.answered = false;
                }


            })


        });
   }
}