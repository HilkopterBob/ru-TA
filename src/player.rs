#[derive(Debug)]
pub struct Player {
    name: String,
    // FIXME: add uuid to entities
    level: i128,
    xp: i128,
    location: String,
    money: i128,
    // FIXME: inventory: Slice
}

impl Player {
    pub fn build(name: String) -> Self {
        let player = Player {
            name,
            level: 0,
            xp: 0,
            location: String::from("nirvana"),
            money: 0,
        };
        return player;
    }
}
