use crate::{
    interface::entity::Entity, model::event::Event, stage::planet::Planet, util::renderer,
};

pub struct Level {
    planet: Planet,
    entities: Vec<Box<dyn Entity>>,
}

impl Level {
    pub fn new(planet: Planet, entities: Vec<Box<dyn Entity>>) -> Level {
        Level { planet, entities }
    }

    pub fn game_loop(self) {
        let planet = self.planet;
        let mut entities = self.entities;
        renderer::render_frame(&planet, &entities);
        let mut quit = false;
        while !quit {
            let mut event_queue: Vec<Event> = vec![];
            let mut tip: String = String::from("");
            let player = entities.get_mut(0).expect("No player entity found.");
            player.take_turn(&planet);
            event_queue.extend(player.get_event());
            for entity in entities.iter_mut() {
                for event in &event_queue[..] {
                    entity.handle_event(event);
                }
            }
            for (i, entity) in entities.iter_mut().enumerate() {
                if i == 0 {
                    continue;
                }
                entity.take_turn(&planet);
                event_queue.extend(entity.get_event());
            }
            loop {
                let mut next_event_queue: Vec<Event> = vec![];
                for event in &event_queue[..] {
                    match event {
                        Event::ExitLevel => quit = true,
                        Event::ShowTip(s) => tip = s.to_string(),
                        _ => {
                            for entity in entities.iter_mut() {
                                entity.handle_event(event);
                                next_event_queue.extend(entity.get_event());
                            }
                        }
                    }
                }
                if next_event_queue.is_empty() {
                    break;
                }
                event_queue = next_event_queue;
            }
            renderer::render_frame(&planet, &entities);
            if !tip.is_empty() {
                renderer::render_tip(tip);
            }
        }
    }
}
