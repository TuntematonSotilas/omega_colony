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
			if selected.visible != visibilty.0 {
				transform.set_translation(selected.pos);
				visibilty.0 =! visibilty.0;
			}
		}
	}	
}