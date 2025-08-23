use musicxml_analysis::analysis::calculate_density_metrics;
use musicxml_analysis::analysis::calculate_diversity_metrics;
use musicxml_analysis::extraction::musicxml::extract_measure_data;
use musicxml_analysis::statistics::correlation::calculate_pearson_correlation;
use plotly::{
    Layout, Plot, Scatter,
    common::{Mode, Title},
};
use plotters::prelude::*;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
struct PieceData {
    name: String,
    avg_density: f64,
    peak_density: f64,
    pitch_diversity: u32,
    key_diversity: u32,
    total_note_count: u32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} [--output-dir <dir>] <path to musicxml file or directory>",
            args[0]
        );
        process::exit(1);
    }

    // Parse arguments
    let mut output_dir = ".".to_string();
    let mut input_path = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output-dir" => {
                if i + 1 >= args.len() {
                    eprintln!("--output-dir requires a directory path");
                    process::exit(1);
                }
                output_dir = args[i + 1].clone();
                i += 2;
            }
            _ => {
                input_path = Some(args[i].clone());
                i += 1;
            }
        }
    }

    let input_path = input_path.unwrap_or_else(|| {
        eprintln!("No input path provided");
        process::exit(1);
    });

    let path = Path::new(&input_path);
    let piece_data = if path.is_file() {
        vec![analyze_file(&input_path)]
    } else if path.is_dir() {
        analyze_directory(&input_path)
    } else {
        eprintln!("Error: '{input_path}' is not a valid file or directory");
        process::exit(1);
    };

    for piece in &piece_data {
        print_piece_results(piece);
        println!();
    }

    if piece_data.len() > 1 {
        println!("Generating charts...");

        let density_base = format!("{output_dir}/note_density_histogram");
        if let Err(e) = generate_note_density_histogram(&piece_data, &density_base) {
            eprintln!("Failed to generate note density chart: {e}");
        }

        let diversity_base = format!("{output_dir}/pitch_diversity_histogram");
        if let Err(e) = generate_pitch_diversity_histogram(&piece_data, &diversity_base) {
            eprintln!("Failed to generate pitch diversity chart: {e}");
        }

        let correlation_base = format!("{output_dir}/note_density_pitch_diversity_correlation");
        if let Err(e) =
            generate_note_density_pitch_diversity_correlation_chart(&piece_data, &correlation_base)
        {
            eprintln!("Failed to generate correlation chart: {e}");
        }

        let note_count_density_base = format!("{output_dir}/note_count_note_density_correlation");
        if let Err(e) = generate_note_count_note_density_correlation_chart(
            &piece_data,
            &note_count_density_base,
        ) {
            eprintln!("Failed to generate note count vs density chart: {e}");
        }

        let note_count_diversity_base =
            format!("{output_dir}/note_count_pitch_diversity_correlation");
        if let Err(e) = generate_note_count_pitch_diversity_correlation_chart(
            &piece_data,
            &note_count_diversity_base,
        ) {
            eprintln!("Failed to generate note count vs diversity chart: {e}");
        }

        let pitch_key_base = format!("{output_dir}/pitch_diversity_key_diversity_correlation");
        if let Err(e) =
            generate_pitch_diversity_key_diversity_correlation_chart(&piece_data, &pitch_key_base)
        {
            eprintln!("Failed to generate pitch vs key diversity chart: {e}");
        }
    } else {
        println!("Skipping chart generation (need multiple pieces for meaningful charts)");
    }
}

fn analyze_file(file_path: &str) -> PieceData {
    analyze_single_file(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to analyze '{file_path}': {e}");
        process::exit(1);
    })
}

fn analyze_directory(dir_path: &str) -> Vec<PieceData> {
    let files = find_musicxml_files(dir_path).unwrap_or_else(|e| {
        eprintln!("Error reading directory '{dir_path}': {e}");
        process::exit(1);
    });

    if files.is_empty() {
        println!("No MusicXML files found in directory '{dir_path}'");
        return Vec::new();
    }

    println!("Found {} MusicXML files in '{dir_path}':\n", files.len());

    let mut piece_data = Vec::new();

    for file_path in files {
        match analyze_single_file(&file_path) {
            Ok(data) => piece_data.push(data),
            Err(e) => eprintln!("Failed to analyze '{file_path}': {e}"),
        }
    }

    piece_data
}

fn analyze_single_file(file_path: &str) -> Result<PieceData, String> {
    println!("Analyzing: {file_path}");

    let score =
        musicxml::read_score_partwise(file_path).map_err(|e| format!("Parse error: {e}"))?;

    let measure_data = extract_measure_data(&score);
    let density = calculate_density_metrics(&measure_data);
    let diversity = calculate_diversity_metrics(&measure_data);

    let name = Path::new(file_path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    Ok(PieceData {
        name,
        avg_density: density.average_notes_per_second,
        peak_density: density.peak_notes_per_second,
        pitch_diversity: diversity.total_unique_pitches,
        key_diversity: diversity.total_unique_keys,
        total_note_count: density.total_note_count,
    })
}

fn find_musicxml_files(dir_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            if matches!(ext.as_str(), "musicxml" | "mxl")
                && let Some(path_str) = path.to_str()
            {
                files.push(path_str.to_string());
            }
        }
    }

    files.sort();
    Ok(files)
}

fn print_piece_results(piece: &PieceData) {
    println!("=== {} ===", piece.name);
    println!("Note Density:");
    println!("  Average: {:>5.2} notes/second", piece.avg_density);
    println!("  Peak   : {:>5.2} notes/second", piece.peak_density);
    println!("  # notes: {}", piece.total_note_count);
    println!("Pitch Diversity:");
    println!("  # unique pitches: {}", piece.pitch_diversity);
    println!("  # unique piano keys: {}", piece.key_diversity);
}

fn generate_note_density_histogram(
    data: &[PieceData],
    output_path_without_extension: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("{output_path_without_extension}.svg");
    let root = SVGBackend::new(&output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let densities: Vec<f64> = data.iter().map(|d| d.avg_density).collect();
    let max_density = densities.iter().fold(0.0f64, |a, &b| a.max(b));

    // Create 10 bins
    let bin_size = max_density / 10.0;
    let mut bins = [0; 10];

    for density in &densities {
        let bin_index = ((density / bin_size) as usize).min(9);
        bins[bin_index] += 1;
    }

    let max_count = *bins.iter().max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Note Density Distribution", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..max_density, 0..(max_count + 1))?;

    chart
        .configure_mesh()
        .x_desc("Average Note Density (notes/second)")
        .y_desc("Number of Pieces")
        .draw()?;

    chart.draw_series(bins.iter().enumerate().map(|(i, &count)| {
        let x0 = i as f64 * bin_size;
        let x1 = (i + 1) as f64 * bin_size;
        Rectangle::new([(x0, 0), (x1, count)], BLUE.filled())
    }))?;

    root.present()?;
    Ok(())
}

fn generate_pitch_diversity_histogram(
    data: &[PieceData],
    output_path_without_extension: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("{output_path_without_extension}.svg");
    let root = SVGBackend::new(&output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let diversities: Vec<u32> = data.iter().map(|d| d.pitch_diversity).collect();
    let max_diversity = *diversities.iter().max().unwrap();

    // Create 10 bins
    let bin_size = max_diversity.div_ceil(10);
    let mut bins = [0; 10];

    for diversity in &diversities {
        let bin_index = ((diversity / bin_size) as usize).min(9);
        bins[bin_index] += 1;
    }

    let max_count = *bins.iter().max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Pitch Diversity Distribution", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0..max_diversity, 0..(max_count + 1))?;

    chart
        .configure_mesh()
        .x_desc("Unique Pitches")
        .y_desc("Number of Pieces")
        .draw()?;

    chart.draw_series(bins.iter().enumerate().map(|(i, &count)| {
        let x0 = i as u32 * bin_size;
        let x1 = (i + 1) as u32 * bin_size;
        Rectangle::new([(x0, 0), (x1, count)], RED.filled())
    }))?;

    root.present()?;
    Ok(())
}

fn generate_note_density_pitch_diversity_correlation_chart(
    data: &[PieceData],
    output_path_without_extension: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let x_values: Vec<f64> = data.iter().map(|d| d.avg_density).collect();
    let y_values: Vec<f64> = data.iter().map(|d| d.pitch_diversity as f64).collect();
    let correlation = calculate_pearson_correlation(&x_values, &y_values);
    println!(
        "Note Density vs Pitch Diversity correlation: r = {:.3}",
        correlation
    );

    generate_scatter_plot(
        data,
        output_path_without_extension,
        "Note Density vs Pitch Diversity",
        "Average Note Density (notes/second)",
        "Unique Pitches",
        |piece| piece.avg_density,
        |piece| piece.pitch_diversity as f64,
        BLUE,
    )
}

fn generate_note_count_note_density_correlation_chart(
    data: &[PieceData],
    output_path_without_extension: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let x_values: Vec<f64> = data.iter().map(|d| d.total_note_count as f64).collect();
    let y_values: Vec<f64> = data.iter().map(|d| d.avg_density).collect();
    let correlation = calculate_pearson_correlation(&x_values, &y_values);
    println!(
        "Note Count vs Note Density correlation: r = {:.3}",
        correlation
    );

    generate_scatter_plot(
        data,
        output_path_without_extension,
        "Note Count vs Avg Note Density",
        "Total Note Count",
        "Average Note Density (notes/second)",
        |piece| piece.total_note_count as f64,
        |piece| piece.avg_density,
        GREEN,
    )
}

fn generate_note_count_pitch_diversity_correlation_chart(
    data: &[PieceData],
    output_path_without_extension: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let x_values: Vec<f64> = data.iter().map(|d| d.total_note_count as f64).collect();
    let y_values: Vec<f64> = data.iter().map(|d| d.pitch_diversity as f64).collect();
    let correlation = calculate_pearson_correlation(&x_values, &y_values);
    println!(
        "Note Count vs Pitch Diversity correlation: r = {:.3}",
        correlation
    );

    generate_scatter_plot(
        data,
        output_path_without_extension,
        "Note Count vs Pitch Diversity",
        "Total Note Count",
        "Unique Pitches",
        |piece| piece.total_note_count as f64,
        |piece| piece.pitch_diversity as f64,
        RED,
    )
}

fn generate_pitch_diversity_key_diversity_correlation_chart(
    data: &[PieceData],
    output_path_without_extension: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let x_values: Vec<f64> = data.iter().map(|d| d.pitch_diversity as f64).collect();
    let y_values: Vec<f64> = data.iter().map(|d| d.key_diversity as f64).collect();
    let correlation = calculate_pearson_correlation(&x_values, &y_values);

    println!(
        "Pitch Diversity vs Key Diversity correlation: r = {:.3}",
        correlation
    );

    generate_scatter_plot(
        data,
        output_path_without_extension,
        "Pitch Diversity vs Key Diversity",
        "Unique Pitches",
        "Unique Piano Keys",
        |piece| piece.pitch_diversity as f64,
        |piece| piece.key_diversity as f64,
        MAGENTA,
    )
}

#[allow(clippy::too_many_arguments)]
fn generate_scatter_plot<F, G>(
    data: &[PieceData],
    output_path_without_extension: &str,
    title: &str,
    x_label: &str,
    y_label: &str,
    x_extractor: F,
    y_extractor: G,
    _color: RGBColor,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&PieceData) -> f64,
    G: Fn(&PieceData) -> f64,
{
    let x_values: Vec<f64> = data.iter().map(&x_extractor).collect();
    let y_values: Vec<f64> = data.iter().map(&y_extractor).collect();
    let names: Vec<String> = data.iter().map(|d| d.name.clone()).collect();

    // Calculate correlation for title
    let correlation = calculate_pearson_correlation(&x_values, &y_values);
    let full_title = format!("{} (r = {:.3})", title, correlation);

    let trace = Scatter::new(x_values, y_values)
        .name("Musical Pieces")
        .text_array(names)
        .mode(Mode::Markers)
        .marker(plotly::common::Marker::new().size(8));

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title(Title::from(&full_title))
        .x_axis(plotly::layout::Axis::new().title(Title::from(x_label)))
        .y_axis(plotly::layout::Axis::new().title(Title::from(y_label)))
        .height(800)
        .auto_size(true);

    plot.set_layout(layout);

    let html_path = format!("{}.html", output_path_without_extension);
    let html_content = plot.to_html();
    std::fs::write(&html_path, html_content)?;

    println!("Interactive chart saved to: {}", html_path);
    Ok(())
}
