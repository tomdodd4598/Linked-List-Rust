use super::Node;

use std::fmt::Display;
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
	pub fn print_get_next(&self) -> &Node<T> {
		print!("{}{}", self.value, if self.next.is_none() { "\n" } else { ", " });
		&self.next
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
			current = match previous.next {
				None => panic!("index out of bounds: the length is {} but the index is {}", _i, n),
				Some(ref x) => x,
			}
		}
		current
	}
}

pub fn fold<T, A, R, S, L, E>(f_some: S, f_last: L, f_empty: E, accumulator: A, item: &Node<T>) -> R
	where T: Display, S: Copy + Fn(&Item<T>, &Item<T>, A) -> A, L: Fn(&Item<T>, A) -> R, E: Fn(A) -> R
{
	match item {
		None => f_empty(accumulator),
		Some(x) => {
			let next = &x.next;
			match next {
				None => f_last(&*x, accumulator),
				Some(y) => fold(f_some, f_last, f_empty, f_some(&*x, &*y, accumulator), next),
			}
		},
	}
}

pub fn foldback<T, A, R, S, L, E>(f_some: S, f_last: L, f_empty: E, generator: &dyn Fn(A) -> R, item: &Node<T>) -> R
	where T: Display, S: Copy + Fn(&Item<T>, &Item<T>, A) -> A, L: Fn(&Item<T>) -> A, E: Fn() -> A
{
	match item {
		None => generator(f_empty()),
		Some(x) => {
			let next = &x.next;
			match next {
				None => generator(f_last(&*x)),
				Some(y) => foldback(f_some, f_last, f_empty, &|inner_val| generator(f_some(&*x, &*y, inner_val)), next),
			}
		},
	}
}
