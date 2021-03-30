// well place all of this into a seperate library, this exist
// to simply add interaction with grid movement.

pub enum Actors {
    Unit(Unit),
    Hazard,
}


pub enum UnitType {
    Player, 
    EnemyAi,
    FriendlyAi,
}

pub struct Unit {
    unit_type: UnitType
}