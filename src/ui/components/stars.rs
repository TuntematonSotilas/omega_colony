use oxygengine::user_interface::raui::core::prelude::*;

widget_component! {
    pub stars(key){
        let stars = Props::new(ImageBoxProps {
            width: ImageBoxSizeValue::Fill,
            height: ImageBoxSizeValue::Fill,
            material: ImageBoxMaterial::Image(ImageBoxImage {
                id: "ui/stars.png".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        });

        widget! {
            (#{"stars"} image_box: {stars})
        }
    }
}