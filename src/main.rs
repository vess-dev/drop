extern crate num_cpus;

use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand::SeedableRng;
use std::env;
use std::num::{ParseIntError, ParseFloatError};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn help() -> () {
	println!("usage:

drop chance: f32 = (0.0, 100.0)
chests: u32 = (0, max]
trials: u32 = (0, max]

drop <drop chance> <chests> <trials>
	Check the total chance out of N trials to receive 1 item out of N chests if the item has N drop chance.

Example usage:
	drop 4.55 20 1,000,000");
}

// Collect the args and verify them.
fn verify() -> (f32, u32, u32) {
	let sys_args: Vec<String> = env::args().collect();
	match sys_args.len() {
		1 => {
			help();
			exit(1);
		}
		4 => {
			let test_drop: Result<f32, ParseFloatError> = sys_args[1].replace(",", "").parse::<f32>();
			let test_chests: Result<u32, ParseIntError> = sys_args[2].replace(",", "").parse::<u32>();
			let test_trials: Result<u32, ParseIntError> = sys_args[3].replace(",", "").parse::<u32>();
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
			if let Err(_e) = test_chests {
				println!("Error: chests: u32 = (0, max]\n");
				help();
				exit(1);
			} else if let Ok(value) = test_chests {
				if value == 0 {
					println!("Error: chest = 0\n");
					help();
					exit(1);
				}
			}
			if let Err(_e) = test_trials {
				println!("Error: trials: u32 = (0, max]\n");
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
			return (test_drop.unwrap(), test_chests.unwrap(), test_trials.unwrap());
		}
		_ => {
			println!("Incorrect number of arguments.\n");
			help();
			exit(1);
		}
	}
}

// Split a number into a batch of near-equal sized numbers.
fn makesplit(arg_num: u32, arg_div: u32) -> Vec<u32> {
	let vec_capacity: usize = (arg_num as f32/ arg_div as f32).ceil() as usize;
	let mut vec_count: Vec<u32> = Vec::with_capacity(vec_capacity);
	for temp_cut in 0..arg_div {
		if temp_cut < (arg_num % arg_div) {
			vec_count.push((arg_num / arg_div) + 1);
		} else {
			vec_count.push(arg_num / arg_div);
		}
	}
	return vec_count;
}

fn main() {
	let (arg_drop, arg_chests, arg_trials): (f32, u32, u32) = verify();
	// Create a weighted choice crate.
	let weight_drop: u32 = (arg_drop * 100.0) as u32;
	let weight_other: u32 = 10_000 - weight_drop;
	let drop_choice: [bool; 2] = [true, false];
	let drop_weight: [u32; 2] = [weight_drop, weight_other];
	let arc_chest: Arc<WeightedIndex<u32>> = Arc::new(WeightedIndex::new(&drop_weight).unwrap());
	// Setup the RNG (thread safe).
	let sys_time: Duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Duration since UNIX_EPOCH failed.");
	let sys_rng: StdRng = rand::rngs::StdRng::seed_from_u64(sys_time.as_secs());
	let arc_rng: Arc<Mutex<StdRng>> = Arc::new(Mutex::new(sys_rng));
	// Batch out the threads appropriately.
	let cpu_count: u32 = num_cpus::get() as u32;
	// Create a thread per trial, Arc and Mutex for sharing.
	let mut vec_thread: Vec<JoinHandle<()>> = Vec::with_capacity(cpu_count as usize);
	let vec_split: Vec<u32> = makesplit(arg_trials, cpu_count);
	let arc_success: Arc<Mutex<u32>> = Arc::new(Mutex::new(0 as u32));
	for temp_split in vec_split {
        // Clone the references because they are consumed.
        let copy_success: Arc<Mutex<u32>> = Arc::clone(&arc_success);
		let copy_chest: Arc<WeightedIndex<u32>> = Arc::clone(&arc_chest);
		let copy_rng: Arc<Mutex<StdRng>> = Arc::clone(&arc_rng);
		// Thread and split the number of trials appropriately.
        let trial_thread: JoinHandle<()> = thread::spawn(move || {
			for _trial in 0..temp_split {
				for _chest in 0..arg_chests {
					if drop_choice[copy_chest.sample(&mut *copy_rng.lock().unwrap())] {
						*copy_success.lock().unwrap() += 1;
						break;
					}
				}
			}
        });
        vec_thread.push(trial_thread);
    }
    for temp_thread in vec_thread {
        temp_thread.join().expect("Thread panicked.");
    }
	// Print out the stats.
	let trial_perc: f32 = (100.0 / arg_trials as f32) * (*arc_success.lock().unwrap() as f32);
	println!("Out of [{}] trials an item with a drop chance of [{}%] was dropped from [{}] chests [{:.2}%] of the time.", arg_trials, arg_drop, arg_chests, trial_perc);
}