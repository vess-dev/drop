use rand::distributions::{Distribution, Weighted, WeightedChoice};
use std::env;
use std::process::exit;
use rand::SeedableRng;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn help() {
	println!("usage:

drop chance: f32 = (0.0, 100.0)
chests: u32 = (0, max]
trials: u32 = (0, max]

drop <drop chance> <chests> <trials>
	Check the total chance out of N trials to receive 1 item out of N chests if the item has N drop chance.

Example usage:
	drop 4.55 20 10,000,000");
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
			let test_drop = sys_args[1].parse::<f32>();
			let test_chests = sys_args[2].parse::<u32>();
			let test_trials = sys_args[3].replace(",", "").parse::<u32>();
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

// Simulate one drop of a chest.
fn drop(arg_drop: u32, arg_other: u32) -> bool {
	let mut drop_weight = vec![
				Weighted {
					weight: arg_drop,
					item: true,
				},
				Weighted {
					weight: arg_other,
					item: false,
				},
			];
	let sys_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Duration since UNIX_EPOCH failed.");
	let mut sys_rng = rand::rngs::StdRng::seed_from_u64(sys_time.as_secs());
	let chest_drop = WeightedChoice::new(&mut drop_weight);
	return chest_drop.sample(&mut sys_rng);
}

fn main() {
	let (arg_drop, arg_chests, arg_trials) = verify();
	// Create a weighted choice.
	let weight_drop = (arg_drop * 100.0) as u32;
	let weight_other = 10_000 - weight_drop;
	// Create a thread per trial.
	let mut trials_success: u64 = 0;
	let mut trials_threads = Vec::new();
	for _trial in 0..arg_trials {
		let trial_thread = thread::spawn(move || {
			for _chest in 0..arg_chests {
				continue;
			}
		});
		trials_threads.push(trial_thread);
	}
	for temp_thread in trials_threads {
		temp_thread.join().expect("Thread panicked.");
	}
	let trials_perc: f64 = (100.0 / arg_trials as f64) * trials_success as f64;
	println!("Out of [{}] trials an item with a drop chance of [{}%] was dropped from [{}] chests [{:.2}%] of the time.", arg_trials, weight_drop as f64 / 100.0, arg_chests, trials_perc);
}