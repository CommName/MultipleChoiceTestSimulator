
mod quiz_loader;
mod quiz_view;
mod quiz;
mod lang;

#[cfg(target_arch = "wasm32")]
mod wasm_lib {
    use super::*;
    use quiz::Quiz;
    use quiz_loader::QuizLoader;
    use quiz_view::QuizApp;
    use wasm_bindgen::prelude::*;
    
    
    #[wasm_bindgen]
    pub fn start_quiz(canves_id: String, quiz_url: String, disable_editor: bool) {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        let web_options = eframe::WebOptions::default();
        
        wasm_bindgen_futures::spawn_local(async move {    
            let quiz = QuizLoader::fetch_async(&quiz_url)
                .await
                .expect("Failed to fetch quiz");
        
            let start_result = eframe::WebRunner::new()
            .start(
                &canves_id,
                web_options,
                Box::new(move |cc| {
                    Box::new(QuizApp::new(cc, quiz).enable_editor(!disable_editor))
                } ),
                    
            )
            .await;
    });
    
    }
}