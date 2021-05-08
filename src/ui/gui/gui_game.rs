use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use crate::ui::{
	components::{
        top_bar::top_bar,
        side_panel::side_panel,
    }, 
	make_text_variants,
    make_bkg_variants,
};

fn theme_game() -> ThemeProps {
    let mut theme = make_default_theme(
        color_from_rgba(255,255,255, 1.),
        color_from_rgba(255, 255, 255, 1.),
        color_from_rgba(255, 255, 255, 1.),
        color_from_rgba(255, 255, 255, 1.),
    );
    make_text_variants(
        "",
        ThemedTextMaterial {
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 12.,
            },
            alignment: TextBoxAlignment::Center,
            ..Default::default()
        },
        &mut theme.text_variants,
    );

    make_bkg_variants(
        "",
        ThemedImageMaterial::Image(ImageBoxImage {
            id: "ui/panel_bkg.png".to_owned(),
            scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
				source: Rect { left: 4., right: 4., top: 4., bottom: 4.},
				destination: Rect { left: 4., right: 4., top: 4., bottom: 4.},
				..Default::default()
			  }),
            ..Default::default()
        }),
        &mut theme.content_backgrounds,
    );

	make_bkg_variants(
        "data",
        ThemedImageMaterial::Image(ImageBoxImage {
            id: "ui/panel_data.png".to_owned(),
            scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
				source: Rect { left: 4., right: 4., top: 4., bottom: 4.},
				destination: Rect { left: 4., right: 4., top: 4., bottom: 4.},
				..Default::default()
			  }),
            ..Default::default()
        }),
        &mut theme.content_backgrounds,
    );

    theme
}

pub fn gui_game(context: WidgetContext) -> WidgetNode {
    let size = Props::new(ContentBoxItemLayout {
        anchors: Rect {
            left: 1.,
            right: 1.,
            top: 0.,
            bottom: 0.,
        },
        align: Vec2 { x: 1., y: 0. },
        offset: Vec2 { x: 0., y: 40. },
        ..Default::default()
    })
    .with(SizeBoxProps {
        height: SizeBoxSizeValue::Exact(300.), 
        width: SizeBoxSizeValue::Exact(300.),
        ..Default::default()
    });

    widget! {
        (#{context.key} content_box | {theme_game()} [
            (#{"top_bar"} top_bar)
            (#{"size"} size_box: {size} {
                content = (#{"side_panel"} side_panel)
            })
        ])
    }
}