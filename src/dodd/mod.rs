use item::Item;

pub mod helpers;
pub mod item;

pub type Node<T> = Option<Box<Item<T>>>;
