use super::Node;

use std::fmt::{Display, Formatter, Result};
use std::ops::Index;

pub struct Item<T>
	where T: Display
{
	pub value: T,
	pub next: Node<T>,
}

impl<T> Item<T>
	where T: Display
{
	pub fn new(value: T, next: Node<T>) -> Self {
		println!("Creating item: {}", value);
		Item { value, next }
	}
	
	pub fn print_get_next(&self) -> &Node<T> {
		let next = &self.next;
		print!("{}{}", &self, if next.is_some() { ", " } else { "\n" });
		next
	}
}

impl<T> Display for Item<T>
	where T: Display
{
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.value)
	}
}

impl<T> Drop for Item<T>
	where T: Display
{
	fn drop(&mut self) {
		println!("Dropping item: {}", self.value)
	}
}

impl<T> Index<usize> for Item<T>
	where T: Display
{
	type Output =  Self;

	fn index<'a>(&'a self, n: usize) -> &'a Self::Output {
		let mut previous;
		let mut current: &'a Item<T> = self;
		for _i in 1..=n {
			previous = current;
			current = match &previous.next {
				Some(x) => x,
				None => panic!("index out of bounds: the length is {} but the index is {}", _i, n),
			}
		}
		current
	}
}

pub fn fold<T, A, R>(f_some: fn(&Item<T>, &Item<T>, A) -> A, f_last: fn(&Item<T>, A) -> R, f_empty: fn(A) -> R, accumulator: A, item: &Node<T>) -> R
	where T: Display
{
	match item {
		Some(x) => {
			let next = &x.next;
			match next {
				Some(y) => fold(f_some, f_last, f_empty, f_some(&*x, &*y, accumulator), next),
				None => f_last(&*x, accumulator),
			}
		},
		None => f_empty(accumulator),
	}
}

pub fn foldback<T, A, R>(f_some: fn(&Item<T>, &Item<T>, A) -> A, f_last: fn(&Item<T>) -> A, f_empty: fn() -> A, generator: &dyn Fn(A) -> R, item: &Node<T>) -> R
	where T: Display
{
	match item {
		Some(x) => {
			let next = &x.next;
			match next {
				Some(y) => foldback(
					f_some, f_last, f_empty, &|inner_val| -> R { generator(f_some(&*x, &*y, inner_val)) }, next
				),
				None => generator(f_last(&*x)),
			}
		},
		None => generator(f_empty()),
	}
}
