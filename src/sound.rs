
extern crate winapi;

use crate::game_map::GameMap;
use crate::game_map::Note;

pub fn beep(freq:u32, time_ms:u32){
    use winapi::um::utilapiset;

    unsafe {
        utilapiset::Beep(freq,time_ms);
    }
}

pub fn beep_map(gm:&GameMap){
    gm.get_content().iter().for_each(
        |note|
        {
            //println!("{}", note.notes());
            beep(750,200);
            std::thread::sleep(note.duration_as_time(gm.get_speed()));
        }
    );
}