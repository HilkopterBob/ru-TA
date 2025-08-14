
#[derive(Debug)]
#[derive(Clone)]
pub struct Level {
    pub name: String,        // The Level Name
    pub description: String, // The Description that will be shown to player
    pub choices: Vec<Choice>, // Choices/Actions the player can take/do in a level. Some of them may hold:
    pub conditions: Vec<Condition>,//      a trigger: a conditional that dictates if/when a choice should be shown/hidden
                           //                 eg: 'unlock_door' should only be shown if the player holds a key
                           //      an action: a function that gets executed if the choice is selected.
                           //                 this eather is one of our pre-implemented functions/methods
                           //                 like 'change_level', but can be arbitrary code too, like lambdas.
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Condition {
    pub name: String,
    pub value: String
}

impl Level {
    pub fn new(name: String, description: String, choices: Vec<Choice>, conditions: Vec<Condition>) -> Level {
        let level = Level{
            name,
            description,
            choices,
            conditions
        };
        return level;
    }

    // TODO: split up!
    pub fn get_availible_choices(self: &Self) -> Vec<Choice> {
        let mut available_choices: Vec<Choice> = Vec::new();
        
        // append choices with valid conditions
        for choice in self.choices.clone() {
            // println!("All Choices in level:");
            // println!("{:#?}", choice);

            // TODO: return early if choice has no condition

            /*
            pseudo:
            if level.conditions where level.conditions[n].name == choice.condition.name
            if level.conditions[n].value == choice.condition.value
                append choice to available_choices
            
             */
            for level_condition in self.conditions.clone() {
                match choice.condition.clone() {
                    Some(choice_condition_condition) => {
                        match choice_condition_condition {
                            ChoiceCondition::If(Condition {name: choice_condition_name, value: choice_condition_value}) => {
                                // if we are not comparing matching conditions: skip!
                                if choice_condition_name != level_condition.name {
                                    // log that we found unmatching conditions
                                    continue;
                                }
                                if choice_condition_value == level_condition.value {
                                    // log that we found matching condition sets!
                                    // we've found a condition that matches its name and value: its valid to print
                                    println!("Append because {choice_condition_value} == {} ", level_condition.value);
                                    available_choices.push(choice.clone())
                                }
                            },
                            ChoiceCondition::Not(Condition {name: choice_condition_name, value: choice_condition_value}) => {
                                // if we are not comparing matching conditions: skip!
                                if choice_condition_name != level_condition.name {
                                    // log that we found unmatching conditions
                                    continue;
                                }
                                if choice_condition_value != level_condition.value {
                                    // log that we found matching condition sets!
                                    // we've found a condition that matches its name and not matches its value: its valid to print
                                    println!("Append because {choice_condition_value} != {} ", level_condition.value);
                                    available_choices.push(choice.clone())
                                }
                            },
                            ChoiceCondition::And(_) => {
                                // log: this is currently unimplemented!
                                println!("*****found a ChoiceCondition that implements eather 'And' or 'Or'.*****");
                                println!("*****This is currently unimplemented behavior. Will Debug Print the Level now:*****");
                                println!("{:#?}", self);
                                println!("***** Will keep executing as if nothing happened! *****");
                                println!("********************************************************");
                            },
                            ChoiceCondition::Or(_) => {
                                // log: this is currently unimplemented!
                                println!("*****found a ChoiceCondition that implements eather 'And' or 'Or'.*****");
                                println!("*****This is currently unimplemented behavior. Will Debug Print the Level now:*****");
                                println!("{:#?}", self);
                                println!("***** Will keep executing as if nothing happened! *****");
                                println!("********************************************************");
                            },
                        }
                    },
                    None => {
                        println!("Append because None condition");
                        available_choices.push(choice.clone())
                    },

                // //if condition.name != choice.condition::If::
                // //early exit if not the right condition for choice
                // match choice.condition {
                //     ChoiceCondition::If => {
                //         if value.name != condition.name {continue}
                //     }
                // }
                }
            } 
        }
        println!("return!!!!!!!!!!!!!!!!!!!!!!!!");
        return available_choices;
    }

    // pub fn handle_choice(self: Self, choice: Choice) -> (String, Level) {
    //     if let Some(trigger) = choice.trigger {
    //         self = self.handle_trigger(trigger)
    //     }
    //     return (, self)

    // }
    
    pub fn handle_trigger(mut self: Self, trigger: &Trigger) -> Self {
        // TODO: log start of trigger handling
        match trigger {
            Trigger::SetCondition(condition) => {
                // TODO: log call to override existing condition
                self = self.set_condition(&condition);
            }
            Trigger::Call(string) => {
                println!("*****Found a Trigger that is variant 'Trigger::Call'.*****");
                println!("*****This is currently unimplemented behavior. Will Debug Print the Trigger now:*****");
                println!("{:#?}", self);
                println!("***** Will keep executing as if nothing happened! *****");
                println!("********************************************************");
            }
        }
        return self;

    }

    pub fn set_condition(mut self: Self, new_condition: &Condition) -> Self{
        // search through level until the level condition that has the same .name as new_condition is found
        // then change the .value of level condition to new_condition.value
        for level_condition in self.conditions.clone() {

            if level_condition.name != new_condition.name {continue;}
            
            if let Some(index) = self.conditions.iter().position(| x| *x == level_condition) {
                self.conditions.swap_remove(index);
                self.conditions.push(new_condition.clone());
            }
        }
        return self;
    }
}
/// ChoiceCondition Enum
/// The Choice Condition Enum is thought to be used for conditionals in Choices.
/// The ChoiceCondition Enum should support even complex requerements. For that it supports
/// nesting with itself in vectors over its own state.
#[derive(Clone)]
#[derive(Debug)]
pub enum ChoiceCondition {
    /// if is used if the condition is one single flag that should evaluate to 'true' with the enums content.
    /// eg. If the Choice to open a door should only be shown if the player has a key the ChoiceCondition is:
    /// 'ChoiceCondition::If(['has_key','true'])'   
    If(Condition),
    /// same as 'if' and 'not' but supports nesting with itself for longer, more complex conditionals.
    And(Vec<ChoiceCondition>),
    /// same as 'if' in reverse
    Not(Condition),
    /// same as and but evaluates to true the moment a nested ChoiceCondition evaluates to 'true'
    Or(Vec<ChoiceCondition>),
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Trigger{
    SetCondition(Condition),
    Call(String)
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Choice {
    /// the short text that is printed to the player on chaoice selection
    pub handle: String,
    /// the response text that gets printed when the player sellects this choice
    pub output_text: String,
    /// the condition the choice has when it should be shown/hidden, see: ChoiceCondition
    pub condition: Option<ChoiceCondition>,
    /// the trigger vec holds a function name and the parameters needed to run on choice selection on the player
    /// should be in ['func','arg1','arg2','arg3','argn']
    pub trigger: Option<Trigger>
}