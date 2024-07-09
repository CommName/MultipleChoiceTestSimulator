

use clap::Parser;
//use clap::Parser;
use quiz::Quiz;

mod quiz;
mod quiz_view;
mod quiz_loader;

use quiz_loader::QuizLoader;
use quiz_view::QuizApp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    quiz_url: Option<String>
}


pub async fn quiz_builder(args: Args) -> Quiz {
    if let Some(quiz_url) = args.quiz_url {
        QuizLoader::fetch_async(&quiz_url)
            .await
            .expect("Failed to fetch quiz")
    } else {
        Quiz::dummy_test()
    }
}

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
