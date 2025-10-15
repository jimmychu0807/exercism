use std::{collections::HashMap, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
	let (tx, rx) = mpsc::channel::<HashMap<char, usize>>();
	let mut ttl_count = HashMap::new();

	if input.is_empty() {
		return ttl_count;
	}

	if input.len() <= worker_count {
		for i in input {
			let tx1 = tx.clone();
			let s = i.to_string();

			thread::spawn(move || {
				let subcount = count_chr(&[s]);
				tx1.send(subcount).unwrap();
			});
		}
	} else {
		let boundary = input.len() / worker_count;
		for i in 0..worker_count {
			let tx1 = tx.clone();
			let sgrp: Vec<String> = input[(i * boundary)..(if i == worker_count - 1 {
				input.len()
			} else {
				(i + 1) * boundary
			})]
				.iter()
				.map(|str| (*str).into())
				.collect();

			thread::spawn(move || {
				let subcount = count_chr(&sgrp);
				tx1.send(subcount).unwrap();
			});
		}
	}

	drop(tx);

	for mut subcount in rx {
		for (k, v) in subcount.drain() {
			ttl_count.entry(k).and_modify(|acc| *acc += v).or_insert(v);
		}
	}

	ttl_count
}

fn count_chr(words: &[String]) -> HashMap<char, usize> {
	let mut count = HashMap::new();

	for single in words {
		single.to_lowercase().chars().filter(|c| c.is_alphabetic()).for_each(|c| {
			count.entry(c).and_modify(|acc| *acc += 1).or_insert(1);
		});
	}

	count
}
