use linked_list::dodd::helpers;
use linked_list::dodd::item::Item;

use num::bigint::BigInt;
use once_cell::sync::OnceCell;
use regex::Regex;

use std::io;
use std::str::FromStr;

fn trim_newline(string: &mut String) {
	if string.ends_with('\n') {
		string.pop();
		if string.ends_with('\r') {
			string.pop();
		}
	}
}

fn is_valid_string(string: &str) -> bool {
	static VALID_REGEX: OnceCell<Regex> = OnceCell::new();
	let regex = VALID_REGEX.get_or_init(
		|| Regex::new("^(0|-?[1-9][0-9]*|[A-Za-z][0-9A-Z_a-z]*)$").unwrap()
	);
	regex.is_match(string)
}

fn insert_before(val: &String, oth: &Item<String>) -> bool {
	if let Ok(x) = BigInt::from_str(val) {
		if let Ok(y) = BigInt::from_str(&oth.value) {
			return x <= y
		}
	}
	*val <= oth.value
}

fn value_equals(item: &Item<String>, val: &String) -> bool {
	item.value == *val
}

fn main() {
	let mut start = None;
	
	let mut begin = true;
	let mut input;
	
	loop {
		if !begin {
			println!();
		}
		else {
			begin = false;
		}
		
		println!("Awaiting input...");
		let mut string = String::new();
		io::stdin().read_line(&mut string).expect("Failed to read line!");
		trim_newline(&mut string);
		
		if string.is_empty() {
			println!("\nProgram terminated!");
			helpers::remove_all(&mut start);
			return;
		}
		else if string.starts_with('~') {
			if string.len() == 1 {
				println!("\nDeleting list...");
				helpers::remove_all(&mut start);
			}
			else {
				input = &string[1..];
				if is_valid_string(input) {
					println!("\nRemoving item...");
					helpers::remove_item(&mut start, String::from(input), value_equals);
				}
				else {
					println!("\nCould not parse input!");
				}
			}
		}
		else if string == "l" {
			println!("\nList print...");
			helpers::print_list(&start);
		}
		else if string == "i" {
			println!("\nIterator print...");
			helpers::print_iterator(&start);
		}
		else if string == "a" {
			println!("\nArray print...");
			helpers::print_array(&start);
		}
		else if string == "r" {
			println!("\nRecursive print...");
			helpers::print_recursive(&start);
		}
		else if string == "f" {
			println!("\nFold print...");
			helpers::print_fold(&start);
		}
		else if string == "b" {
			println!("\nFoldback print...");
			helpers::print_foldback(&start);
		}
		else if is_valid_string(&string[..]) {
			println!("\nInserting item...");
			helpers::insert_item(&mut start, string, insert_before);
		}
		else {
			println!("\nCould not parse input!");
		}
	}
}
