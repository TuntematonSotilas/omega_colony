use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::resources::stock::{Stock, StockType, StockItemCost};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefeCode {
	Base,
	Technician
}

impl Default for RefeCode {
    fn default() -> RefeCode {
        RefeCode::Base
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefeItem  {
    pub name: String,
	pub preview: String,
    pub cost: HashMap<StockType, StockItemCost>,
	pub units: HashMap<RefeCode, RefeItem>, 
	pub upgrades: HashMap<RefeCode, RefeItem>, 
}

#[derive(Default)]
pub struct Referential {
	pub is_init: bool,
	pub buildings: HashMap<RefeCode, RefeItem>,
}

impl Referential {
	pub fn init(&mut self) {

		let mut stock = Stock::default();
		stock.init();

		// units
		let mut stock_tech = HashMap::new();
		stock_tech.insert(StockType::Energy, StockItemCost { 
			item: stock.refe.get(&StockType::Energy).cloned().unwrap_or_default(),
			cost: 50
		});
		stock_tech.insert(StockType::Steel, StockItemCost { 
			item: stock.refe.get(&StockType::Steel).cloned().unwrap_or_default(),
			cost: 100
		});
		let technician = RefeItem {
			name: "Technician".to_string(),
			preview: "ui/technician.png".to_string(),
			cost: stock_tech,
			units: HashMap::new(),
			upgrades: HashMap::new(),
		};
		
		// buildings
		let mut base_childs = HashMap::new();
		base_childs.insert(RefeCode::Technician, technician);

		let base = RefeItem {
			name: "Base".to_string(),
			preview: "ui/base.png".to_string(),
			cost: HashMap::new(),
			units: base_childs,
			upgrades: HashMap::new(),
		};
		self.buildings = HashMap::new();
		self.buildings.insert(RefeCode::Base, base);

		self.is_init = true;
	}
}