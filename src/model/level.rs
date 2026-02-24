use std::time::SystemTime;

use crate::model::{entity::Entity, event::Event, stage::Stage};

pub struct Level<T> {
    stage: Stage,
    entities: Vec<Box<dyn Entity<T>>>,
}

impl<T> Level<T> {
    pub fn new(stage: Stage, entities: Vec<Box<dyn Entity<T>>>) -> Self {
        Level::<T> { stage, entities }
    }

    pub fn run(&mut self) -> i32 {
        let start_time = SystemTime::now();
        let mut turns: Vec<u128> = self.entities.iter().map(|_e| 0).collect();
        loop {
            let mut event_buffer = vec![];
            for i in 0..self.entities.len() {
                let turn = start_time
                    .elapsed()
                    .expect("system clock went backwards!")
                    .as_millis()
                    % self.entities.get(i).unwrap().get_turn_delay().as_millis();

                if &turn != turns.get(i).unwrap() {
                    let returned_events =
                        self.entities.get_mut(i).unwrap().take_turn(&mut self.stage);
                    event_buffer.extend(returned_events);
                    turns[i] = turn;
                }
            }
            loop {
                let mut new_event_buffer = vec![];
                for event in event_buffer {
                    match event {
                        Event::User(user_event) => {
                            for entity in self.entities.iter_mut() {
                                new_event_buffer
                                    .extend(entity.handle_event(&user_event, &mut self.stage));
                            }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_constructor() {
        let stage = Stage::build(
            String::from("Test stage"),
            vec!["...", "...", "..."]
                .iter()
                .map(|&s| String::from(s))
                .collect(),
            vec![],
            vec![],
        );
        let _level: Level<i32> = Level::new(stage, vec![]);
    }

    #[test]
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
        };

        let stage = Stage::build(
            String::from("Test stage"),
            vec!["...", "...", "..."]
                .iter()
                .map(|&s| String::from(s))
                .collect(),
            vec![],
            vec![],
        );
        let mut level: Level<i32> = Level::new(stage, vec![Box::new(TestEntity::new())]);
        assert_eq!(level.run(), EXIT_CODE);
    }
}
