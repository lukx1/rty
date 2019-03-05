
use std::thread;
use std::sync::mpsc;
use multiinput::*;

fn keyboard_loop(tx: std::sync::mpsc::Sender<RawEvent>){
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    loop {
        if let Some(event) = manager.get_event(){
            tx.send(event).unwrap();
        }
        else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

pub fn start_keyboard_reader() -> std::sync::mpsc::Receiver<RawEvent> {
    let (tx, rx) = mpsc::channel::<RawEvent>();

    thread::spawn( move || keyboard_loop(tx));

    rx
}
