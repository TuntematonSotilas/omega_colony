use oxygengine::prelude::*;

use crate::resources::selected::Selected;

pub struct PanelSystem;

impl<'s> System<'s> for PanelSystem {
    type SystemData = (
        WriteStorage<'s, CompositeUiElement>,
		Write<'s, Selected>,
    );

    fn run(
        &mut self, 
		(mut ui_elements, selected)
		: Self::SystemData,
    ) {
		for ui_element in (&mut ui_elements).join() {	
			for child in &mut ui_element.children {  
				if let Some(id) = &child.id {
					if id == "title" {
						for sec_child in &mut child.children {  
							if let UiElementType::Text(text) = &mut sec_child.element_type {
								text.text = selected.name.clone().into();
							}
						}
					}
				}
			}
		}
	}
}