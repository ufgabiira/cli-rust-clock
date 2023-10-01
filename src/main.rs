use ansi_term::Color::*;
use chrono::{DateTime, Local, Timelike};
use std::env;

fn create_matrix(time: String) -> [String; 3] {
    let mut initial = [["#"; 8]; 3];

    for (i, num) in time.chars().enumerate() {
        match num {
            '0' => {
                initial[0][i] = "┌─┐";
                initial[1][i] = "│ │";
                initial[2][i] = "└─┘";
            }
            '1' => {
                initial[0][i] = " ┐ ";
                initial[1][i] = " │ ";
                initial[2][i] = " ┴ ";
            }
            '2' => {
                initial[0][i] = "╶─┐";
                initial[1][i] = "┌─┘";
                initial[2][i] = "└─╴";
            }
            '3' => {
                initial[0][i] = "╶─┐";
                initial[1][i] = "╶─┤";
                initial[2][i] = "╶─┘";
            }
            '4' => {
                initial[0][i] = "╷ ╷";
                initial[1][i] = "└─┤";
                initial[2][i] = "  ╵";
            }
            '5' => {
                initial[0][i] = "┌─╴";
                initial[1][i] = "└─┐";
                initial[2][i] = "╶─┘";
            }
            '6' => {
                initial[0][i] = "┌─╴";
                initial[1][i] = "├─┐";
                initial[2][i] = "└─┘";
            }
            '7' => {
                initial[0][i] = "╶─┐";
                initial[1][i] = "  │";
                initial[2][i] = "  ╵";
            }
            '8' => {
                initial[0][i] = "┌─┐";
                initial[1][i] = "├─┤";
                initial[2][i] = "└─┘";
            }
            '9' => {
                initial[0][i] = "┌─┐";
                initial[1][i] = "└─┤";
                initial[2][i] = "  ╵";
            }
            _ => {
                initial[0][i] = ".";
                initial[1][i] = ".";
                initial[2][i] = " ";
            }
        }
    }

    let mut matrix: [String; 3] = [String::new(), String::new(), String::new()];

    for i in 0..3 {
        matrix[i] = initial[i].join("");
    }

    matrix
}

fn render(matrix: [String; 3], local: DateTime<Local>, is_pm: bool, us_format: bool) {
    println!();

    for (i, line) in matrix.iter().enumerate() {
        print!("{}", Blue.bold().paint(line));

        if i == 0 {
            println!("{}", Green.paint(local.format("%a %b").to_string()));
        } else if i == 2 && us_format {
            if is_pm {
                println!("{}", Red.bold().paint("pm"));
            } else {
                println!("{}", Red.bold().paint("am"));
            }
        } else {
            println!();
        }
    }
}

fn main() {
    let local = Local::now();
    let (is_pm, hour) = local.hour12();

    let mut time = format!("{}", local.format("%H:%M:%S"));
    let mut us_format = false;

    let args = env::args();

    for arg in args {
        if arg == "-b" {
            us_format = true;

            if hour < 10 {
                time = format!("0{}:{}", hour, local.format("%M:%S"));
            } else {
                time = format!("{}:{}", hour, local.format("%M:%S"));
            }
        }
    }

    let matrix = create_matrix(time);
    render(matrix, local, is_pm, us_format);
}
