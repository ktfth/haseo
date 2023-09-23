use std::env;
use std::fs;
use std::cmp;
use std::path::PathBuf;

use colorize::*;

fn main() -> std::io::Result<()> {
    let left_file_path = env::args().nth(1).expect("Left file is required");
    let right_file_path = env::args().nth(2).expect("Right file is required");

    let is_pure: bool = env::args().len() > 3 && env::args().nth(3).unwrap() == "--pure".to_owned();

    let cwd = env::current_dir()?;

    let complete_l_f_path: PathBuf = [cwd.to_str().unwrap(), left_file_path.as_str()].iter().collect();
    let complete_r_f_path: PathBuf = [cwd.to_str().unwrap(), right_file_path.as_str()].iter().collect();

    let left_file_contents = fs::read_to_string(complete_l_f_path).unwrap();
    let right_file_contents = fs::read_to_string(complete_r_f_path).unwrap();

    let splitted_left_file = left_file_contents.split("\n");
    let splitted_right_file = right_file_contents.split("\n");

    let left_lines = splitted_left_file.collect::<Vec<&str>>();
    let right_lines = splitted_right_file.collect::<Vec<&str>>();

    let max_len_between_files = cmp::max(left_lines.len(), right_lines.len());

    let mut left_compared_lines: Vec<String> = Vec::new();
    let mut right_compared_lines: Vec<String> = Vec::new();

    let mut combo = 0;

    for i in 0..max_len_between_files {
        let mut left_line = "";
        let mut right_line = "";

        if i < left_lines.len() {
            left_line = left_lines[i];
        }

        if i < right_lines.len() {
            right_line = right_lines[i];
        }

        let mut plus_sign = "+".green();
        let mut minus_sign = "-".red();

        if is_pure {
            plus_sign = "+".to_owned();
            minus_sign = "-".to_owned();
        }


        if left_line == right_line {
            if combo > 0 {
                for line in &left_compared_lines {
                    println!("{}", line);
                }

                for line in &right_compared_lines {
                    println!("{}", line);
                }

                left_compared_lines.clear();
                right_compared_lines.clear();

                combo = 0;
            }
            println!("{}", left_line.to_owned().to_string());
        } else if !left_line.is_empty() && !right_line.is_empty() && left_line != right_line {
            if !is_pure {
                left_compared_lines.push(format!("{}{}", minus_sign, left_line.to_owned().red()));
                right_compared_lines.push(format!("{}{}", plus_sign, right_line.to_owned().green()));
            } else {
                left_compared_lines.push(format!("{}{}", minus_sign, left_line));
                right_compared_lines.push(format!("{}{}", plus_sign, right_line));
            }
            combo += 1;
        } else if !left_line.is_empty() {
            if !is_pure {
                left_compared_lines.push(format!("{}{}", minus_sign, left_line.to_owned().red()));
            } else {
                left_compared_lines.push(format!("{}{}", minus_sign, left_line));
            }
            combo += 1;
        } else if !right_line.is_empty() {
            if !is_pure {
                right_compared_lines.push(format!("{}{}", plus_sign, right_line.to_owned().green()));
            } else {
                right_compared_lines.push(format!("{}{}", plus_sign, right_line));
            }
            combo += 1;
        }

        if i == max_len_between_files - 1 && combo > 0 {
            for line in &left_compared_lines {
                println!("{}", line);
            }

            for line in &right_compared_lines {
                println!("{}", line);
            }

            left_compared_lines.clear();
            right_compared_lines.clear();

            combo = 0;
        }
    }

    Ok(())
}
