enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature")
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}");
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }
        }
    }
}

fn main() {
    LaundryCycle::Cold.wash_laundry();
    let hot_cycle = LaundryCycle::Hot { temperature: 100 };
    hot_cycle.wash_laundry();

    let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();
}
//LaundryCycle::Cold---ispe/kispe call hau hai

// match the value

/*
impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature");
            }
            LaundryCycle::Hot { temperature } if *temperature == 100 => {
                println!("Running the laundry with exactly 100 degrees!");
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}");
            }
            LaundryCycle::Delicate(fabric_type) if fabric_type == "Silk" => {
                println!("Running the laundry delicately for Silk fabric");
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }
        }
    }
}

*/
