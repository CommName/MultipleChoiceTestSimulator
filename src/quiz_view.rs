use crate::quiz::Quiz;
use crate::lang::Labels;

use egui_extras::{Column, TableBuilder};
use egui::*;

#[derive(Default)]
struct QuizJsonEditor {
    error: Option<String>,
    quiz_json: String,
}

enum  QuizView {
    Quiz,
    Editor,
    EditorJson    
}


pub struct  QuizApp {
    quiz: Quiz,
    answered: bool,
    view: QuizView,
    enable_editor: bool,
    quiz_json: Option<QuizJsonEditor>,
    labels: Labels
}


impl QuizApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, quiz: Quiz) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self { 
            quiz,
            answered: false,
            view: QuizView::Quiz,
            enable_editor: true,
            quiz_json: Default::default(),
            labels: Default::default()
        }
    }

    pub fn enable_editor(self, enable_editor: bool) -> Self {
        Self {
            enable_editor,
            ..self
        }
    }

    fn draw_quiz_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

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
                let prev_button = egui::Button::new(&self.labels.prev_question_button);
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
                
                let next_button = egui::Button::new(&self.labels.next_question_button);
                if ui.add(next_button)
                    .clicked() {
                    if self.answered {
                        self.quiz.next_question();
                    }
                    self.answered = !self.answered;
                }

                if ui.button(&self.labels.restart_quiz_button).clicked() {
                    self.quiz.randomize_quiz();
                    self.answered = false;
                }

                if self.enable_editor {
                    if ui.button(&self.labels.enter_quiz_editor_button).clicked() {
                        self.view = QuizView::Editor
                    }
                }

            })


        });
    }

    fn draw_editor_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading(format!("{question_heading} ({question_number} / {number_of_questions})",
                    question_heading=self.labels.question_heading_editor, 
                    question_number=self.quiz.current_question_number() + 1,
                    number_of_questions=self.quiz.number_of_questions()));

                let question = self.quiz.question();

                ui.horizontal(|ui| {
                    ui.strong(&self.labels.question_label_editor);
                    ui.text_edit_singleline(&mut question.question);
                });

                TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder())
                    .column(Column::auto())
                    .column(Column::auto())
                    .striped(true)
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong(&self.labels.answer_col_heading_editor);
                        });
                        header.col(|ui| {
                            ui.strong(&self.labels.correct_answers_col_heading_editor);
                        });
                        header.col(|ui|{
                            ui.strong(&self.labels.actions_col_heading_editor);
                        });

                    })
                    .body(|mut body| {
                        let mut index_to_delete = None;
                        for (index, answer) in question.answers.iter_mut().enumerate() {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.text_edit_singleline(&mut answer.answer);
                                });
                                row.col(|ui| {
                                    ui.checkbox(&mut answer.is_answer_correct, "");
                                });

                                row.col(|ui| {
                                    if ui.button(&self.labels.remove_answer_button_editor).clicked() {
                                        let _ = index_to_delete.insert(index);
                                    }
                                });
                            })
                        }
                        if let Some(index) = index_to_delete {
                            question.answers.remove(index);
                        }
                });
                
                if ui.button(&self.labels.add_answer_button_editor).clicked() {
                    question.answers.push(Default::default());
                }
            });
        });

        egui::TopBottomPanel::bottom("Editor naviagtion bar").show(ctx, |ui| {
            ui.horizontal(|ui| {

                if ui.button(&self.labels.exit_quiz_editor_button).clicked() {
                    self.view = QuizView::Quiz
                }
                
                if ui.button(&self.labels.new_question_button).clicked() {
                    self.quiz.add_new_question();
                    while self.quiz.is_there_next_question() {
                        self.quiz.next_question();
                    }
                }

                if ui.button(&self.labels.remove_qustion_button).clicked() {
                    self.quiz.remove_current_question();
                }

                if ui.button(&self.labels.next_question_button).clicked() {
                    self.quiz.next_question();
                }

                if ui.button(&self.labels.prev_question_button).clicked() {
                    self.quiz.prev_question();
                }

                if ui.button(&self.labels.enter_json_quiz_edditor_button).clicked() {
                    self.view = QuizView::EditorJson;
                }
            });
        });
    }

    fn draw_editor_json_view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.quiz_json.is_none() {
            let _ = self.quiz_json.insert(QuizJsonEditor {
                error: None,
                quiz_json: serde_json::to_string_pretty(&self.quiz).unwrap()
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref mut quiz_json) = self.quiz_json {
                if let Some(ref error) = quiz_json.error {
                    ui.label(RichText::new(error).color(Rgba::RED));
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut quiz_json.quiz_json)
                        .desired_width(f32::INFINITY)
                    );
                });
            }
        });
        egui::TopBottomPanel::bottom("Editor json navigator bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(&self.labels.cancel_button).clicked() {
                    let _ = self.quiz_json.take();
                    self.view = QuizView::Editor;
                }
    
                if ui.button(&self.labels.save_button).clicked() {
                    if let Some(ref mut quiz_json) = self.quiz_json {
                        match serde_json::from_str::<Quiz>(&quiz_json.quiz_json) {
                            Ok(quiz) => {
                                self.quiz = quiz;
                                let _ = self.quiz_json.take();
                                self.view = QuizView::Editor;
    
                            }, 
                            Err(e) => {
                                quiz_json.error = Some(format!("{e}"));
                            }
                        }
                    }
                }
            });
        });
    }
}

impl eframe::App for QuizApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.view {
            QuizView::Quiz => self.draw_quiz_view(ctx, frame),
            QuizView::Editor => self.draw_editor_view(ctx, frame),
            QuizView::EditorJson => self.draw_editor_json_view(ctx, frame)
        }
   }
}