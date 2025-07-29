use std::env;
use std::process;

use musicxml_analysis::analyze_note_density;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <musicxml-file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    match musicxml::read_score_partwise(file_path) {
        Ok(score) => {
            let metrics = analyze_note_density(&score);
            println!(
                "Average: {:>5.2} notes/second",
                metrics.average_notes_per_second
            );
            println!(
                "Peak   : {:>5.2} notes/second",
                metrics.peak_notes_per_second
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
