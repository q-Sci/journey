use colored::Colorize;

use crate::type_text;

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
        type_text(&format!("\n<{}>\n-----\n  Crew: {}\n  Stability: {}%\n  Engines: {}\n  O2: {}%\n  Control unit: {}\n  Core reactor: {}\n-----\n<{}>\n",
            "Status".bold().cyan(),
            self.crew,
            self.stability,
            self.engines,
            self.o2,
            self.control_unit,
            self.core_reactor,
            "Status end".bold().cyan()
        ));
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
            type_text(&format!("Status update: {} reduced by {}%", component, value));
        } else if component == "crew" {
            type_text(&format!("Status update: {} crew members died", value));
        } else {
            match value {
                1 => type_text(&format!("Status update: 1 engine failed")),
                4 => type_text(&format!("Status update: all engines failed")),
                _ => type_text(&format!("Status update: {} engines failed", value)),
            }
        }
    }

    pub fn fail(&mut self, component: &str) {
        match component {
            "control unit" => self.control_unit = false,
            "core reactor" => self.core_reactor = false,
            &_ => unimplemented!(),
        }
        type_text(&format!("Status update: {} engine failed", component));
    }
}
