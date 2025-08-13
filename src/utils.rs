use boxy_cli::prelude::*;
use std::io;

pub struct Printer {

}

impl Printer {
    pub fn print(choices: Vec<String>, player_health: String, player_wealth: String, player_location: String) {
        
        let mut ui = Boxy::builder()
            .box_type(BoxType::Double)       // Set border style
            .color("#434343ff")               // Set border color
            .padding(
                BoxPad::uniform(1),         // External padding
                BoxPad::from_tldr(2, 2, 1, 1) // Internal padding
            )
            .align(BoxAlign::Center)         // Center the box in the terminal
            .add_segment("RU-TA", "#ff0000ff", BoxAlign::Center)
            .add_segment("Player:", "#663399", BoxAlign::Left)
            .add_line(format!("Player Health: {player_health}").as_str(), "#663399", )
            .add_line(format!("Player Wealth: {player_wealth}").as_str(), "#663399", )
            .add_line(format!("Player Location: {player_location}").as_str(), "#663399", )
            .add_segment("Choices:", "#ffffff", BoxAlign::Left)
            .width(0)
            .height(0);
            //.build();

        for choice in choices{
            ui = ui.add_line(&choice, "#ffffff");
        }

        clearscreen::clear().expect("failed to clear screen");
        ui.build().display();

        // let mut line = String::new();
        // let b1 = std::io::stdin().read_line(&mut line).unwrap();
        // println!("{}", line);
    }
}