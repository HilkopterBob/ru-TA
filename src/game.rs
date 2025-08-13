use crate::{effects, entities, items, level, player};
use std::collections::HashMap;
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
    player: Option<player::Player>,
    level_list: Option<Vec<level::Level>>,
    items_list: Option<Vec<items::Item>>,
    entities_list: Option<Vec<entities::Entity>>,
    effects_list: Option<Vec<effects::Effect>>,
    //TODO: add config management, eg with serde or figments
    //TODO: add save file/functionality
    menu: Menu,
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

    pub fn mainstate() -> () {

    }
}
