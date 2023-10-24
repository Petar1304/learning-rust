struct Sedan;
struct SUV;

// impl Sedan {
//     fn drive(&self) {
//         println!("Sedan is driving");
//     }
// }

// impl SUV {
//     fn drive(&self) {
//         println!("SUV is driving");
//     }
// }

/**
 * dyn for dynamic dispatch
 * impl for static dispatch
*/
// fn road_trip(vehicle: &dyn Vehicle) {
fn road_trip(vehicle: &impl Vehicle) {
    vehicle.drive();
}

pub trait Vehicle {
    fn drive(&self);
}

impl Vehicle for Sedan {
    fn drive(&self) {
        println!("Sedan is driving");
    }
}

impl Vehicle for SUV {
    fn drive(&self) {
        println!("SUV is driving");
    }
}

pub fn run() {
    let car = Sedan;
    road_trip(&car);
    let suv = SUV;
    road_trip(&suv);
}