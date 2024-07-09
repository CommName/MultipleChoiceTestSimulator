
mod quiz_loader;
mod quiz_view;
mod quiz;
#[cfg(target_arch = "wasm32")]
mod wasm_lib {
    use super::*;
    use quiz::Quiz;
    use quiz_loader::QuizLoader;
    use quiz_view::QuizApp;
    use wasm_bindgen::prelude::*;
    
    
    #[wasm_bindgen]
    pub fn start_quiz() {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        let web_options = eframe::WebOptions::default();
        
        wasm_bindgen_futures::spawn_local(async {
            
            let quiz = Quiz::dummy_test();
        
            log::info!("hello world 1");
            let start_result = eframe::WebRunner::new()
            .start(
                "quiz_app", // hardcode it
                web_options,
                Box::new(|cc| Box::new(QuizApp::new(cc, quiz))),
            )
            .await;
        let args = std::env::args();
        
        log::info!("hello world 2 {:?}", args);
    });
    
    }
}