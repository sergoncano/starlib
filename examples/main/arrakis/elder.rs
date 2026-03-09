use std::time::Duration;

use starlib::{Coords, Entity, Event, Sprite, Tip};

use crate::user_event::UserEvent;

pub(crate) struct Elder {
    coords: Coords,
    can_interact: bool,
    dialogue_stage: i32,
    is_in_stage: bool
}

impl Elder {
    pub(crate) fn new(coords: Coords) -> Self {
        Elder {
            coords,
            can_interact: false,
            dialogue_stage: 0,
            is_in_stage: true
        }
    }
}

impl Entity<UserEvent> for Elder {
    fn take_turn(&mut self, _stage: &mut starlib::Stage) -> Vec<Event<UserEvent>> {
        vec![]
    }

    fn get_turn_delay(&self) -> Duration {
        Duration::ZERO
    }

    fn get_render(&self) -> (starlib::Coords, starlib::Sprite) {
        (if self.is_in_stage { self.coords.clone() } else { Coords::new(-1, -1) }, Sprite::new('ඉ', 4))
    }
    
    fn handle_event(&mut self, event: &UserEvent, _stage: &mut starlib::Stage) -> Vec<Event<UserEvent>> {
        let mut res = vec![];
        match event {
            UserEvent::PlayerMoved(player_coords) => {
                if self.coords.distance(player_coords) == 1 {
                    self.can_interact = true;
                    res.push(Event::Tip(Tip::new("Press E to talk".to_string(), Duration::from_hours(1), 1)))
                } else {
                    self.can_interact = false;
                    res.push(Event::Tip(Tip::new("".to_string(), Duration::ZERO, 1)));
                }
            },
            UserEvent::PlayerInteracted => {
                if self.can_interact && self.is_in_stage {
                    let text = match self.dialogue_stage {
                        0 => "Oh? You want to leave the planet? You're in for a tough ride.",
                        1 => "Well, it's simple. You just need to walk east until you leave the desert.",
                        2 => "There's an escape pod there.",
                        3 => "...",
                        4 => "Oh! I forgot to mention the tricky part.",
                        5 => "The sands are full of gargantuan worms, if they catch you, you're done for.",
                        6 => "Hiding behind rocks won't do any good, they'll digest the rocks too.",
                        7 => "But it's also not a good idea to just run, you'll get slowed by the sand.",
                        8 => "...",
                        9 => "You want a tip?",
                        10 => "Don't die.",
                        _ => "...",
                    };
                    self.dialogue_stage += 1;
                    res.push(Event::Tip(Tip::new(text.to_string(), Duration::from_secs(30), 1)));
                }
            }
            UserEvent::ChangedStage(stage_i) => {
                match stage_i {
                    0 => self.is_in_stage = true,
                    _ => self.is_in_stage = false,
                }
            }
        }
        res
    }
}
