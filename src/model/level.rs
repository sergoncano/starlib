use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    FRAME_DURATION,
    graphics::renderer::render_game,
    model::{entity::Entity, event::Event, stage::Stage, tip::Tip},
};

pub struct Level<T> {
    stages: Vec<Stage>,
    entities: Vec<Box<dyn Entity<T>>>,
}

impl<T> Level<T> {
    pub fn build(stages: Vec<Stage>, entities: Vec<Box<dyn Entity<T>>>) -> Self {
        if stages.is_empty() {
            panic!("No stages were provided for the level!");
        }
        Level::<T> { stages, entities }
    }

    pub fn run(&mut self) -> i32 {
        let mut stage_i = 0;
        let mut delta_time = Duration::ZERO;
        let mut tip = Tip::new(String::from(""), Duration::ZERO, 0);
        let mut remaining_turn_time: Vec<Duration> =
            self.entities.iter().map(|e| e.get_turn_delay()).collect();
        loop {
            let ti = Instant::now();
            tip.ellapse(delta_time);
            let mut event_buffer = vec![];
            for i in 0..self.entities.len() {
                let mut time = *remaining_turn_time.get(i).unwrap();
                time = if time < delta_time {
                    Duration::ZERO
                } else {
                    time - delta_time
                };
                if time == Duration::ZERO {
                    let returned_events = self
                        .entities
                        .get_mut(i)
                        .unwrap()
                        .take_turn(&mut self.stages[stage_i]);
                    event_buffer.extend(returned_events);
                    time = self.entities.get(i).unwrap().get_turn_delay();
                }
                remaining_turn_time[i] = time;
            }
            loop {
                let mut new_event_buffer = vec![];
                for event in event_buffer {
                    match event {
                        Event::User(user_event) => {
                            for entity in self.entities.iter_mut() {
                                new_event_buffer.extend(
                                    entity.handle_event(&user_event, &mut self.stages[stage_i]),
                                );
                            }
                        }
                        Event::Tip(t) => {
                            tip = t.overlap(tip);
                        }
                        Event::ChangeStage(new_stage_i) => {
                            stage_i = new_stage_i;
                        }
                        Event::ExitLevel(exit_code) => {
                            return exit_code;
                        }
                    }
                }
                event_buffer = new_event_buffer;
                if event_buffer.is_empty() {
                    break;
                }
            }
            render_game(
                &self.stages[stage_i].map,
                &self.stages[stage_i].decorations,
                &self.entities.iter().map(|e| e.get_render()).collect(),
                if !(tip).has_expired() {
                    Some(tip.clone())
                } else {
                    None
                },
            );
            sleep(FRAME_DURATION);
            let tf = Instant::now();
            delta_time = tf - ti;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{Sprite, graphics::map::Map};

    use super::*;

    #[test]
    fn test_constructor() {
        let stage = Stage::build(Map::test_map(), vec![], vec![]);
        let _level: Level<i32> = Level::build(vec![stage], vec![]);
    }

    #[test]
    #[ignore]
    fn test_run() {
        const EXIT_CODE: i32 = 3;
        const EVENT_CODE: i32 = 2341;

        struct TestEntity {
            counter: i32,
        }
        impl TestEntity {
            fn new() -> Self {
                Self { counter: 0 }
            }
        }

        impl Entity<i32> for TestEntity {
            fn take_turn(&mut self, _stage: &mut Stage) -> Vec<Event<i32>> {
                if self.counter == 0 {
                    vec![Event::User(EVENT_CODE)]
                } else {
                    vec![Event::ExitLevel(EXIT_CODE)]
                }
            }
            fn handle_event(&mut self, event: &i32, _stage: &mut Stage) -> Vec<Event<i32>> {
                if event == &EVENT_CODE {
                    self.counter += 1;
                }
                vec![]
            }
            fn get_turn_delay(&self) -> std::time::Duration {
                Duration::from_secs(1)
            }
            fn get_render(&self) -> (crate::model::coords::Coords, crate::Sprite) {
                (
                    crate::model::coords::Coords::new(0, 0),
                    Sprite::build("o", 3),
                )
            }
        }

        let stage = Stage::build(Map::test_map(), vec![], vec![]);
        let mut level: Level<i32> = Level::build(vec![stage], vec![Box::new(TestEntity::new())]);
        assert_eq!(level.run(), EXIT_CODE);
    }
}
