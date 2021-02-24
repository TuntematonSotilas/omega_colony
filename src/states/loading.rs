use oxygengine::prelude::*;
use crate::states::splash::SplashState;

#[derive(Default)]
pub struct LoadingState {
    preloader: Option<AssetPackPreloader>,
}

impl State for LoadingState {
    fn on_enter(&mut self, world: &mut World) {
        let token = world.read_resource::<AppLifeCycle>().current_state_token();
        
        //Camera
		world
            .create_entity()
            .with(CompositeCamera::new(CompositeScalingMode::CenterAspect))
            .with(CompositeTransform::scale(320.0.into()))
            .with(NonPersistent(token))
			.build();
        
        //Text
		world
			.create_entity()
			.with(CompositeRenderable(
				Text::new("Verdana", "Loading")
					.color(Color::rgb(66, 165, 219))
					.align(TextAlign::Center)
					.size(10.0)
					.into(),
			))
			.with(CompositeTransform::translation([0., -10.].into()))
			.with(NonPersistent(token))
            .build();
    }
    
    fn on_process(&mut self, world: &mut World) -> StateChange {
        let assets = &mut world.write_resource::<AssetsDatabase>();
        if let Some(preloader) = &mut self.preloader {
            if preloader.process(assets).unwrap() {
                return StateChange::Swap(Box::new(SplashState));
            }
        } else {
            self.preloader = Some(
                AssetPackPreloader::new("assets.pack", assets, vec!["set://assets.txt"])
                    .expect("could not create asset pack preloader"),
            );
        }
        StateChange::None
    }
}
