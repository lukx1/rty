use std::time;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Clone,Debug)]
pub struct Note {
    notes:u8,
    duration:i8,
}

impl Note {
    pub fn new(notes:u8,duration:i8) -> Note{
        Note {
            notes,
            duration,
        }
    }

    pub fn notes(&self) -> u8 {self.notes}
    pub fn duration(&self) -> i8 {self.duration}

    pub fn duration_exp(&self) -> u32 {
        let mut dur:i32 = 0;

        if self.duration < 0 {
            dur = -self.duration as i32;
        }
        else {
            dur = self.duration as i32;
        }

        let dur = 2.0f64.powi(dur) as u32;

        dur
    }

    pub fn duration_as_time(&self, base_duration:&time::Duration) -> time::Duration {
        let mult = self.duration_as_f64();

        base_duration.mul_f64(mult)
    }

    pub fn is_note_match(&self,note:Note) -> bool{
        self.notes == note.notes()
    }

    pub fn duration_as_f64(&self) -> f64 {
        2.0f64.powi(self.duration as i32)
    }
}

pub struct GameMap {
    name: String,
    shortcut_name:String,
    speed: time::Duration,
    content: Vec<Note>
}

/**
File template
Name
ShortCutName
Speed(BPM)
Content
*/

impl GameMap {

    fn read_line(reader:&mut BufReader<File>) -> String {
        let mut string = String::new();
        reader.read_line(&mut string).unwrap();
        string
    }

    fn parse_line_to_note(line:&str) -> Note {
        let mut notes = 0u8;
        let mut duration = 0i8;
        let mut i = 0u8;

        let mut read_duration = false;

        let mut dur_temp = String::new();
        let mut dur_neg = 1;

        for c in line.chars() {
            if !read_duration {
                match c {
                    '1' | '4' | '7' => notes |= 1,
                    '2' | '5' | '8' => notes |= 2,
                    '3' | '6' | '9' => notes |= 4,
                    '.' => read_duration = true,
                    _ => panic!(format!("Invalid char {} on line {}",c,line)),
                };
            }
            else {
                match c {
                    '-' => dur_neg = -1,
                    '0'...'9' => dur_temp.push(c),
                    ' '|'\r'|'\n'|'\t' => continue,
                    _ => panic!(format!("Invalid char {}",c)),
                };
            }

            i += 1;
        }

        match dur_temp.as_ref() {
            ""|"0" => duration = 1,
            "2" => duration += 2 ,
            "4" => duration += 3 ,
            "8" => duration += 4 ,
            "16" => duration += 5 ,
            "32" => duration += 6 ,
            "64" => duration += 7 ,
            "128" => duration += 8 ,
            _ => panic!(format!("Invalid duration {}",dur_temp)),
        }

        duration *= dur_neg;

        let note = Note::new(notes,duration);
        note
    }

    pub fn from_file(path:&str) -> GameMap {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        let name = GameMap::read_line(&mut reader).trim().to_string();
        let shortcut_name = GameMap::read_line(&mut reader).trim().to_string();
        let speed = GameMap::read_line(&mut reader).trim().to_string();

        let speed:u32= speed.parse().unwrap();
        let speed = (1.0/(speed as f64/60.0)*1000.0) as u64;
        let speed = time::Duration::from_millis(speed);

        let mut content = Vec::with_capacity(200);

        reader.lines()
            .into_iter()
            .for_each(|l|
                content.push(GameMap::parse_line_to_note(&l.unwrap().trim())));


        GameMap {
            name,
            shortcut_name,
            speed,
            content,
        }
    }

    pub fn get_name(&self) -> &str {&self.name}
    pub fn get_shortcut_name(&self) -> &str {&self.shortcut_name}
    pub fn get_speed(&self) -> &time::Duration {&self.speed}
    pub fn get_content(&self) -> &Vec<Note> {&self.content}
}