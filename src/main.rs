

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
    quiz_url: Option<String>,
    #[arg(long, default_value_t = false)]
    disable_editor: bool,
    #[arg(short, long, default_value_t = String::from("My quiz app"))]
    title: String
}


async fn quiz_builder(args: Args) -> Quiz {
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
    let args = Args::parse();
    
    let _ = eframe::run_native(
        &args.title, 
        native_options, 
        Box::new(move |cc| {
            Box::new(
                QuizApp::new(cc, quiz)
                .enable_editor(!args.disable_editor))
        }));
}