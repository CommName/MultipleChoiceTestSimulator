

//use clap::Parser;
use quiz::Quiz;

mod quiz;
mod quiz_view;
mod quiz_loader;

use quiz_view::QuizApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {

    let native_options = eframe::NativeOptions::default();
    let quiz = Quiz::dummy_test();

    let _ = eframe::run_native(
        "My quiz app", 
        native_options, 
        Box::new(|cc| Box::new(QuizApp::new(cc, quiz))));
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use quiz_loader::QuizLoader;


    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();
//let quiz  = Quiz::dummy_test();

wasm_bindgen_futures::spawn_local(async {
        let quiz = QuizLoader::fetch_async("./test.json")
            .await
            .unwrap();
        let start_result = eframe::WebRunner::new()
            .start(
                "quiz_app", // hardcode it
                web_options,
                Box::new(|cc| Box::new(QuizApp::new(cc, quiz))),
            )
            .await;
        let loading_text = eframe::web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        match start_result {
            Ok(_) => {
                loading_text.map(|e| e.remove());
            }
            Err(e) => {
                loading_text.map(|e| {
                    e.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    )
                });
                panic!("failed to start eframe: {e:?}");
            }
        }
    });

}