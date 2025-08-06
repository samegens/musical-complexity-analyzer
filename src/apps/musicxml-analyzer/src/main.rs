use std::env;
use std::process;

use musicxml_analysis::analysis::calculate_density_metrics;
use musicxml_analysis::analysis::calculate_diversity_metrics;
use musicxml_analysis::extraction::musicxml::extract_measure_data;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <musicxml-file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    match musicxml::read_score_partwise(file_path) {
        Ok(score) => {
            let measure_data = extract_measure_data(&score);
            let density_metrics = calculate_density_metrics(&measure_data);
            let diversity_metrics = calculate_diversity_metrics(&measure_data);

            println!("=== Note Density ===");
            println!(
                "Average: {:>5.2} notes/second",
                density_metrics.average_notes_per_second
            );
            println!(
                "Peak   : {:>5.2} notes/second @ measure {}",
                density_metrics.peak_notes_per_second, density_metrics.peak_measure
            );

            println!("\n=== Pitch Diversity ===");
            println!("Unique pitches: {}", diversity_metrics.total_unique_pitches);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}
