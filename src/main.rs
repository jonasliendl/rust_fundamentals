use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {        
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {
    let mut orders: HashMap<i32, Car> = HashMap::new();
    let mut requested_orders: Vec<i32> = vec![];
    let mut current_order: i32 = if requested_orders.len() > 0 { requested_orders[0] } else { 1 };

    loop {
        println!("Create a car order (y/n)?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "y" {
            requested_orders.push(current_order);
            current_order = requested_orders.len() as i32;
        } else {
            break;
        }
    }

    for i in requested_orders {
        let car = car_factory(i, 0);
        orders.insert(i+1, car);
        println!("Produces car #{}", i+1);
    }

    let mut i: i32 = 1;
    loop {
        if i > orders.len() as i32 {
            break;
        }
        let car = orders.get(&i).unwrap();
        println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", i, car.age.0, car.roof, car.motor, car.color, car.age.1);
        i += 1;
    }
}
