use std::env;
use std::fs;
use std::path::Path;
use std::process;

use musicxml_analysis::analysis::calculate_density_metrics;
use musicxml_analysis::analysis::calculate_diversity_metrics;
use musicxml_analysis::extraction::musicxml::extract_measure_data;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path to musicxml file or directory>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let path = Path::new(input_path);

    if path.is_file() {
        analyze_single_file(input_path);
    } else if path.is_dir() {
        analyze_directory(input_path);
    } else {
        eprintln!("Error: '{input_path}' is not a valid file or directory");
        process::exit(1);
    }
}

fn analyze_single_file(file_path: &str) {
    println!("Analyzing: {file_path}...");
    match musicxml::read_score_partwise(file_path) {
        Ok(score) => {
            print_analysis_results(&score);
        }
        Err(e) => {
            eprintln!("Error analyzing '{file_path}': {e}");
            process::exit(1);
        }
    }
}

fn analyze_directory(dir_path: &str) {
    let musicxml_files = match find_musicxml_files(dir_path) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error reading directory '{dir_path}': {e}");
            process::exit(1);
        }
    };

    if musicxml_files.is_empty() {
        println!("No MusicXML files found in directory '{dir_path}'");
        return;
    }

    println!(
        "Found {} MusicXML files in '{dir_path}':\n",
        musicxml_files.len()
    );

    for file_path in musicxml_files {
        analyze_single_file(&file_path);
    }
}

fn find_musicxml_files(dir_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut musicxml_files = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if ext == "musicxml" || ext == "mxl" {
                    if let Some(path_str) = path.to_str() {
                        musicxml_files.push(path_str.to_string());
                    }
                }
            }
        }
    }

    musicxml_files.sort();
    Ok(musicxml_files)
}

fn print_analysis_results(score: &musicxml::elements::ScorePartwise) {
    let measure_data = extract_measure_data(score);
    let density_metrics = calculate_density_metrics(&measure_data);
    let diversity_metrics = calculate_diversity_metrics(&measure_data);

    println!("Note Density:");
    println!(
        "  Average: {:>5.2} notes/second",
        density_metrics.average_notes_per_second
    );
    println!(
        "  Peak   : {:>5.2} notes/second @ measure {}",
        density_metrics.peak_notes_per_second, density_metrics.peak_measure
    );
    println!("Pitch Diversity:");
    println!(
        "  Unique pitches: {}",
        diversity_metrics.total_unique_pitches
    );
    println!();
}
