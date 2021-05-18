use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

pub fn log_box(context: WidgetContext) -> WidgetNode {
	let bkg = PaperProps { 
        frame: None, 
        variant: "logs".to_owned(),
        ..Default::default() 
    };
    widget! {
        (#{context.key} paper: {bkg})
    }
}