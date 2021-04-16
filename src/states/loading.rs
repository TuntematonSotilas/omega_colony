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
            .with(CompositeTransform::scale(640.0.into()))
            .with(NonPersistent(token))
			.build();
        
        //Background
		world
        .create_entity()
        .with(CompositeRenderable(
            Rectangle {
                color: Color::black(),
                rect: Rect::with_size([2000.0, 2000.0].into()).align(0.5.into()),
            }
            .into(),
        ))
        .with(CompositeTransform::translation([0., 0.].into()))
        .with(NonPersistent(token))
        .build();

        //Text
		world
			.create_entity()
			.with(CompositeRenderable(
				Text::new("Arial", "Loading")
					.color(Color::rgb(255,220,78))
					.align(TextAlign::Center)
					.size(20.0)
					.into(),
			))
			.with(CompositeTransform::translation([0., 0.].into()))
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
