use oxygengine::prelude::*;
use crate::states::splash::SplashState;

#[derive(Default)]
pub struct LoadingState {
    preloader: Option<AssetPackPreloader>,
}

impl State for LoadingState {
    fn on_enter(&mut self, universe: &mut Universe) {
        let token = universe.read_resource::<AppLifeCycle>().current_state_token();
        
        //Camera
		universe
            .create_entity()
            .with(CompositeCamera::new(CompositeScalingMode::CenterAspect))
            .with(CompositeTransform::scale(640.0.into()))
            .with(NonPersistent(token))
			.build();
        
        //Text
		universe
			.create_entity()
			.with(CompositeRenderable(
				Text::new("Arial", "Loading")
					.color(Color::rgb(255,220,78))
					.align(TextAlign::Center)
					.size(20.)
					.into(),
			))
			.with(CompositeTransform::translation([0., 0.].into()))
			.with(NonPersistent(token))
            .build();
    }
    
    fn on_process(&mut self, universe: &mut Universe) -> StateChange {
        let assets = &mut universe.expect_resource_mut::<AssetsDatabase>();
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
