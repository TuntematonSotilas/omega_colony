use oxygengine::prelude::*;

use crate::{
	components::selector::Selector,
	resources::selector::SelectorPos,
};

pub struct SelectorSystem;

impl<'s> System<'s> for SelectorSystem {
    type SystemData = (
        ReadStorage<'s, Selector>,
		WriteStorage<'s, CompositeTransform>,
		WriteStorage<'s, CompositeVisibility>,
		Read<'s, SelectorPos>,
    );

    fn run(
        &mut self, 
		(selectors, mut transforms, mut visibilties, selector_pos)
		: Self::SystemData,
    ) {
       
		for (transform, visibilty, _selector) in (&mut transforms, &mut visibilties, &selectors).join() {	
			let tr_pos = transform.get_translation();
			if tr_pos != selector_pos.pos {
				transform.set_translation(selector_pos.pos);
				if visibilty.0 == false {
					visibilty.0 = true;
				}
			}
		}
	}	
}