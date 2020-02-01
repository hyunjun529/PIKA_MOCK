struct Position {
    x: f32,
    y: f32
}

struct Object {
    pos: Position
}

pub mod game {
    pub fn init() {
        println!("init");
    }
    
    pub fn update () {
        println!("update");
    }

    pub fn addObejct() {
        println!("addObject");
    }
}

