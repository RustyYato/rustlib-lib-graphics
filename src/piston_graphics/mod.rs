use piston_window::{Window, PistonWindow};

use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::OpenGL;

use std::clone::Clone;

pub mod app;
pub mod data;
// pub mod color;

use self::app::App;
use self::data::{Ptr, AppData, WindowData};

impl WindowData {
    fn new(window: PistonWindow) -> Self {
        Self { app_data: AppData::new(&window), window }
    }
}

impl AppData {
    fn new(window: &PistonWindow) -> Self {
        let size = window.size();
        let (screen_width, screen_height) = (size.width, size.height);

        Self {
            is_cursor_on: false,
            is_window_focus: false,
            screen_width, screen_height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            button_held: Vec::new()
        }
    }
}

pub fn build_window(name: &str, width: u32, height: u32) -> PistonWindow {
    WindowSettings::new(
            name,
            [width, height]
        )
        .opengl(OpenGL::V4_5)
        .exit_on_esc(true)
        .build()
        .unwrap()
}

#[macro_export]
macro_rules! deref {
    ($data: expr) => {
        &mut *$data.lock().unwrap()
    };
}

pub fn start<'a, T, E>(window: PistonWindow, mut app: T)
where T: App {
    // let mut events = Events::new(EventSettings::new());
    let mut _found = false;

    let data = Ptr::from(WindowData::new(window));
    app.set_window_data(data.clone());

    loop {
        let e = deref!(&data).window.next();

        if let None = e {
            return;
        }

        let e = e.unwrap();

        match e {
            Event::Custom(_a, _b) => {

            },
            Event::Loop(l) => {
                match l {
                    Loop::Render(r) => {
                        deref!(&data).window.draw_2d(&e, |c, g| app.render(&r, c, g));
                    },
                    Loop::Update(u) => {
                        for button in &deref!(&data).app_data.button_held {
                            match button {
                                &Button::Keyboard(key) => 
                                    app.handle_key_held(key),
                                &Button::Mouse(mouse_button) => 
                                    app.handle_mouse_held(mouse_button),
                                &Button::Controller(controller_button) => 
                                    app.handle_controller_held(controller_button)
                            }
                        }

                        app.update(&u);
                    },
                    Loop::AfterRender(ar) => {
                        app.post_render(&ar);
                    },
                    Loop::Idle(i) => {
                        // println!("idle time {:?}ms", _a.dt * 1000.0);
                        app.idle(&i);
                    }
                }
            }
            Event::Input(i) => {
                match i {
                    Input::Button(b) => {

                        let (x, y, contains) = {
                            let mut data = deref!(&data);
                            (data.app_data.mouse_x, data.app_data.mouse_y, 
                             data.app_data.button_held.contains(&b.button))
                        };
                        
                        if !contains {
                            match b.button {
                                Button::Keyboard(key) => 
                                    app.handle_key(key),
                                Button::Mouse(mouse_button) => 
                                    app.handle_mouse(mouse_button, x, y),
                                Button::Controller(controller_button) => 
                                    app.handle_controller(controller_button)
                            }
                        }

                        match b.state {
                            ButtonState::Press => {
                                if !contains {
                                    deref!(&data).app_data.button_held.push(b.button);
                                }
                            },
                            ButtonState::Release => {
                                if contains {
                                    let mut data = deref!(&data);
                                    let index = data.app_data.button_held.iter().position(|x| *x == b.button).unwrap();
                                    data.app_data.button_held.remove(index);
                                }
                            }
                        }
                    },
                    Input::Move(m) => {
                        if let Motion::MouseCursor(x, y) = m {
                            let mut data = deref!(&data);
                            data.app_data.mouse_x = x;
                            data.app_data.mouse_y = y;
                        }
                    },
                    Input::Resize(w, h) => {
                        let mut data = deref!(&data);
                        data.app_data.screen_width = w;
                        data.app_data.screen_height = h;
                    },
                    Input::Text(_t) => {

                    },
                    Input::Cursor(c) => {
                        deref!(&data).app_data.is_cursor_on = c;
                        app.handle_cursor(c);
                    },
                    Input::Focus(f) => {
                        deref!(&data).app_data.is_window_focus = f;
                        app.handle_focus(f);
                    },
                    Input::Close(c) => {
                        app.on_close(&c);
                    }
                }
            }
        }
    }
}
