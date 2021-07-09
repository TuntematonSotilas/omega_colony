use oxygengine::prelude::*;
use crate::states::splash::SplashState;

#[derive(Default)]
pub struct LoadingState {
    preloader: Option<AssetPackPreloader>,
}

impl State for LoadingState {
    fn on_enter(&mut self, universe: &mut Universe) {
        
		let token = universe.expect_resource::<AppLifeCycle>().current_state_token();

		let mut world = universe.world_mut();
		world.spawn((
			CompositeCamera::new(CompositeScalingMode::CenterAspect),
			CompositeTransform::scale(640.0.into()),
			NonPersistent(token),
		));

		world.spawn((
			CompositeRenderable(
				Text::new("Arial", "Loading")
					.color(Color::rgb(255,220,78))
					.align(TextAlign::Center)
					.size(20.)
					.into(),
			),
			CompositeTransform::translation([0., 0.].into()),
			NonPersistent(token),
		));
    }
    
    fn on_process(&mut self, universe: &mut Universe) -> StateChange {
        let mut assets = universe.expect_resource_mut::<AssetsDatabase>();
        if let Some(preloader) = &mut self.preloader {
            if preloader.process(&mut assets).unwrap() {
                return StateChange::Swap(Box::new(SplashState));
            }
        } else {
            self.preloader = Some(
                AssetPackPreloader::new("assets.pack", &mut assets, vec!["set://assets.txt"])
                    .expect("could not create asset pack preloader"),
            );
        }
        StateChange::None
    }
}
