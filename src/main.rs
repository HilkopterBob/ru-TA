#![doc(html_no_source)]



use level::{Choice, Condition};
use std::{thread, time::Duration};


mod effects;
mod entities;
mod game;
mod items;
mod level;
mod player;
mod utils;

fn main() {



    //let game = game::Game::new();
    //println!("{:#?}", game);
    let mut test_availible_choices_level = level::Level::new(
        "Test availible choices".to_string(),
        "This is a testlevel to test the aggregation of availible choices".to_string(),
        vec![
                    Choice{
                        handle: "Sollte sofort angezeigt werden".to_string(),
                        output_text: "test output".to_string(),
                        condition: None,
                        trigger: None,
                    },
                    Choice{
                        handle: "Sollte überprüft werden".to_string(),
                        output_text: "test2 output".to_string(),
                        condition: Some(level::ChoiceCondition::If(Condition{
                            name: "default_true".to_string(),
                            value: "true".to_string()
                        })),
                        trigger: Some(level::Trigger::SetCondition(Condition{
                            name: "default_true".to_string(),
                            value: "false".to_string()
                        }))
                    },
                    Choice{
                        handle: "Sollte nicht angezeigt werden".to_string(),
                        output_text: "Sollte nur angezeigt werden wenn default_true==false ist".to_string(),
                        condition: Some(level::ChoiceCondition::If(Condition { 
                            name: "default_true".to_string(), 
                            value: "false".to_string()
                        })),
                        trigger: None
                    },
                ],
        vec![
                        Condition{
                            name: "default_true".to_string(),
                            value: "true".to_string()
                        }
                    ]
    );
    //println!("{:#?}", test_availible_choices_level)
    println!("{:#?}", test_availible_choices_level.get_availible_chocies().clone());
    println!("Calling 'Handle Trigger' *******************");
    test_availible_choices_level = test_availible_choices_level.handle_trigger(
        level::Trigger::SetCondition(Condition{
            name: "default_true".to_string(),
            value: "false".to_string()
        }));
    println!("{:#?}", test_availible_choices_level.get_availible_chocies());
    
    utils::Printer::print(vec!["1. Go".to_string(), "2. Exit".to_string()], "100/100".to_string(), "1000".to_string(), "Testlocation".to_string());
}

fn print_ui() {
    // TODO: make width variabe, eather by getting the terminal width or with conf opt.
    // The current width is set to 50 by convention
    println!("|--------------------------------------------------|");
    hud();
    println!("|--------------------------------------------------|");

    // let test_choices = vec![
    //                 level::Choice{
    //                     handle: "Sollte sofort angezeigt werden".to_string(),
    //                     output_text: "test output".to_string(),
    //                     condition: None,
    //                     trigger: None,
    //                 },
    //                 Choice{
    //                     handle: "Sollte überprüft werden".to_string(),
    //                     output_text: "test2 output".to_string(),
    //                     condition: level::ChoiceCondition::If(Choice{
    //                         name: "default_true".to_string(),
    //                         value: "true".to_string()
    //                     }),
    //                     trigger: level::Trigger::Set(Choice{
    //                         name: "default_true".to_string(),
    //                         value: "false".to_string()
    //                     })
    //                 },
    //                 Choice{
    //                     handle: "Sollte nicht angezeigt werden",
    //                     output_text: "Sollte nur angezeigt werden wenn default_true==false ist",
    //                     condition: None,
    //                     trigger: None
    //                 },
    //             ]

    

}

fn hud() {
    println!("|Location: <unknown>                               |");
    println!("|hp: <unknown>                                     |");
    println!("|xp: <unknown>                                     |");
}

// fn level1() {
//     // TODO: dbg print player
//     let player = Player;
//     println!()

//     // print!("Du wachst auf.\n");
//     // loop {
//     //     print_ui();

//     //     println!("Dein schädel brummt\nEs ist gleißend hell.");
//     //     println!("[1] Umgucken\n[2] wieder einschlafen");

//     //     let mut input = String::new();
//     //     io::stdin()
//     //         .read_line(&mut input)
//     //         .expect("Failed to read line");
//     //     let input: u8 = input.trim()
//     //                         .parse()
//     //                         .expect("Bitte gebe die Nummer der gewünschten Aktion ein!");

//     //     if input == 1 {
//     //         println!("Du reibst dir die Augen und guckst dich um.\nDu liegst auf einer Wiese.\nDie Sonne scheint.\nEs ist warm.");
//     //     }
//     //     if input == 2 {
//     //         println!("Du versuchst wieder einzuschlafen.\nAuf Wiedersehen!");
//     //         process::exit(0x0100); // exit with return 0
//     //     }
//     //     else {
//     //         // FIXME: else runs besides having valid input?
//     //         println!("Bitte gib eine valide Ganzzahl der vorgeschlagenen Möglichkeiten ein.");
//     //     }

//     // }
// }
