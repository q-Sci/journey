fn main() {
    struct Spaceship {
        crew: i32,
        stability: i32,
        engines: i32,
        o2: i32,
        control_unit: bool,
        core_reactor: bool,
    }

    let spaceship = Spaceship {
        crew: 250,
        stability: 100,
        engines: 4,
        o2: 100,
        control_unit: true,
        core_reactor: true,
    };

    fn status(spaceship: Spaceship) {
        println!("Status\n  Crew: {}\n  Stability: {}\n  engines: {}\n  o2: {}\n  Control unit: {}\n  Core reactor: {}\nStatus end",
            spaceship.crew,
            spaceship.stability,
            spaceship.engines,
            spaceship.o2,
            spaceship.control_unit,
            spaceship.core_reactor
        );
    }

    status(spaceship);
}
