use oxygengine::prelude::*;

use crate::{
	resources::{
		ui_widget::UiWidget,
		player_stock::PlayerStock,
	},
	ui::components::top_bar::TopBarSignal,
};

pub struct PlayerStockSystem;

impl<'s> System<'s> for PlayerStockSystem {
    type SystemData = (
        Write<'s, UserInterfaceRes>,
		Read<'s, UiWidget>,
		Write<'s, PlayerStock>,
    );

    fn run(
        &mut self, 
		(mut ui, ui_widget, mut player_stock)
		: Self::SystemData,
    ) {
		
		if !player_stock.is_init {
			player_stock.init();
			if let Some(app) = ui.application_mut("") {
				if let Some(side_panel) = &ui_widget.side_panel {
					//let stock = world.read_resource::<Stock>();
						//let cp = stock.clone();
						//app.send_message(&caller, TopBarSignal::InitRefeStock(*cp));
						
						app.send_message(side_panel, TopBarSignal::UpdateStock(player_stock.to_owned()));

						
				}
			}
		}
	}	
}