use crate::keys;
use crate::keyboard;
use multiinput::*;
use std::sync::mpsc;
use std::time;

extern crate stopwatch;

use stopwatch::Stopwatch;
use std::collections::HashSet;
use crate::game_map::GameMap;
use crate::keys::GameKey;
use crate::game_map::Note;
use std::thread;
use std::rc::Rc;

fn flash() {
    use crate::keys::*;

    for _i in 0..3 {
        set_key(GameKey::ScrollLock, true);
        set_key(GameKey::Numlock, true);
        set_key(GameKey::Capslock, true);

        std::thread::sleep(std::time::Duration::from_millis(100));

        set_key(GameKey::ScrollLock, false);
        set_key(GameKey::Numlock, false);
        set_key(GameKey::Capslock, false);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

pub struct Game {
    banned_keys: Vec<KeyId>,
    rx: mpsc::Receiver<RawEvent>,
    sw: Stopwatch,
    tick_rate: time::Duration,
    keys_pressed: HashSet<keys::RealKey>,
    run: bool,
    game_map: Option<GameMap>,
    act_turn: bool,
    cur_line: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            banned_keys: vec![KeyId::CapsLock, KeyId::Numlock, KeyId::ScrollLock],
            sw: Stopwatch::new(),
            rx: keyboard::start_keyboard_reader(),
            tick_rate: time::Duration::from_millis(1000),
            keys_pressed: HashSet::with_capacity(3),
            run: false,
            game_map: None,
            act_turn: false,
            cur_line: 0,
        }
    }

    fn key_id_to_game_key(&self, key_id: &KeyId) -> Option<keys::GameKey> {
        use keys::GameKey;

        match key_id {
            KeyId::ScrollLock => Some(GameKey::ScrollLock),
            KeyId::Numlock => Some(GameKey::Numlock),
            KeyId::CapsLock => Some(GameKey::Capslock),
            _ => None
        }
    }

    fn clear_keys_pressed(&mut self) {
        self.keys_pressed.clear();
    }

    fn handle_real_ley(&mut self, rk: keys::RealKey) {
        self.keys_pressed.insert(rk);
    }

    fn keys_pressed_as_u8(&self) -> u8 {
        let mut kp = 0u8;

        self.keys_pressed.iter().for_each(|k|
            match k {
                keys::RealKey::Left => kp |= 1,
                keys::RealKey::Mid => kp |= 2,
                keys::RealKey::Right => kp |= 4,
            }
        );

        kp
    }

    fn failed_turn(&mut self) {
        println!("Failed");
        self.stop();
    }

    fn show_lights(&self, note: &Note) {
        use keys::set_key;
        use GameKey::*;

        let line = note.notes();

        set_key(Capslock, line & 1 > 0);
        set_key(Numlock, line & 2 > 0);
        set_key(ScrollLock, line & 4 > 0);
    }

    /*fn show_next_line(&mut self) -> bool {
        use keys::{set_key};

        if  self.cur_line + 1 >= self.game_map.as_ref().unwrap().get_content().len() as u32 {
            return false;
        }

        self.cur_line += 1;

        let line = self.game_map.as_ref().unwrap().get_content()[self.cur_line as usize].notes();

        println!("Line : {}",line);

        set_key(GameKey::Capslock, line & 1 > 0);
        set_key(GameKey::Numlock, line & 2 > 0);
        set_key(GameKey::ScrollLock, line & 4 > 0);

        return true;
    }*/

    fn win(&mut self) {
        println!("Win");
        self.stop();
    }

    fn are_keys_valid(&mut self, note: &Note) -> bool {
        self.keys_pressed_as_u8() == note.notes()
    }

    /*fn logic_tick(&mut self){
        if self.do_keys_match(self.get_cur_line()) {
            if !self.show_next_line() {
                self.win();
            }
        }
        else {
            self.failed_turn();
        }
    }*/

    fn handle_key_press(&mut self, key: &KeyId) {
        if self.banned_keys.contains(&key) {
            keys::send_key(self.key_id_to_game_key(&key).unwrap());
            return;
        }

        if let Some(game_key) = self.key_id_to_game_key(key) {
            self.handle_real_ley(game_key.into());
        }
    }

    pub fn stop(&mut self) {
        self.run = false;
    }


    pub fn play(&mut self, game_map: GameMap) {
        self.game_map = Some(game_map);

        self.start();
    }

    fn get_game_map(&self) -> &GameMap {
        self.game_map.as_ref().expect("Can't get map - no map is selected")
    }

    fn shutdown(&mut self) {}

    fn start(&mut self) {
        for i in 0..self.game_map.as_ref().unwrap().get_content().len() {
            if !self.run {
                self.shutdown();
                return;
            }

            let note = Rc::new(&self.game_map.as_ref().unwrap().get_content()[i]);
            {
                self.show_lights(&note);
                self.get_ks_in_period(self.game_map.as_ref().unwrap().get_speed().clone());
            }
            if !self.are_keys_valid(&self.game_map.as_ref().unwrap().get_content()[i].clone()){
                self.failed_turn()
            }
        }

        self.win();

        /*while note_index < gm.get_content().len() {
            let note = gm.get_content().get(note_index).unwrap();
            note_index += 1;
        }*/
    }

    ///
    /// Freezes
    ///
    fn get_ks_in_period(&mut self, period: time::Duration) {
        self.keys_pressed.clear();

        thread::sleep(period);

        while let Ok(key) = self.rx.try_recv() {
            if let RawEvent::KeyboardEvent(_, key_id, State::Released) = key {
                self.handle_key_press(&key_id);
            }
        }
    }

    /*fn start(&mut self){
        self.sw.start();
        self.run = true;
        while self.run {
            if self.sw.elapsed() > self.tick_rate {
                self.logic_tick();
                self.sw.restart();
            }

            if !self.run {
                break;
            }

            let wait_time = self.tick_rate - self.sw.elapsed();

            let key = self.rx.recv_timeout(wait_time);

            match key {
                Err(_) => continue,
                _ => {}
            }

            let key = key.unwrap();

            if let RawEvent::KeyboardEvent(_,key_id,State::Released) = key {
                self.handle_key_press(&key_id);
            }
        }
    }*/
}