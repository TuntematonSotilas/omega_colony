use std::collections::HashMap;

#[derive(Clone)]
pub enum ItemType {
	Unit,
	Building,
	Upgrade
}

impl Default for ItemType {
    fn default() -> Self { ItemType::Unit }
}

#[derive(Default, Clone)]
pub struct Item  {
    pub name: String,
	pub itype: ItemType,
    pub cost: u32,
}

#[derive(Default, Clone)]
pub struct Referential {
	pub items: HashMap<String, Item>
}