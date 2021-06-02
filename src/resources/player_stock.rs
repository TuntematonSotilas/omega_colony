use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::resources::stock::{StockType, StockItemCost};

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

	pub fn is_buyabe(&self, item_cost: HashMap<StockType, StockItemCost>) -> bool
	{
		let check = item_cost.iter().all(|(stock_type, item_cost)| {
			let cnt = self.stock.get(stock_type).cloned().unwrap_or_default();
			cnt >= item_cost.cost
		});
		check
	}
}
