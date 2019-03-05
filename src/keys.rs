fn press_key_raw(vk: u8){
    use winapi::um::winuser;
    unsafe {
        winuser::keybd_event(vk, 0x45, winuser::KEYEVENTF_EXTENDEDKEY, 0);
        winuser::keybd_event(vk, 0x45, winuser::KEYEVENTF_EXTENDEDKEY | winuser::KEYEVENTF_KEYUP, 0);
    }
}

pub fn send_key(key: GameKey){
    press_key_raw(key.clone() as u8);
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum GameKey {
    Capslock = 0x14,
    Numlock = 0x90,
    ScrollLock = 0x91,
}

impl From<RealKey> for GameKey {
    fn from(rk: RealKey) -> Self {
        match rk {
            RealKey::Left => GameKey::Capslock,
            RealKey::Mid => GameKey::Numlock,
            RealKey::Right => GameKey::ScrollLock,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum RealKey {
    Left,
    Mid,
    Right,
}

impl From<GameKey> for RealKey {
    fn from(gk: GameKey) -> Self {
        match gk {
            GameKey::Capslock => RealKey::Left,
            GameKey::Numlock => RealKey::Mid,
            GameKey::ScrollLock => RealKey::Right,
        }
    }
}


fn get_key_state_raw(vk: i32) -> i16{
    use winapi::um::winuser;

    unsafe {
        winuser::GetKeyState(vk)
    }
}

pub fn get_key_state(key: GameKey) -> KeyState{
    let state = get_key_state_raw(key.clone() as i32);
    KeyState::new(
        state & 0x80 > 0 ,
        state & 0x1 > 0
    )
}

pub fn set_key(key: GameKey, state: bool) {
    let key_state = get_key_state(key.clone());
    if key_state.toggle != state {
        send_key(key.clone());
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct KeyState {
    pub pressed: bool,
    pub toggle: bool,
}

impl KeyState {
    pub fn new(pressed:bool , toggle:bool) -> KeyState {
        KeyState {
            pressed,
            toggle
        }
    }
}