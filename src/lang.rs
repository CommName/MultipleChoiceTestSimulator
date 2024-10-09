
pub struct Labels {
    pub prev_question_button: String,
    pub next_question_button: String,
    pub restart_quiz_button: String,
    pub remove_qustion_button: String,
    pub new_question_button: String,
    pub enter_quiz_editor_button: String,
    pub enter_json_quiz_edditor_button: String,
    pub exit_quiz_editor_button: String,
    pub save_button: String,
    pub cancel_button: String,

    pub question_heading_editor: String,
    pub question_label_editor: String,
    pub add_answer_button_editor: String,
    pub remove_answer_button_editor: String,
    pub answer_col_heading_editor: String,
    pub correct_answers_col_heading_editor: String,
    pub actions_col_heading_editor: String,
}

impl Labels {
    pub fn english() -> Self {
        Self {
            prev_question_button: "Previous question".to_string(),
            next_question_button: "Next question".to_string(),
            restart_quiz_button: "Restart quiz".to_string(),
            remove_qustion_button: "Remove".to_string(),
            new_question_button: "New".to_string(),
            cancel_button: "Cancel".to_string(),
            save_button: "Save".to_string(),
            exit_quiz_editor_button: "Back".to_string(),
            enter_quiz_editor_button: "Edit quiz".to_string(),
            enter_json_quiz_edditor_button: "Edit json".to_string(),

            question_heading_editor: "Question editor".to_string(),
            question_label_editor: "Question:".to_string(),
            answer_col_heading_editor: "Answer".to_string(),
            correct_answers_col_heading_editor: "Correct".to_string(),
            actions_col_heading_editor: "Actions".to_string(),
            remove_answer_button_editor: "Remove".to_string(),
            add_answer_button_editor: "Add answer".to_string(),


        }
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self::english()
    }
}