use crate::{effects, entities, items, level, player, utils};
use std::{collections::HashMap, option};
use crate::level::Choice;

#[derive(Debug)]
struct Menu {
    title: String,
    choices: Vec<level::Choice>,
}

impl Menu {
    pub fn new() -> Menu {
        let menu = Menu{
            title: "ru-TA - Nicks Rust Textadventure".to_string(),
            choices: vec![
                    level::Choice{
                        handle: "New Game".to_string(),
                        output_text: "Starting New Game".to_string(),
                        condition: None,
                        trigger: None
                    },
                    Choice{
                        handle: "Load Game".to_string(),
                        output_text: "Loading from existing save".to_string(),
                        condition: None,
                        trigger: None
                    },
                    Choice{
                        handle: "Options".to_string(),
                        output_text: "".to_string(), // No Output, move to Level: Options
                        condition: None,
                        trigger: None
                    },
                    Choice{
                        handle: "Exit".to_string(),
                        output_text: "Exiting...".to_string(),
                        condition: None,
                        trigger: None               // trigger exit
                    },
                    
                ]
        };
        return menu;
    }
}

#[derive(Debug)]
pub struct Game {
    pub player: Option<player::Player>,
    pub level_list: Option<Vec<level::Level>>,
    pub items_list: Option<Vec<items::Item>>,
    pub entities_list: Option<Vec<entities::Entity>>,
    pub effects_list: Option<Vec<effects::Effect>>,
    //TODO: add config management, eg with serde or figments
    //TODO: add save file/functionality
    pub menu: Menu,
    //TODO: add stat tracking right here, maybe?
        //something like steps taken(level switches), enemies slain, treasures found...
}

impl Game {
    pub fn new() -> Game {
        /*  Logic:
           creates a game instance with no data except the menu to start execution.
           (the menu will trigger the save state and asset loading later).
           Returns the game instance.
        */
        let game = Game {
            player: None,
            level_list: None,
            items_list: None,
            entities_list: None,
            effects_list: None,
            menu: Menu::new(),
        };
        return game;
    }

    pub fn run(mut self: Self) -> Self {
        /*
            This should start the menu
         */

        //self.mainstate()

        self = self.mainstate();
        return self;
    }

    pub fn mainstate(mut self: Self) -> Self {
        //let  = Some(self.player.location.get_available_chocies());
        
        let available_choices = self.player.clone().unwrap().location.get_availible_choices();

        clearscreen::clear().expect("failed to clear screen");
        
        if let Some(player) = self.player.clone() {
            utils::Printer::print(
                player.location.description.clone(),
                available_choices.clone(),
                player.health.to_string(),
                player.money.to_string(),
                player.location.name.clone()
            );     
        }

        let player_input: i32 = utils::InputHandler::getInput() - 1;
        println!("got input: {}", player_input);


        if available_choices[
            usize::try_from(player_input).unwrap()
            ].trigger != None {
            let edited_level = self.player.clone().unwrap()
                .location
                .handle_trigger(
                    available_choices[
                        usize::try_from(player_input).unwrap()
                    ].trigger.as_ref().unwrap()
            );

            let mut edited_player = self.player.clone().unwrap();
            edited_player.location = edited_level;

            self.player = Some(edited_player);
            
            return self;
                };
        
        return self;

        
        // if let Some( mut player) = self.player.clone() {
        //     player.location = current_level;   
        // }


    }
}
