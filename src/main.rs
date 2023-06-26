fn main() {
    struct Spaceship {
        crew: i32,
        stability: i32,
        engines: i32,
        o2: i32,
        control_unit: bool,
        core_reactor: bool,
    }
    impl Spaceship {
        fn status(&self) {
            println!("Status\n-----\n  Crew: {}\n  Stability: {}%\n  engines: {}\n  o2: {}%\n  Control unit: {}\n  Core reactor: {}\n-----\nStatus end",
                self.crew,
                self.stability,
                self.engines,
                self.o2,
                self.control_unit,
                self.core_reactor
            );
        }
        fn modify(&mut self, component: &str, value: i32) {
            match component {
                "crew" => self.crew -= value,
                "stability" => self.stability -= value,
                "engines" => self.engines -= value,
                "o2" => self.o2 -= value,
                &_ => unimplemented!(),
            };

            if component == "stability" || component == "oxygen" {
                println!("Status update: {} reduced by {}%", component, value);
            } else if component == "crew" {
                println!("Status update: {} crew members died", value);
            } else {
                match value {
                    1 => println!("Status update: 1 engine failed"),
                    4 => println!("Status update: all engines failed"),
                    _ => println!("Status update: {} engines failed", value),
                }
            }
        }

        fn fail(&mut self, component: &str) {
            match component {
                "control unit" => self.control_unit = false,
                "core reactor" => self.core_reactor = false,
                &_ => unimplemented!(),
            }
            println!("Status update: {} engine failed", component);
        }
    }

    let mut spaceship = Spaceship {
        crew: 250,
        stability: 100,
        engines: 4,
        o2: 100,
        control_unit: true,
        core_reactor: true,
    };

    spaceship.status();
}
