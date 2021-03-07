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
		Write<'s, SelectorPos>,
    );

    fn run(
        &mut self, 
		(selectors, mut transforms, mut selector_pos)
		: Self::SystemData,
    ) {
       
		for (transform, selector) in (&mut transforms, &selectors).join() {	
			let tr_pos = transform.get_translation();
			if transform.get_translation() != selector_pos.pos {
				transform.set_translation(selector_pos.pos);
				selector_pos.pos = tr_pos;

			}
		}
	
			
        
	}	
}