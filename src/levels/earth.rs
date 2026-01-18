use crate::{coords::Coords, entities::{Entity, player::Player, rabbit::Rabbit}, event::Event, planet::Planet, planets::earth, terminal};

pub fn play() {
    let mut earth = earth::generate();
    game_loop(&mut earth);
}

fn game_loop(planet: &mut Planet) {
    let player: Box<dyn Entity> = Box::new(Player::new(Coords::new(3, 2), String::from("ඞ")));
    let rabbit: Box<dyn Entity> = Box::new(Rabbit::new(Coords::new(5, 4)));
    let mut entities = vec![player, rabbit];
    print!("{}", planet.generate_banner());
    print!("{}", planet.generate_map(&entities));
    loop {
        let mut event_queue: Vec<Event> = vec![];
        for entity in entities.iter_mut() {
            entity.take_turn(&planet);
            event_queue.extend(entity.get_event());
        }
        for event in event_queue {
            for entity in entities.iter_mut() {
                entity.handle_event(&event);
            }
        }
        terminal::clear_screen();
        print!("{}", planet.generate_banner());
        print!("{}", planet.generate_map(&entities));
    }

}
