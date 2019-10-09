use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use linecount::count_lines;
// #[macro_use]
// extern crate timeit;
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    // let path = Path::new("lab.csv");
    let path = Path::new("lab.csv");

    /*
    * @dev Count the number of lines.
    */

    // let mut no_lines = 0;
    // let (elapsed, sum) = measure_time(|| {

    //     no_lines = cnt_lines(&path);
    //     println!("Number of lines : {}", no_lines);
	// });
    // println!("Time Taken = {}", elapsed);

    /*
    * @name Question 1
    * @dev Get nth line
    */

    // let (elapsed2, sum2) = measure_time(|| {
    //     for i in 1..=20 {
    //         print!("({}) : ", i);
    //         let line = get_line_at(&path, i);
    //         println!("{}", line.unwrap());
    //     };
    //     for i in no_lines-20..=no_lines-1 {
    //         print!("({}) : ", i);
    //         let line = get_line_at(&path, i);
    //         println!("{}", line.unwrap());
    //     };
	// });
    // println!("Time Taken = {}", elapsed2);

    // let (elapsed3, sum3) = measure_time(|| {
    //     let st = "Hello";
    //     add_line(&st.to_string());
	// });

    // println!("Time Taken = {}", elapsed3);

    // let (elapsed4, sum4) = measure_time(|| {
    //     rep(&path);
	// });
    // println!("Time Taken = {}", elapsed4);

}

fn cnt_lines(path: &Path) -> usize {
    let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
    return lines;
}

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}

fn dup(path: &Path){
    let file = File::open(&path).unwrap();
    for line in BufReader::new(file).lines() {
        let st = line.unwrap();
        add_line(&st.to_string());
    }
}

fn rep(path: &Path){
    let file = File::open(&path).unwrap();
    let mut cnt = 0;
    for line in BufReader::new(file).lines() {
        let st = line.unwrap();
        let result = str::replace(&st, "line", "play");
        cnt+=1;
        add_line(&result.to_string());
    }
    println!("{} Occurrences Replaced", cnt);
}

fn add_line(line : &String){
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("foo2.txt")
        .unwrap();

    if let Err(e) = writeln!(file,"{}", line.to_string()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
