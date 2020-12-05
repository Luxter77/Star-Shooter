#![allow(dead_code)]
pub struct Colour {
    r: u8, g: u8, b: u8, a: u8,
}

impl Colour {
    fn change_colour_to() {
        
    }
}

pub struct ShipAestetics { // I don't know how to write that one word so now we are just rolling with this
    C0: Colour,
    C1: Colour,
    C2: Colour,
    Hat: u8,
}

pub struct PhisicsState {
    X_position: i32, X_speed: i32, X_tilt: i64,
    Y_position: i32, Y_speed: i32, Y_tilt: i64,
    Z_position: i32, Z_speed: i32, Z_titl: i64,
}

pub struct GShipState {
    Ship_id: i8,
    
    Hit_points: i32,
    alive: bool,
    
    Power: i32,
    Power_mod: i32,

    phisics: PhisicsState,
}

pub struct SnipperType {
    State: GShipState,
}

pub struct DOLOR {
    
}