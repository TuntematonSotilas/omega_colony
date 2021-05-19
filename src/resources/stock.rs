use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StockType {
	Energy,
	Water,
	Cereal,
	Steel,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockItem  {
    pub name: String,
	pub pic: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockItemCost  {
    pub item: StockItem,
	pub cost: u32,
}

#[derive(Default, Clone)]
pub struct Stock {
	pub refe: HashMap<StockType, StockItem>,
}

impl Stock {
	pub fn init(&mut self) {
		self.refe = HashMap::new();
		let energy = StockItem {
			name: "Energy".to_string(),
			pic: "ui/energy.png".to_string(),
		};
		let water = StockItem {
			name: "Water".to_string(),
			pic: "ui/water.png".to_string(),
		};
		let steel = StockItem {
			name: "Steel".to_string(),
			pic: "ui/steel.png".to_string(),
		};
		let cereal = StockItem {
			name: "Cereal".to_string(),
			pic: "ui/cereal.png".to_string(),
		};
		self.refe.insert(StockType::Energy, energy);
        self.refe.insert(StockType::Steel, steel);
		self.refe.insert(StockType::Water, water);
        self.refe.insert(StockType::Cereal, cereal);
	}
}