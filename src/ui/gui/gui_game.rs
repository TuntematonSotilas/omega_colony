use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use crate::ui::{
	components::{
        top_bar::top_bar,
        side_panel::side_panel,
        log_box::log_box,
    }, 
	make_text_variants,
    make_bkg_variants,
	make_button_variants,
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
    make_text_variants(
        "unit",
        ThemedTextMaterial {
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 9.,
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
				source: Rect { left: 2., right: 4., top: 4., bottom: 4.},
				destination: Rect { left: 2., right: 4., top: 4., bottom: 4.},
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

    make_bkg_variants(
        "bkg_title",
        ThemedImageMaterial::Image(ImageBoxImage {
            id: "ui/bkg_title.png".to_owned(),
            scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
				source: Rect { left: 3., right: 3., top: 3., bottom: 3.},
				destination: Rect { left: 3., right: 3., top: 3., bottom: 3.},
				..Default::default()
			  }),
            ..Default::default()
        }),
        &mut theme.content_backgrounds,
    );

    make_bkg_variants(
        "logs",
        ThemedImageMaterial::Image(ImageBoxImage {
            id: "ui/log_box.png".to_owned(),
            scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
				source: Rect { left: 2., right: 4., top: 4., bottom: 4.},
				destination: Rect { left: 2., right: 4., top: 4., bottom: 4.},
				..Default::default()
			  }),
            ..Default::default()
        }),
        &mut theme.content_backgrounds,
    );

	make_button_variants(
        "",
        ThemedButtonMaterial {
            default: ThemedImageMaterial::Image(ImageBoxImage {
                id: "ui/panel_btn.png".to_owned(),
				scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
					source: Rect { left: 3., right: 3., top: 3., bottom: 3.},
					destination: Rect { left: 3., right: 3., top: 3., bottom: 3.},
					..Default::default()
				  }),
                ..Default::default()
            }),
            selected: ThemedImageMaterial::Image(ImageBoxImage {
                id: "ui/btn_c.png".to_owned(),
				scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
					source: Rect { left: 3., right: 3., top: 3., bottom: 3.},
					destination: Rect { left: 3., right: 3., top: 3., bottom: 3.},
					..Default::default()
				  }),
                ..Default::default()
            }),
            ..Default::default()
        },
        &mut theme.button_backgrounds,
    );

    theme
}

pub fn gui_game(context: WidgetContext) -> WidgetNode {
    let size_panel = Props::new(ContentBoxItemLayout {
        anchors: Rect {
            left: 1.,
            right: 1.,
            top: 0.,
            bottom: 0.,
        },
        align: Vec2 { x: 1., y: 0. },
        offset: Vec2 { x: 0., y: 30. },
        ..Default::default()
    })
    .with( SizeBoxProps {
        height: SizeBoxSizeValue::Exact(610.), 
        width: SizeBoxSizeValue::Exact(300.),
        ..Default::default()
    });
    let size_log = Props::new(ContentBoxItemLayout {
        anchors: Rect {
            left: 0.,
            right: 1.,
            top: 1.,
            bottom: 1.,
        },
        align: Vec2 { x: 0., y: 1. },
        ..Default::default()
    })
    .with( SizeBoxProps {
        height: SizeBoxSizeValue::Exact(30.), 
        width: SizeBoxSizeValue::Exact(250.),
        ..Default::default()
    });

    widget! {
        (#{context.key} content_box | {theme_game()} [
            (#{"top_bar"} top_bar)
            (#{"size_panel"} size_box: {size_panel} {
                content = (#{"side_panel"} side_panel)
            })
            (#{"size_log"} size_box: {size_log} {
                content = (#{"log_box"} log_box)
            })
        ])
    }
}