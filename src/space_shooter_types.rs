#![allow(dead_code)]
pub struct Colour {
    r: u8, g: u8, b: u8, a: u8,
}

impl Colour {
    fn change_colour_to() {
        
    }
}

pub struct ShipAestetics { // I don't know how to write that one word so now we are just rolling with this
    c0: Colour,
    c1: Colour,
    c2: Colour,
    hat: u8,
}

pub struct PhisicsState {
    x_position: i32,
    x_speed: i32,
    x_tilt: i64,

    y_position: i32,
    y_speed: i32,
    y_tilt: i64,

    z_position: i32,
    z_speed: i32,
    z_titl: i64,
}

pub struct GShipState {
    ship_id: i8, 
    
    hit_points: i32,
    alive: bool,
    
    power: i32,
    power_mod: i32,
    
    phisics: PhisicsState,
}

pub struct SnipperType {
    state: GShipState,
}

pub struct DOLOR {
    
}