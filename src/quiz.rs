use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Quiz {
    questions: Vec<Question>,
    current_question: usize
}

#[derive(Default, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>
}

#[derive(Default, Serialize, Deserialize)]
pub struct Answer {
    pub answer: String,
    pub is_answer_correct: bool,
    pub checked: bool
}

impl Quiz {

    pub fn randomize_quiz(&mut self) {
        self.current_question = 0;
        randomize_vec(&mut self.questions);
        self.questions.iter_mut()
            .for_each(|q| {
                randomize_vec(&mut q.answers);
                q.answers.iter_mut().for_each(|a| a.checked = false)
            });
    }

    pub fn number_of_correctly_answered_questions(&self) -> usize {
        self.questions.iter()
            .filter(|q| q.is_answered_correct())
            .count()
    }

    pub fn question(&mut self) -> &mut Question {
        self.questions.get_mut(self.current_question).unwrap()
    }

    pub fn number_of_questions(&self) -> usize {
        self.questions.len()
    }

    pub fn current_question_number(&self) -> usize {
        self.current_question
    }

    pub fn is_there_next_question(&self) -> bool {
        self.questions.len() > self.current_question + 1
    }

    pub fn next_question(&mut self) {
        if self.is_there_next_question()  {
            self.current_question += 1;
        }
    }

    pub fn is_there_prev_question(&self) -> bool {
        self.current_question > 0
    }

    pub fn prev_question(&mut self) {
        if self.is_there_prev_question() {
            self.current_question -= 1;
        }
    }

    pub fn add_new_question(&mut self) {
        self.questions.push(Default::default());
    }

    pub fn remove_current_question(&mut self) {
        if self.questions.len() > 1 {
            self.questions.remove(self.current_question);
            if self.current_question != 0 {
                self.current_question -= 1;
            }
        }
    }

    pub fn dummy_test() -> Self {
        Self {
            current_question: 0,
            questions: vec![
                Question {
                    question: "Question 1".to_string(),
                    answers: vec![
                        Answer {
                            answer: "Answer 1".to_string(),
                            is_answer_correct: false,
                            checked: false,
                        },
                        Answer {
                            answer: "Answer 2".to_string(),
                            is_answer_correct: true,
                            checked: false,
                        }
                    ]
                },
                Question {
                    question: "Question 2".to_string(),
                    answers: vec![
                        Answer {
                            answer: "Answer 1".to_string(),
                            is_answer_correct: true,
                            checked: false,
                        },
                        Answer {
                            answer: "Answer 2".to_string(),
                            is_answer_correct: false,
                            checked: false,
                        }
                    ]
                }
            ]
        }
    }
}

impl Question {

    pub fn get_points(&self) -> f32 {
        match self.is_answered_correct() {
            true => 1.0,
            false => 0.0
        }
    }

    pub fn is_answered_correct(&self) -> bool {
        self.answers.iter().all(|a| a.is_answered_correct())
    }
}

impl Answer {
    pub fn is_answered_correct(&self) -> bool {
        self.checked == self.is_answer_correct
    }
}

fn randomize_vec<T>(vec: &mut Vec<T>) {

    let mut tmp = Vec::new();


    while !vec.is_empty() {
        let index = rand::random::<usize>() % vec.len();
        tmp.push(vec.remove(index));
    }

    *vec = tmp;
}