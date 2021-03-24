use oxygengine::prelude::*;

use crate::{
	components::selector::Selector,
	resources::selected::Selected,
};

pub struct SelectorSystem;

impl<'s> System<'s> for SelectorSystem {
    type SystemData = (
        ReadStorage<'s, Selector>,
		WriteStorage<'s, CompositeTransform>,
		WriteStorage<'s, CompositeVisibility>,
		Read<'s, Selected>,
    );

    fn run(
        &mut self, 
		(selectors, mut transforms, mut visibilties, selected)
		: Self::SystemData,
    ) {
       
		for (transform, visibilty, _selector) in (&mut transforms, &mut visibilties, &selectors).join() {	
			let tr_pos = transform.get_translation();
			if tr_pos != selected.pos {
				transform.set_translation(selected.pos);
				if visibilty.0 == false {
					visibilty.0 = true;
				}
			}
		}
	}	
}