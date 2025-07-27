use musicxml::read_score_partwise;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <musicxml-file>", args[0]);
        std::process::exit(1);
    }

    match read_score_partwise(&args[1]) {
        Ok(score) => {
            println!("=== MusicXML DOM Structure ===");
            println!("{:#?}", score);
        }
        Err(e) => {
            eprintln!("Error parsing MusicXML: {}", e);
            std::process::exit(1);
        }
    }
}
