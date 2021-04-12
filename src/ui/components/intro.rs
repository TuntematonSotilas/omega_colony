use oxygengine::user_interface::raui::core::prelude::*;

widget_component! {
    pub intro_comp(key, props, state) {
        let landscape = Props::new(ImageBoxProps {
            content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
                horizontal_alignment: 0.5,
                vertical_alignment: 0.5,
            }),
            material: ImageBoxMaterial::Image(ImageBoxImage {
                id: "ui/landscape.png".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        });
        widget! {
            (#{key} content_box: {props.clone()} [
                (#{"landscape"} image_box: {landscape})
            ])
        }
	}
}

widget_component! {
    pub intro(key){
        widget! {
            (#{key} intro_comp )
        }
    }
}