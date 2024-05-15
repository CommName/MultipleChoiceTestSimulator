use std::vec;


pub struct Quiz {
    questions: Vec<Question>,
    name_of_quize: String,
    current_question: usize
}

pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>
}

pub struct Answer {
    pub answer: String,
    pub is_answer_correct: bool,
    pub checked: bool
}

impl Quiz {
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

    pub fn dummy_test() -> Self {
        Self {
            name_of_quize: "Dummy quize".to_string(),
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