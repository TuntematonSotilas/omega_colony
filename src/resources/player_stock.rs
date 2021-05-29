use std::collections::HashMap;
use crate::resources::stock::StockType;

#[derive(Default)]
pub struct PlayerStock {
	pub is_init: bool,
	pub stock: HashMap<StockType, u32>,
}

impl PlayerStock {
	pub fn init(&mut self) {
		self.stock.insert(StockType::Energy, 100);
        self.stock.insert(StockType::Steel, 100);
		self.stock.insert(StockType::Water, 100);
        self.stock.insert(StockType::Cereal, 100);
		self.is_init = true;
	}
}
