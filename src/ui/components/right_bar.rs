use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

fn right_bar_comp(context: WidgetContext) -> WidgetNode {

	let WidgetContext {
        key,
        props,
        named_slots,
        ..
    } = context;

	let bkg = PaperProps { 
		frame: None, 
		..Default::default() 
	};
	/*let margin = Props::new(ContentBoxItemLayout {
		margin: Rect {
			top: 6.,
			left: 50.,
			..Default::default()
		},
		..Default::default()
	});*/
	let anchor = Props::new(ContentBoxItemLayout {
		anchors: Rect {
            left: 0.0,
            right: 1.0,
            top: 0.0,
            bottom: 1.0,
        },
		align: Vec2 { x: 0.0, y: 1.0 },
		..Default::default() 
	});
	let dialog_size_props = Props::new(ContentBoxItemLayout {
        anchors: Rect {
            left: 0.0,
            right: 1.0,
            top: 1.0,
            bottom: 1.0,
        },
        align: Vec2 { x: 0.0, y: 1.0 },
        ..Default::default()
    })
    .with(SizeBoxProps {
        width: SizeBoxSizeValue::Exact(50.0),
        height: SizeBoxSizeValue::Exact(180.0),
        ..Default::default()
    });

	let img = Props::new(ImageBoxProps {
		width: ImageBoxSizeValue::Exact(16.),
		height: ImageBoxSizeValue::Exact(16.),
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: "ui/steel.png".to_owned(),
			..Default::default()
		}),
		..Default::default()
	});
	widget!{
		(#{"ctn"} content_box [
			(#{"sizeX"} size_box: {dialog_size_props} {
				content = (#{"bkg"} content_box: {props.clone()} [
					(#{"background"} image_box: {img})
				])
			})
		])
	}
}

pub fn right_bar(context: WidgetContext) -> WidgetNode {
	widget! {
		(#{context.key} right_bar_comp)
	}
}