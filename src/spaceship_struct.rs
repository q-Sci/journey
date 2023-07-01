pub struct Spaceship {
    pub crew: i32,
    pub stability: i32,
    pub engines: i32,
    pub o2: i32,
    pub control_unit: bool,
    pub core_reactor: bool,
}
impl Spaceship {
    pub fn status(&self) {
        println!("Status\n-----\n  Crew: {}\n  Stability: {}%\n  engines: {}\n  o2: {}%\n  Control unit: {}\n  Core reactor: {}\n-----\nStatus end",
            self.crew,
            self.stability,
            self.engines,
            self.o2,
            self.control_unit,
            self.core_reactor
        );
    }
    pub fn modify(&mut self, component: &str, value: i32) {
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

    pub fn fail(&mut self, component: &str) {
        match component {
            "control unit" => self.control_unit = false,
            "core reactor" => self.core_reactor = false,
            &_ => unimplemented!(),
        }
        println!("Status update: {} engine failed", component);
    }
}