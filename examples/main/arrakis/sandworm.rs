use std::{collections::HashSet, time::Duration};

use rand::random_range;
use starlib::{Collider, Coords, Entity, Event, Sprite, model::movement::Movement};

use crate::user_event::UserEvent;

pub(crate) struct Sandworm {
    step: usize,
    can_eat: bool,
    player_coords: Coords,
    real_player_coords: Coords,
    stage: usize,
}

impl Sandworm {
    pub(crate) fn new() -> Self {
        Self {
            step: 0,
            player_coords: Coords::new(0, 0),
            real_player_coords: Coords::new(0, 0),
            can_eat: false,
            stage: 0,
        }
    }
}

const STILL_SAND: char = '.';
const MOVING_SAND: char = ',';
const WORM_FANG: char = '^';
const WORM_RADIUS: usize = 8;
const TURNS_WATING: usize = 5;
const TURNS_WARNING: usize = 4;
const TURNS_EATING: usize = 1;

impl Entity<UserEvent> for Sandworm {
    fn get_turn_delay(&self) -> std::time::Duration {
        Duration::from_millis(600)
    }

    fn get_render(&self) -> (Coords, starlib::Sprite) {
        (Coords::new(-3, -3), Sprite::new('?', -1))
    }

    fn take_turn(&mut self, stage: &mut starlib::Stage) -> Vec<starlib::Event<UserEvent>> {
        let sands = vec![MOVING_SAND, STILL_SAND];
        if self.step <= TURNS_WATING && self.can_eat {
            self.step += 1;
            self.player_coords = self.real_player_coords.clone();
            self.player_coords.x += random_range(3..=5)
        } else if self.step > TURNS_WATING && self.step < TURNS_WATING + TURNS_WARNING {
            self.step += 1;
            for coord in coord_radius(self.player_coords.clone(), WORM_RADIUS) {
                let new_sprite = Sprite::new(sands[self.step % 2], 1);
                if let Some(sprite) = stage.get_decoration(&coord)
                    && sprite.get_z_index() > 1
                {
                    stage.set_decoration(coord, sprite.overlap(new_sprite));
                } else {
                    stage.set_decoration(coord, new_sprite);
                }
            }
        } else if self.step > TURNS_WATING
            && self.step < TURNS_WATING + TURNS_WARNING + TURNS_EATING
        {
            self.step += 1;
            for coord in coord_radius(self.player_coords.clone(), WORM_RADIUS) {
                stage.set_decoration(coord.clone(), Sprite::new(WORM_FANG, 1));
                if &self.real_player_coords == &coord {
                    return vec![Event::ExitLevel(1)];
                }
                stage.set_collider(coord, Collider::try_from("----").unwrap());
            }
        } else if self.step == TURNS_WATING + TURNS_WARNING + TURNS_EATING {
            for coord in coord_radius(self.player_coords.clone(), WORM_RADIUS) {
                stage.set_decoration(coord, Sprite::new('?', -2));
            }
            self.step = 0;
        }
        vec![]
    }

    fn handle_event(
        &mut self,
        event: &UserEvent,
        _stage: &mut starlib::Stage,
    ) -> Vec<starlib::Event<UserEvent>> {
        match event {
            UserEvent::PlayerMoved(coords) => {
                self.real_player_coords = coords.clone();
                if self.step < TURNS_WATING {
                    self.player_coords = coords.clone();
                    self.player_coords.x += random_range(3..=5);
                }
                self.can_eat = !((self.real_player_coords.x < 25 && self.stage == 0)
                    || (self.real_player_coords.x > 90 && self.stage == 2));
            }
            UserEvent::ChangedStage(stage_i) => {
                self.stage = *stage_i;
                self.step = 0;
            }
            _ => (),
        }
        vec![]
    }
}

fn coord_radius(coords: Coords, radius: usize) -> Vec<Coords> {
    let mods = vec![
        Movement::Up,
        Movement::Down,
        Movement::Left,
        Movement::Right,
    ];
    let mut res = HashSet::new();
    res.insert(coords);
    for _ in 1..radius {
        let mut newres = res.clone();
        for c in res {
            for modifier in mods.clone() {
                newres.insert(c.clone().do_movement(&modifier));
            }
        }
        res = newres;
    }
    res.into_iter().collect()
}
