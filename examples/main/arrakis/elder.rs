use std::time::Duration;

use starlib::{Coords, Entity, Event, Sprite, Tip};

use crate::user_event::UserEvent;

pub(crate) struct Elder {
    coords: Coords,
    can_interact: bool,
    dialogue_stage: i32,
    current_stage: usize,
}

impl Elder {
    pub(crate) fn new(coords: Coords) -> Self {
        Elder {
            coords,
            can_interact: false,
            dialogue_stage: 0,
            current_stage: 0,
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
        (
            if self.current_stage == 0 {
                self.coords.clone()
            } else {
                Coords::new(-1, -1)
            },
            Sprite::new('ඉ', 4),
        )
    }

    fn handle_event(
        &mut self,
        event: &UserEvent,
        _stage: &mut starlib::Stage,
    ) -> Vec<Event<UserEvent>> {
        let mut res = vec![];
        match event {
            UserEvent::PlayerMoved(player_coords) => {
                if self.coords.distance(player_coords) == 1 && self.current_stage == 0 {
                    self.can_interact = true;
                    res.push(Event::Tip(Tip::new(
                        "Press E to talk".to_string(),
                        Duration::from_hours(1),
                        1,
                    )))
                } else {
                    self.can_interact = false;
                    res.push(Event::Tip(Tip::new("".to_string(), Duration::ZERO, 1)));
                }
            }
            UserEvent::PlayerInteracted => {
                if self.can_interact && self.current_stage == 0 {
                    let text = match self.dialogue_stage {
                        0 => "Oh? You want to leave the planet?",
                        1 => {
                            "Well, it's simple. You just need to walk east until you leave the desert."
                        }
                        2 => "There's an escape pod there.",
                        3 => "...",
                        4 => "Oh! I forgot to mention the tricky part.",
                        5 => {
                            "The sands are full of gargantuan worms, if they catch you, you're done for."
                        }
                        6 => "Hiding behind rocks won't do any good, they'll digest the rocks too.",
                        7 => {
                            "But it's also not a good idea to just run, you'll get slowed by the sand."
                        }
                        8 => "...",
                        9 => "You want a tip?",
                        10 => "Someone once told me baiting the worms is a good idea",
                        11 => "but the person who told me is now dead.",
                        12 => "In the end, the choice is yours.",
                        13 => "...",
                        _ => "Try not to die.",
                    };
                    self.dialogue_stage += 1;
                    res.push(Event::Tip(Tip::new(
                        text.to_string(),
                        Duration::from_secs(30),
                        1,
                    )));
                }
            }
            UserEvent::ChangedStage(stage_i) => {
                self.current_stage = *stage_i;
            }
        }
        res
    }
}
