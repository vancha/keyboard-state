use input::event::{self, keyboard::KeyState, keyboard::KeyboardEventTrait,KeyboardEvent};
use input::Event::{Keyboard};
use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}
/* scancode -> key combinations for some important keys:  https://www.millisecond.com/support/docs/current/html/viewer.htm#https://www.millisecond.com/support/docs/current/html/language/scancodes.htm
 * 1 	ESC
2 	1
3 	2
4 	3
5 	4
6 	5
7 	6
8 	7
9 	8
10 	9
11 	0
16 	Q
17 	W
18 	E
19 	R
20 	T
21 	Y
22 	U
23 	I
24 	O
25 	P
28 	Enter
30 	A
31 	S
32 	D
33 	F
34 	G
35 	H
36 	J
37 	K
38 	L
44 	Z
45 	X
46 	C
47 	V
48 	B
49 	N
50 	M
57 	Space
59 	F1
60 	F2
61 	F3
62 	F4
63 	F5
64 	F6
65 	F7
66 	F8
67 	F9
68 	F10
*/
#[derive(Debug, Hash, Eq, PartialEq)]
enum Key {
    KEY_0,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
}
fn main() {
    let mut initial_key_states = HashMap::new();

    initial_key_states.insert(Key::KEY_0, KeyState::Released);
    initial_key_states.insert(Key::KEY_1, KeyState::Released);
    initial_key_states.insert(Key::KEY_2, KeyState::Released);
    initial_key_states.insert(Key::KEY_3, KeyState::Released);
    initial_key_states.insert(Key::KEY_4, KeyState::Released);
    initial_key_states.insert(Key::KEY_5, KeyState::Released);
    initial_key_states.insert(Key::KEY_6, KeyState::Released);
    initial_key_states.insert(Key::KEY_7, KeyState::Released);
    initial_key_states.insert(Key::KEY_8, KeyState::Released);
    initial_key_states.insert(Key::KEY_9, KeyState::Released);
    initial_key_states.insert(Key::KEY_A, KeyState::Released);
    initial_key_states.insert(Key::KEY_B, KeyState::Released);
    initial_key_states.insert(Key::KEY_C, KeyState::Released);
    initial_key_states.insert(Key::KEY_D, KeyState::Released);
    initial_key_states.insert(Key::KEY_E, KeyState::Released);
    initial_key_states.insert(Key::KEY_F, KeyState::Released);

    let mut keys_pressed = Arc::new(Mutex::new(initial_key_states));

    let mut keys_pressed2 = keys_pressed.clone();

    thread::spawn(move || {
        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();
        loop {
            input.dispatch().unwrap();

            for event in &mut input {
                match event {
                    //only interested in keyboard events
                    Keyboard(kb_event) => {
                        match kb_event {
                            KeyboardEvent::Key(key) => {
                                let t_keys = keys_pressed2.clone();
                                let mut keything = t_keys.lock().unwrap();
                                match key.key() {
                                    2 => {
                                        keything.remove(&Key::KEY_1);
                                        keything.insert(Key::KEY_1,key.key_state());
                                        },
                                    3 => {
                                        keything.remove(&Key::KEY_2);
                                        keything.insert(Key::KEY_2,key.key_state());
                                    }
                                    4 => {
                                        keything.remove(&Key::KEY_3);
                                        keything.insert(Key::KEY_3,key.key_state());
                                    }
                                    5 => {
                                        keything.remove(&Key::KEY_4);
                                        keything.insert(Key::KEY_4,key.key_state());
                                    }
                                    6 => {
                                        keything.remove(&Key::KEY_5);
                                        keything.insert(Key::KEY_5,key.key_state());
                                    }
                                    7 => {
                                        keything.remove(&Key::KEY_6);
                                        keything.insert(Key::KEY_6,key.key_state());
                                    }
                                    8 => {
                                        keything.remove(&Key::KEY_7);
                                        keything.insert(Key::KEY_7,key.key_state());
                                    }
                                    9 => {
                                        keything.remove(&Key::KEY_8);
                                        keything.insert(Key::KEY_8,key.key_state());
                                    }
                                    10 => {
                                        keything.remove(&Key::KEY_9);
                                        keything.insert(Key::KEY_9,key.key_state());
                                    }
                                    11 => {
                                        keything.remove(&Key::KEY_0);
                                        keything.insert(Key::KEY_0,key.key_state());
                                    }
                                    30 => {
                                        keything.remove(&Key::KEY_A);
                                        keything.insert(Key::KEY_A,key.key_state());
                                    } //A
                                    48 => {
                                        keything.remove(&Key::KEY_B);
                                        keything.insert(Key::KEY_B,key.key_state());
                                    } //B
                                    46 => {
                                        keything.remove(&Key::KEY_C);
                                        keything.insert(Key::KEY_C,key.key_state());
                                    } //C
                                    32 => {
                                        keything.remove(&Key::KEY_D);
                                        keything.insert(Key::KEY_D,key.key_state());
                                    } //D
                                    18 => {
                                        keything.remove(&Key::KEY_E);
                                        keything.insert(Key::KEY_E,key.key_state());
                                    } //E
                                    33 => {
                                        keything.remove(&Key::KEY_F);
                                        keything.insert(Key::KEY_F,key.key_state());
                                    } //F
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
       }
    });
    loop {
        let ex = keys_pressed.clone();
        let keystatething = ex.lock().unwrap();
        println!("");
        for key in keystatething.keys() {
            println!("{:?} =>{:?}", key, keystatething.get(key));
        }
        println!("");
    }
}
