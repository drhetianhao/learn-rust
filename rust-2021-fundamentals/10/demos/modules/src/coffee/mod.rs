// Top level Coffee module for WBC - declares the espresso submodule
pub mod espresso;

#[derive(Debug)]
pub struct Coffee {
	pub name: String,
	pub cost: f64,
	pub count: i32
}
