use oxygengine::prelude::*;

use crate::{
	components::interactive_sprite::InteractiveSprite,
	resources::{
		camera::Camera,
		selected::Selected,
	},
};

pub struct SpriteClickSystem;

impl<'s> System<'s> for SpriteClickSystem {
    type SystemData = (
        Read<'s, InputController>,
		Read<'s, CompositeCameraCache>,
		Read<'s, Camera>,
		ReadStorage<'s, InteractiveSprite>,
		ReadStorage<'s, CompositeTransform>,
		Write<'s, Selected>,
    );

    fn run(
        &mut self, 
		(input, camera_cache, camera_res, interactive_sprites, transforms, mut selected)
		: Self::SystemData,
    ) {
        if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
			let x = input.axis_or_default("mouse-x");
            let y = input.axis_or_default("mouse-y");
			let point = [x, y].into();
			
			if let Some(camera_entity) = camera_res.camera {
				if let Some(pos) = camera_cache.screen_to_world_space(camera_entity, point) {

					for (transform, interactive_sprite) in (&transforms, &interactive_sprites).join() {	
						let matrix = transform.matrix();
						if let Some(inv_mat) = matrix.inverse() {
							let pos_inv = pos * inv_mat;
							if pos_inv.x >= 0.0 && 
								pos_inv.x <= interactive_sprite.w && 
								pos_inv.y >= 0.0 && 
								pos_inv.y < interactive_sprite.h {
									let tile_pos = matrix * Vec2::new(16.,0.);
									selected.pos = tile_pos;
									selected.code = interactive_sprite.code.clone();
							}
						}
					}
				}
			}
        }
	}	
}