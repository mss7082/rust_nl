use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

#[derive(Clone)]
enum PadMode {
    PadLeft,
    PadRight,
    // PadCenter,
}

type NumberedLine = (Option<u64>, String);

type NumberedLines = Vec<NumberedLine>;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn number_lines(
    should_incr: fn(String) -> bool,
    should_number: fn(String) -> bool,
    filename: &str,
) -> NumberedLines {
    let mut numbered_lines: NumberedLines = Vec::new();
    let mut counter: u64 = 1;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            if should_incr(line.clone()) {
                numbered_lines.push((Some(counter), line.to_string()));
                counter += 1;
            } else if should_number(line.clone()) {
                numbered_lines.push((Some(counter), line.to_string()));
            } else {
                numbered_lines.push((None, line.to_string()));
            }
        }
    }
    numbered_lines
}

fn number_all_lines(filename: &str) -> NumberedLines {
    number_lines(always_true, always_true, filename)
}

fn number_and_increment_non_empty_lines(filename: &str) -> NumberedLines {
    number_lines(is_not_empty, is_not_empty, filename)
}

fn is_not_empty(line: String) -> bool {
    if line.is_empty() {
        return false;
    }
    true
}

fn always_true(_: String) -> bool {
    true
}

fn pad(mode: PadMode, justify_by: usize, line: &str) -> String {
    let diff = justify_by - line.len();
    let padding = " ".repeat(diff);
    // let center_padding = " ".repeat(diff / 2);
    match mode {
        PadMode::PadLeft => format!("{}{}", padding, line),
        PadMode::PadRight => format!("{}{}", line, padding),
        // PadMode::PadCenter => format!("{}{}{}", center_padding, line, center_padding),
    }
}

fn pretty_numbered_lines(mode: &PadMode, num_lines: NumberedLines) -> Vec<String> {
    let (numbers, lines): (Vec<_>, Vec<_>) = num_lines.iter().cloned().unzip();

    let num_strings: Vec<String> = numbers
        .iter()
        .map(|n| match n {
            Some(line_number) => line_number.to_string(),
            None => "".to_string(),
        })
        .collect();

    let max_length = match lines.iter().map(|line| line.len()).max() {
        Some(x) => x,
        None => usize::MIN,
    };

    let padded_numbers: Vec<_> = num_strings
        .iter()
        .map(|l| pad(mode.clone(), max_length, l))
        .collect();

    padded_numbers
        .iter()
        .zip(lines)
        .map(|(n, l)| format!("{} {}", n, l))
        .collect()
}

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    filepath: String,
    #[arg(short, long)]
    reverse: bool,
    #[arg(short, long)]
    skip_empty: bool,
    #[arg(short, long)]
    left_align: bool,
}

fn main() {
    let cli = Cli::parse();

    let filepath = cli.filepath;

    let number_function = if cli.skip_empty {
        number_and_increment_non_empty_lines
    } else {
        number_all_lines
    };

    let pad_mode = if cli.left_align {
        PadMode::PadRight
    } else {
        PadMode::PadLeft
    };

    let numbered = number_function(&filepath);

    let pretty_numbered = pretty_numbered_lines(&pad_mode, numbered);

    let rev_numbered = number_function(&filepath);
    let reversed_pretty = pretty_numbered_lines(&pad_mode, rev_numbered);

    let results = if cli.reverse {
        reversed_pretty
    } else {
        pretty_numbered
    };

    for line in results {
        println!("{}", line);
    }
}
