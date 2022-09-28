use chrono::{Local, Timelike, DateTime};
use ansi_term::Color::*;
use std::env;

fn create_matrix(time: &String) -> [Vec<[char;3]>;3] {
    let mut matrix: [Vec<[char;3]>; 3] = [
        vec![],
        vec![],
        vec![]
    ];
    for i in time.chars() {
        match i {
            '0' => {
                matrix[0].push(['┌', '─', '┐']);
                matrix[1].push(['│', ' ', '│']);
                matrix[2].push(['└', '─', '┘']);
            }
            '1' => {
                matrix[0].push([' ', '┐', ' ']);
                matrix[1].push([' ', '│', ' ']);
                matrix[2].push([' ', '┴', ' ']);
            }
            '2' => {
                matrix[0].push(['╶', '─', '┐']);
                matrix[1].push(['┌', '─', '┘']);
                matrix[2].push(['└', '─', '╴']);
            }
            '3' => {
                matrix[0].push(['╶', '─', '┐']);
                matrix[1].push(['╶', '─', '┤']);
                matrix[2].push(['╶', '─', '┘']);
            }
            '4' => {
                matrix[0].push(['╷', ' ', '╷']);
                matrix[1].push(['└', '─', '┤']);
                matrix[2].push([' ', ' ', '╵']);
            }
            '5' => {
                matrix[0].push(['┌', '─', '╴']);
                matrix[1].push(['└', '─', '┐']);
                matrix[2].push(['╶', '─', '┘']);
            }
            '6' => {
                matrix[0].push(['┌', '─', '╴']);
                matrix[1].push(['├', '─', '┐']);
                matrix[2].push(['└', '─', '┘']);
            }
            '7' => {
                matrix[0].push(['╶', '─', '┐']);
                matrix[1].push([' ', ' ', '│']);
                matrix[2].push([' ', ' ', '╵']);
            }
            '8' => {
                matrix[0].push(['┌', '─', '┐']);
                matrix[1].push(['├', '─', '┤']);
                matrix[2].push(['└', '─', '┘']);
            }
            '9' => {
                matrix[0].push(['┌', '─', '┐']);
                matrix[1].push(['└', '─', '┤']);
                matrix[2].push([' ', ' ', '╵']);
            }
            _ => {}
        }
    }
    return matrix;
}

fn render(
    matrix: &[Vec<[char;3]>;3], 
    local: &DateTime<Local>,
    is_twelve_fmt: bool,
    is_pm: bool) 
{
    println!();
    for i in 0..3 {
        for j in 0..6 {
            for k in 0..3{
                print!("{}", Blue.bold().paint(matrix[i][j][k].to_string()));
            }           
            if [1, 3].contains(&j) {
                if i != 2 {
                    print!("{}", Blue.bold().paint("."));
                } else {
                    print!(" ");
                }
            }   
        }
        if i == 0 {
            print!("{}", Green.paint(local.format("%a %d").to_string()));
        } else if i == 2 && is_twelve_fmt {
            let meridiem = if is_pm { "PM" } else { "AM" };
            print!("{}", Red.paint(meridiem));
        }
        println!();
    }
    println!();
}

fn main() {
    let mut is_twelve = false;   
    let local = Local::now();
    let mut time = format!("{}", local.format("%H%M%S"));
    let (is_pm, hour) = local.hour12();

    let args: Vec<String> = env::args().collect();
    for i in args.iter() {
        if i == "-b" {
            is_twelve = true;
            let mut new_hour = hour.to_string();
            if hour < 10 {
                new_hour = format!("0{}", hour);
            }
             
            time = format!("{}{}", new_hour, local.format("%M%S"));
        }
    }

    let matrix = create_matrix(&time);
    render(&matrix, &local, is_twelve, is_pm);
}