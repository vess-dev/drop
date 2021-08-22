extern crate num_cpus;
#[macro_use]
extern crate prettytable;

use num_format::{Locale, ToFormattedString};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::SeedableRng;
use std::env;
use std::num::{ParseFloatError, ParseIntError};
use std::process::exit;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Print infomation about the program.
fn help() {
    println!("usage:

drop chance: f32 = (0.0, 100.0)
chests: u32 = (0, 4,294,967,295]
trials: u32 = (0, 4,294,967,295]

drop <drop chance> <chests> <trials>
	Simulate the chance out of N trials to get at least 1 item from N chests if the item has N drop chance.

Example usage:
	drop 4.55 20 1,000,000");
}

// Collect the args and verify them winthin limits.
fn verify() -> (f32, u32, u32) {
    let sys_args: Vec<String> = env::args().collect();
    match sys_args.len() {
        1 => {
            help();
            exit(1);
        }
        4 => {
            // Parse the arguments to ints.
            let test_drop: Result<f32, ParseFloatError> = sys_args[1].parse::<f32>();
            let test_chests: Result<u32, ParseIntError> =
                sys_args[2].replace(",", "").parse::<u32>();
            let test_trials: Result<u32, ParseIntError> =
                sys_args[3].replace(",", "").parse::<u32>();
            // Check that the drop chance arg is within the bounds.
            if let Err(_e) = test_drop {
                println!("Error: drop chance: f32 = (0.0, 100.0)\n");
                help();
                exit(1);
            } else if let Ok(value) = test_drop {
                if value >= 100.0 {
                    println!("Error: drop chance >= 100.0\n");
                    help();
                    exit(1);
                }
            }
            // Check that the chests arg is within the bounds.
            if let Err(_e) = test_chests {
                println!("Error: chests: u32 = (0, 4,294,967,295]\n");
                help();
                exit(1);
            } else if let Ok(value) = test_chests {
                if value == 0 {
                    println!("Error: chest = 0\n");
                    help();
                    exit(1);
                }
            }
            // Check that the trials arg is within the bounds.
            if let Err(_e) = test_trials {
                println!("Error: trials: u32 = (0, 4,294,967,295]\n");
                help();
                exit(1);
            } else if let Ok(value) = test_trials {
                if value == 0 {
                    println!("Error: trials = 0\n");
                    help();
                    exit(1);
                }
            }
            // Return if all are correct.
            (
                test_drop.unwrap(),
                test_chests.unwrap(),
                test_trials.unwrap(),
            )
        }
        _ => {
            println!("Incorrect number of arguments.\n");
            help();
            exit(1);
        }
    }
}

// Split a number into a vector of near-equal sized parts.
fn makesplit(arg_num: u32, arg_div: u32) -> Vec<u32> {
    let vec_capacity: usize = (arg_num as f32 / arg_div as f32).ceil() as usize;
    let mut vec_count: Vec<u32> = Vec::with_capacity(vec_capacity);
    for temp_cut in 0..arg_div {
        if temp_cut < (arg_num % arg_div) {
            vec_count.push((arg_num / arg_div) + 1);
        } else {
            vec_count.push(arg_num / arg_div);
        }
    }
    vec_count
}

fn main() {
    let time_start: Instant = Instant::now();
    let (arg_drop, arg_chests, arg_trials): (f32, u32, u32) = verify();
    // Create a weighted choice chest based on the chance.
    let weight_drop: u32 = (arg_drop * 100.0) as u32;
    let weight_other: u32 = 10_000 - weight_drop;
    let drop_choice: &[bool; 2] = &[true, false];
    let drop_weight: [u32; 2] = [weight_drop, weight_other];
    // Setup the RNG for the chests. (thread safe)
    let time_rng: Duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed.");
    // Batch out the threads appropriately based on CPU cores.
    let cpu_count: u32 = num_cpus::get() as u32;
    // Create a thread per trial batch, and add to a vector pool.
    let mut vec_thread: Vec<JoinHandle<u32>> = Vec::with_capacity(cpu_count as usize);
    let vec_split: Vec<u32> = makesplit(arg_trials, cpu_count);
    for temp_split in vec_split {
        // Thread and split the number of trials appropriately.
        let drop_chest: WeightedIndex<u32> = WeightedIndex::new(&drop_weight).unwrap();
        let mut sys_rng: StdRng = rand::rngs::StdRng::seed_from_u64(time_rng.as_secs());
        let trial_thread: JoinHandle<u32> = thread::spawn(move || {
            let mut trial_success: u32 = 0;
            for _trial in 0..temp_split {
                for _chest in 0..arg_chests {
                    if drop_choice[drop_chest.sample(&mut sys_rng)] {
                        trial_success += 1;
                        break;
                    }
                }
            }
            trial_success
        });
        vec_thread.push(trial_thread);
    }
    // Tabulate the results from each thread.
    let mut vec_success: u32 = 0;
    for temp_thread in vec_thread {
        vec_success += temp_thread.join().unwrap();
    }
    // Print out all of the stats in a pretty table.
    let trial_perc: f32 = (100.0 / arg_trials as f32) * (vec_success as f32);
    let trial_num: String = arg_trials.to_formatted_string(&Locale::en);
    let table_data = table!(["Chance to drop", r->format!("{}%", arg_drop.to_string())],
				["Number of chests", r->arg_chests.to_string()],
                       		["Number of trials", r->trial_num],
				["Calculated chance", r->format!("{:.2}%", trial_perc)],
				["Elapsed time (sec)", r->time_start.elapsed().as_secs_f64().to_string()]);
    table_data.printstd();
}
