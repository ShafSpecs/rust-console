// mod error_handling;
use std::{thread, time::Duration};

use console::{style, Style, Term};

use indicatif::{ProgressBar, ProgressStyle};

// use dialoguer::Editor;
// use dialoguer::Completion;
// use dialoguer::Input;

fn main() {
    // Print to standard output
    let term = Term::stdout();
    term.write_line("Hello World!")
        .expect("Expected a newline to write to!");
    thread::sleep(Duration::from_millis(2000));

    term.clear_line().unwrap();

    // Create a style instance, can be saved and re-used later
    let green = Style::new().green();

    // can also be applied inline
    println!(
        "This is {} {}!",
        style("quite").cyan(),
        green.apply_to("nice")
    );

    // still need to read up error handling

    /* Dialoguer */
    // Error
    // let editor = Editor::new();
    // let s = editor.edit("Hello, Worl");

    // Error
    // let completion = Completion::get(&self, "What do you want to eat?");
    // println!("You want to eat: {:#?}", completion);

    // let input = Input::new();
    // input.default("What do you want?").history_with(history);

    // Adding another 'module' in Rust
    // Surprisingly running after the code below have run
    // error_handling::printer();

    /* Indicatif */
    let progress = ProgressBar::new(1000);

    for _ in 0..20 {
        thread::sleep(Duration::from_millis(500));
        progress.inc(50)
    }

    progress.finish();

    // example from the repo (indicatif)
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message("Calculating...");
    thread::sleep(Duration::from_secs(5));
    pb.finish_with_message("Done");
}
