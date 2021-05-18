use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StockType {
	Energy,
	Water,
	Cereal,
	Steel,
}

#[derive(Default, Clone)]
pub struct Stock {
	pub data: HashMap<StockType, u32>,
}

impl Stock {
	pub fn init(&mut self) {
		self.data = HashMap::new();
		self.data.insert(StockType::Energy, 100);
		self.data.insert(StockType::Water, 100);
        self.data.insert(StockType::Cereal, 100);
        self.data.insert(StockType::Steel, 100);
	}
}