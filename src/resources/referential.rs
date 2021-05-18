use std::collections::HashMap;

use crate::resources::stock::StockType;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Building {
	Base,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Unit {
	Technician,
}

#[derive(Default, Clone)]
pub struct RefeItem  {
    pub name: String,
	pub preview: String,
    pub cost: HashMap<StockType, u32>,
}

#[derive(Default, Clone)]
pub struct Referential {
	pub is_init: bool,
	pub buildings: HashMap<Building, RefeItem>,
	pub units: HashMap<Unit, RefeItem>,
}

impl Referential {
	pub fn init(&mut self) {
		// buildings
		self.buildings = HashMap::new();
		
		// units
		let mut stock_tech = HashMap::new();
		stock_tech.insert(StockType::Energy, 100);

		let technician = RefeItem {
			name: "Technician".to_string(),
			preview: "ui/technician.png".to_string(),
			cost: stock_tech,
		};
		self.units = HashMap::new();
		self.units.insert(Unit::Technician, technician);
		
		self.is_init = true;
	}
}