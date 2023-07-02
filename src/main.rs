mod spaceship_struct;
mod type_text;

use spaceship_struct::Spaceship;
use type_text::type_text;

fn main() {
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
