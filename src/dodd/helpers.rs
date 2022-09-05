use super::item::Item;
use super::Node;

use std::fmt::Display;
use crate::dodd::item;

pub fn insert_item<T>(start: &mut Node<T>, val: T, insert_before: fn(&T, &Item<T>) -> bool)
	where T: Display
{
	unsafe {
		let mut tracer = start as *mut Node<T>;
		while let Some(x) = &mut *tracer {
			match insert_before(&val, &x) {
				true => break,
				false => tracer = &mut x.next,
			}
		}
		*tracer = Some(Box::new(Item::new(val, tracer.replace(None))))
	}
}

pub fn remove_item<T>(start: &mut Node<T>, val: T, value_equals: fn(&Item<T>, &T) -> bool)
	where T: Display
{
	unsafe {
		let mut tracer = start as *mut Node<T>;
		while let Some(x) = &mut *tracer {
			match value_equals(&x, &val) {
				true => break,
				false => tracer = &mut x.next,
			}
		}

		match &mut *tracer {
			Some(x) => *tracer = (&mut x.next as *mut Node<T>).replace(None),
			None => println!("Item {} does not exist!", val),
		}
	}
}

pub fn remove_all<T>(start: &mut Node<T>)
	where T: Display
{
	*start = None
}

pub fn print_list<T>(mut start: &Node<T>)
	where T: Display
{
	while let Some(x) = start {
		start = x.print_get_next()
	}
}

pub fn print_iterator<T>(start: &Node<T>)
	where T: Display
{
	struct ItemIterator<'a, T> where T: Display {
		pub item: &'a Node<T>
	}

	impl<'a, T> Iterator for ItemIterator<'a, T> where T: Display {
		type Item = &'a Box<Item<T>>;

		fn next(&mut self) -> Option<Self::Item> {
			match self.item {
				Some(x) => {
					let current = self.item.as_ref();
					self.item = &x.next;
					current
				},
				None => None,
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
