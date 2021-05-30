use oxygengine::prelude::*;

use crate::{
	resources::{
		ui_widget::UiWidget,
		stock::Stock,
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
		Read<'s, Stock>,
    );

    fn run(
        &mut self, 
		(mut ui, ui_widget, mut player_stock, stock)
		: Self::SystemData,
    ) {
		if !player_stock.is_init {
			if let Some(app) = ui.application_mut("") {
				if let Some(top_bar) = &ui_widget.top_bar {
					player_stock.init();
					app.send_message(top_bar, TopBarSignal::InitRefeStock(stock.to_owned(), player_stock.to_owned()));
				}
			}
		}
	}	
}