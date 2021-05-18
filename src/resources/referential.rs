use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::resources::stock::StockType;

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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RefeItem  {
    pub name: String,
	pub preview: String,
    pub cost: HashMap<StockType, u32>,
	pub childs: HashMap<RefeCode, RefeItem>, 
}

#[derive(Default, Clone)]
pub struct Referential {
	pub is_init: bool,
	pub buildings: HashMap<RefeCode, RefeItem>,
	pub units: HashMap<RefeCode, RefeItem>,
}

impl Referential {
	pub fn init(&mut self) {
		// buildings
		let base = RefeItem {
			name: "Base".to_string(),
			preview: "ui/base.png".to_string(),
			cost: HashMap::new(),
			childs: HashMap::new(),
		};
		self.buildings = HashMap::new();
		self.buildings.insert(RefeCode::Base, base);

		// units
		let mut stock_tech = HashMap::new();
		stock_tech.insert(StockType::Energy, 100);
		let technician = RefeItem {
			name: "Technician".to_string(),
			preview: "ui/technician.png".to_string(),
			cost: stock_tech,
			childs: HashMap::new(),
		};
		self.units = HashMap::new();
		self.units.insert(RefeCode::Technician, technician);
		
		self.is_init = true;
	}
}