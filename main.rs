/*
 *  The 100 Prisoners Problem / Riddle
 *  https://en.wikipedia.org/wiki/100_prisoners_problem
 *  https://www.youtube.com/watch?v=9zfeTw-uFCw
 */

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};
use std::env;

const DEFAULT_RUNS: usize = 100000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut prisoners: [i32; 100] = [0; 100];
    let mut boxes: [i32; 100] = [0; 100];

    let mut success_count = 0;
    let mut fail_count = 0;
    let mut number_of_runs = DEFAULT_RUNS;

    if args.len() > 1 {
        let int_arg: usize = match args[1].parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid argument {}. Expected a number", args[1]);
                std::process::exit(1);
            }
        };

        if int_arg < 1 {
            println!("Invalid argument. Expected a number greater than 0");
            std::process::exit(1);
        }

        number_of_runs = int_arg;
    }

    cls();
    println!("100 Prisoners Problem / Riddle");
    println!("https://github.com/freshe");
    println!("");
    println!("Running {} times...", number_of_runs);

    for _i in 0..number_of_runs {
        let run_success = play(&mut prisoners, &mut boxes);

        if run_success {
            success_count += 1;
        } else {
            fail_count += 1;
        }
    }

    println!("Successful runs: {} / Failed runs: {}", success_count, fail_count);
}

fn play(prisoners: &mut [i32], boxes: &mut [i32]) -> bool {
    //init
    for i in 0..prisoners.len() {
        let number : i32 = i as i32;
        prisoners[i] = number + 1;
        boxes[i] = number + 1;
    }

    //arrange
    prisoners.shuffle(&mut thread_rng());
    boxes.shuffle(&mut thread_rng());

    for i in 0..prisoners.len() {
        let prisoner_number = prisoners[i];
        let mut box_index = prisoner_number as usize - 1;
        let mut count = 0;
        let mut failed = false;

        loop {
            if count >= 50 {
                failed = true;
                break;
            }

            count += 1;
            let next_number : i32 = boxes[box_index];

            if next_number == prisoner_number {
                break;
            }

            box_index = next_number as usize - 1;
        }

        if failed {
            return false;
        }
    }

    return true;
}

fn cls() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}