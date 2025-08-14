use crate::level;

#[derive(Debug)]
#[derive(Clone)]
pub struct Player {
    pub name: String,
    // FIXME: add uuid to entities
    pub health: i128,
    pub health_max: i128,
    pub level: i128,
    pub xp: i128,
    pub location: level::Level,
    pub money: i128,
    // FIXME: inventory: Slice
}

impl Player {
    pub fn build(name: String, location: level::Level) -> Self {
        // TODO: rewrite
        let player = Player {
            name,
            health: 100,
            health_max: 100,
            level: 0,
            xp: 0,
            location: location,
            money: 0,
        };
        return player;
    }
}
