use crate::{entities::Entity, event::Event, planet::Planet, terminal};

pub mod earth;

fn game_loop(planet: Planet, mut entities: Vec<Box<dyn Entity>>) {
    print!("{}", planet.generate_banner());
    print!("{}", planet.generate_map(&entities));
    loop {
        let mut event_queue: Vec<Event> = vec![];
        for entity in entities.iter_mut() {
            entity.take_turn(&planet);
            event_queue.extend(entity.get_event());
        }
        loop {
            let mut reaction_event_queue: Vec<Event> = vec![];
            for event in &event_queue[..] {
                for entity in entities.iter_mut() {
                    reaction_event_queue.extend(entity.handle_event(&event));
                }
            }
            event_queue = reaction_event_queue;
            if event_queue.is_empty() {
                break;
            }
        }
        terminal::clear_screen();
        print!("{}", planet.generate_banner());
        print!("{}", planet.generate_map(&entities));
    }
}
