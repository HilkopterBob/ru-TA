use boxy_cli::prelude::*;
use std::io;

use crate::level::Choice;

pub struct Printer {

}

impl Printer {
    pub fn box_print(level_text: String, choices: Vec<Choice>, player_health: String, player_wealth: String, player_location: String) {
        // TODO: conoslidate paramiters with player struct field names

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
            .add_line(format!("Health: {player_health}").as_str(), "#663399", )
            .add_line(format!("Wealth: {player_wealth}").as_str(), "#663399", )
            .add_line(format!("Location: {player_location}").as_str(), "#663399", )
            .add_segment("-", "#663399", BoxAlign::Center)
            .add_line(format!("{level_text}").as_str(), "#663399", )
            .width(0)
            .height(0);
            //.build();

        ui = ui.add_segment("Choices:", "#ffffff", BoxAlign::Left);
        for (i, choice) in choices.iter().enumerate(){
           // ui = ui.add_line(format!(" [{}]: {choice_handle}", index = i + 1, choice_handle = choice.handle.as_str()).as_str(), "#ffffff");
            ui = ui.add_line(format!(" [{index}]: {choice_handle}", index = i + 1, choice_handle = choice.handle).as_str(), "#ffffff");
        }

        clearscreen::clear().expect("failed to clear screen");
        ui.build().display();

        // let mut line = String::new();
        // let b1 = std::io::stdin().read_line(&mut line).unwrap();
        // println!("{}", line);
    }

    pub fn print(level_text: String, choices: Vec<Choice>, player_health: String, player_wealth: String, player_location: String) {
        // TODO: conoslidate paramiters with player struct field names
   
        println!("RU-TA");
        println!("Player:");
        println!("Health: {player_health}");
        println!("Wealth: {player_wealth}");
        println!("Location: {player_location}");
        println!("{level_text}");
        

        println!("Choices:");
        for (i, choice) in choices.iter().enumerate(){
           // ui = ui.add_line(format!(" [{}]: {choice_handle}", index = i + 1, choice_handle = choice.handle.as_str()).as_str(), "#ffffff");
            println!(" [{index}]: {choice_handle}", index = i + 1, choice_handle = choice.handle);
        }

        // let mut line = String::new();
        // let b1 = std::io::stdin().read_line(&mut line).unwrap();
        // println!("{}", line);
    }
}

pub struct InputHandler {
}

impl InputHandler{
    pub fn getInput() -> i32 {
        // TODO: Implement correct input 
        //handling,
        //sanitization,
        // game commands (debug, give, spawn, kill, move, set <game-var/condition>)
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");
        let input: i32 = input.trim().parse().expect("Please type a number!");
        
        return input;
    }
}