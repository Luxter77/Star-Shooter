/*struct GLaserState {
    power: i32,
    start_x: i32,
    start_y: i32,
    angle: i16,
}

impl GLaserState {
    pub fn asaaaaaasasdsadasasasdasdaaaa //aaaaaaaaaaaaaaAAAAA
}*/

struct ShipAestetics { // I don't know how to write that one word so now we are just rolling with this
    color1: (u8, u8, u8, u8),
}

struct GShipState {
    Ship_id: i8,
    
    Hit_points: i32,
    alive: bool,
    
    Power: i32,
    Power_mod: i32,

    X_position: i32,
    X_speed: i32,
    Y_position: i32,
    Y_speed: i32,
    Z_position: i32,
    Z_speed: i32,
}

struct SnipperType {
    State: GShipState,
    
}