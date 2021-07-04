use rand::distributions::{Weighted, WeightedChoice, Distribution};
use std::env;
use std::process::exit;

fn help() {
	println!("usage:

drop chance: f64 = (0, 100)
chests: u64 = (0, max]
trials: u64 = (0, max]

drop <drop chance> <chests> <trials>
	Check the total chance out of N trials to recieve 1 item out of N chests if the item has N drop chance.

Example usage:
	drop 4.55 20 10000000");
}

fn main() {
	// Catch incorrect arguments.
	let sys_args: Vec<String> = env::args().collect();
	match sys_args.len() {
		1 => {
			help();
			exit(1);
		}
		4 => {
			let test_drop = sys_args[1].parse::<f64>();
			let test_chests = sys_args[2].parse::<u64>();
			let test_trials = sys_args[3].parse::<u64>();
			if let Err(_e) = test_drop {
				println!("Error: drop chance: f64 = (0, 100)\n");
				help();
				exit(1);
			} else {
				if let Ok(value) = test_drop {
					if value >= 100.0 {
						println!("Error: drop chance >= 100\n");
						help();
						exit(1);
					}
				}
			}
			if let Err(_e) = test_chests {
				println!("Error: chests: u64 = (0, max]\n");
				help();
				exit(1);
			} else {
				if let Ok(value) = test_chests {
					if value == 0 {
						println!("Error: chest = 0\n");
						help();
						exit(1);
					}
				}
			}
			if let Err(_e) = test_trials {
				println!("Error: trials: u64 = (0, max]\n");
				help();
				exit(1);
			} else {
				if let Ok(value) = test_trials {
					if value == 0 {
						println!("Error: trials = 0\n");
						help();
						exit(1);
					}
				}
			}
		}
		_ => {
			println!("Incorrect number of arguments.\n");
			help();
			exit(1);
		}
	}
	// Create a weighted choice.
	let weight_drop = (sys_args[1].parse::<f64>().unwrap() * 100.0) as u32;
	let weight_other = 10_000 - weight_drop;
	let mut drop_weight = vec!(Weighted{weight: weight_drop, item: true}, Weighted{weight: weight_other, item: false});
	let drop_chest  = WeightedChoice::new(&mut drop_weight);
	let mut sys_rng = rand::thread_rng();
	// Total trials where 1 item was pulled.
	let mut trials_success = 0;
	let trials_chests = sys_args[2].parse::<u64>().unwrap();
	let trials_total = sys_args[3].parse::<u64>().unwrap();
	for _trial in 0..trials_total {
		for _chest in 0..trials_chests {
			if drop_chest.sample(&mut sys_rng) {
				trials_success = trials_success + 1;
				break;
			}
		}
	}
	let trials_perc: f64 = (100.0/trials_total as f64) * trials_success as f64;
	println!("Out of {} trials, an item was pulled from {} chests: {:.2}% of the time.", trials_total, trials_chests, trials_perc);
}