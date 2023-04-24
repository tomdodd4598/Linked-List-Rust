use super::Node;
use crate::dodd::item::{self, Item};

use std::fmt::Display;

pub fn insert_item<T, F>(start: &mut Node<T>, val: T, insert_before: F)
	where T: Display, F: Fn(&T, &Item<T>) -> bool
{
	println!("Creating item: {}", val);
	unsafe {
		let mut tracer = start as *mut Node<T>;
		while let Some(x) = &mut *tracer {
			if insert_before(&val, &x) {
				break
			}
			else {
				tracer = &mut x.next
			}
		}
		*tracer = Some(Box::new(Item { value: val, next: tracer.replace(None) }))
	}
}

pub fn remove_item<T, F>(start: &mut Node<T>, val: T, value_equals: F)
	where T: Display, F: Fn(&Item<T>, &T) -> bool
{
	unsafe {
		let mut tracer = start as *mut Node<T>;
		while let Some(x) = &mut *tracer {
			if value_equals(&x, &val) {
				break
			}
			else {
				tracer = &mut x.next
			}
		}

		match *tracer {
			None => println!("Item {} does not exist!", val),
			Some(ref mut x) => *tracer = (&mut x.next as *mut Node<T>).replace(None),
		}
	}
}

pub fn remove_all<T>(start: &mut Node<T>)
	where T: Display
{
	*start = None
}

pub fn print_loop<T>(mut start: &Node<T>)
	where T: Display
{
	while let Some(x) = start {
		start = x.print_get_next()
	}
}

pub fn print_iterator<T>(start: &Node<T>)
	where T: Display
{
	struct ItemIterator<'a, T>
		where T: Display
	{
		pub item: &'a Node<T>
	}

	impl<'a, T> Iterator for ItemIterator<'a, T>
		where T: Display
	{
		type Item = &'a Box<Item<T>>;

		fn next(&mut self) -> Option<Self::Item> {
			match self.item {
				None => None,
				Some(x) => {
					let current = self.item.as_ref();
					self.item = &x.next;
					current
				},
			}
		}
	}

	for item in (ItemIterator { item: start }) {
		item.print_get_next();
	}
}

pub fn print_array<T>(start: &Node<T>)
	where T: Display
{
	if let Some(x) = start {
		let mut item = &x[0];
		let mut i = 1;
		while item.print_get_next().is_some() {
			item = &x[i];
			i += 1;
		}
	}
}

pub fn print_recursive<T>(start: &Node<T>)
	where T: Display
{
	if let Some(x) = start {
		print_recursive(x.print_get_next())
	}
}

pub fn print_fold<T>(start: &Node<T>)
	where T: Display
{
	let f_some = |current: &Item<T>, _: &Item<T>, accumulator: String| -> String { format!("{}{}, ", accumulator, current.value) };
	let f_last = |current: &Item<T>, accumulator: String| -> String { format!("{}{}\n", accumulator, current.value) };
	let f_empty = |accumulator: String| -> String { accumulator };
	print!("{}", item::fold(f_some, f_last, f_empty, "".to_string(), start))
}

pub fn print_foldback<T>(start: &Node<T>)
	where T: Display
{
	let f_some = |current: &Item<T>, _: &Item<T>, inner_val: String| -> String { format!("{}, {}", current.value, inner_val) };
	let f_last = |current: &Item<T>| -> String { format!("{}\n", current.value) };
	let f_empty = || -> String { "".to_string() };
	print!("{}", item::foldback(f_some, f_last, f_empty, &|x: String| -> String { x }, start))
}
