use oxygengine::prelude::*;

use crate::resources::{
	referential::Referential,
	selected::Selected,
};

pub struct PanelSystem;

impl<'s> System<'s> for PanelSystem {
    type SystemData = (
        WriteStorage<'s, CompositeUiElement>,
		Write<'s, Selected>,
		Read<'s, Referential>,
    );

    fn run(
        &mut self, 
		(mut ui_elements, selected, referential)
		: Self::SystemData,
    ) {
		for ui_element in (&mut ui_elements).join() {	
			for child in &mut ui_element.children {  
				if let Some(id) = &child.id {
					if id == "title" {
						for sec_child in &mut child.children {  
							if let UiElementType::Text(text) = &mut sec_child.element_type {
								let refe = referential.refes.get(&selected.code);
								if let Some(item) = refe {
									text.text = item.name.clone().into();
								}
							}
						}
					}
				}
			}
		}
	}
}