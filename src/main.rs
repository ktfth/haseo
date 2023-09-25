use std::env;
use std::fs;
use std::path::PathBuf;

use haseo::lib::compare;

fn main() -> std::io::Result<()> {
    let left_file_path = env::args().nth(1).expect("Left file is required");
    let right_file_path = env::args().nth(2).expect("Right file is required");

    let is_pure: bool = env::args().len() > 3 && env::args().nth(3).unwrap() == "--pure".to_owned();

    let cwd = env::current_dir()?;

    let complete_l_f_path: PathBuf = [cwd.to_str().unwrap(), left_file_path.as_str()].iter().collect();
    let complete_r_f_path: PathBuf = [cwd.to_str().unwrap(), right_file_path.as_str()].iter().collect();

    let left_file_contents = fs::read_to_string(complete_l_f_path).unwrap();
    let right_file_contents = fs::read_to_string(complete_r_f_path).unwrap();

    for line in compare(left_file_contents, right_file_contents, is_pure) {
        println!("{}", line);
    }

    Ok(())
}
