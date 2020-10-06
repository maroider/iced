use iced_native::{
    keyboard::{self, KeyCode, ModifiersState},
    mouse, window, Event, Point,
};

/// Converts a winit window event into an iced event.
pub fn window_event(
    event: &winit::event::WindowEvent<'_>,
    scale_factor: f64,
    modifiers: winit::event::ModifiersState,
) -> Option<Event> {
    use winit::event::WindowEvent;

    match event {
        WindowEvent::Resized(new_size) => {
            let logical_size = new_size.to_logical(scale_factor);

            Some(Event::Window(window::Event::Resized {
                width: logical_size.width,
                height: logical_size.height,
            }))
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            let logical_size = new_inner_size.to_logical(scale_factor);

            Some(Event::Window(window::Event::Resized {
                width: logical_size.width,
                height: logical_size.height,
            }))
        }
        WindowEvent::CursorMoved { position, .. } => {
            let position = position.to_logical::<f64>(scale_factor);

            Some(Event::Mouse(mouse::Event::CursorMoved {
                x: position.x as f32,
                y: position.y as f32,
            }))
        }
        WindowEvent::MouseInput { button, state, .. } => {
            let button = mouse_button(*button);

            Some(Event::Mouse(match state {
                winit::event::ElementState::Pressed => mouse::Event::ButtonPressed(button),
                winit::event::ElementState::Released => mouse::Event::ButtonReleased(button),
            }))
        }
        WindowEvent::MouseWheel { delta, .. } => match delta {
            winit::event::MouseScrollDelta::LineDelta(delta_x, delta_y) => {
                Some(Event::Mouse(mouse::Event::WheelScrolled {
                    delta: mouse::ScrollDelta::Lines {
                        x: *delta_x,
                        y: *delta_y,
                    },
                }))
            }
            winit::event::MouseScrollDelta::PixelDelta(position) => {
                Some(Event::Mouse(mouse::Event::WheelScrolled {
                    delta: mouse::ScrollDelta::Pixels {
                        x: position.x as f32,
                        y: position.y as f32,
                    },
                }))
            }
        },
        WindowEvent::ReceivedCharacter(c) if !is_private_use_character(*c) => {
            Some(Event::Keyboard(keyboard::Event::CharacterReceived(*c)))
        }
        WindowEvent::KeyboardInput {
            input:
                winit::event::KeyboardInput {
                    virtual_keycode: Some(virtual_keycode),
                    state,
                    ..
                },
            ..
        } => {
            if let Some(key_code) = key_code(*virtual_keycode) {
                let modifiers = modifiers_state(modifiers);

                Some(Event::Keyboard(match state {
                    winit::event::ElementState::Pressed => keyboard::Event::KeyPressed {
                        key_code,
                        modifiers,
                    },
                    winit::event::ElementState::Released => keyboard::Event::KeyReleased {
                        key_code,
                        modifiers,
                    },
                }))
            } else {
                None
            }
        }
        WindowEvent::ModifiersChanged(new_modifiers) => Some(Event::Keyboard(
            keyboard::Event::ModifiersChanged(modifiers_state(*new_modifiers)),
        )),
        WindowEvent::HoveredFile(path) => {
            Some(Event::Window(window::Event::FileHovered(path.clone())))
        }
        WindowEvent::DroppedFile(path) => {
            Some(Event::Window(window::Event::FileDropped(path.clone())))
        }
        WindowEvent::HoveredFileCancelled => Some(Event::Window(window::Event::FilesHoveredLeft)),
        _ => None,
    }
}

pub fn mouse_interaction(interaction: mouse::Interaction) -> winit::window::CursorIcon {
    use mouse::Interaction;

    match interaction {
        Interaction::Idle => winit::window::CursorIcon::Default,
        Interaction::Pointer => winit::window::CursorIcon::Hand,
        Interaction::Working => winit::window::CursorIcon::Progress,
        Interaction::Grab => winit::window::CursorIcon::Grab,
        Interaction::Grabbing => winit::window::CursorIcon::Grabbing,
        Interaction::Crosshair => winit::window::CursorIcon::Crosshair,
        Interaction::Text => winit::window::CursorIcon::Text,
        Interaction::ResizingHorizontally => winit::window::CursorIcon::EwResize,
        Interaction::ResizingVertically => winit::window::CursorIcon::NsResize,
    }
}

/// Converts a `MouseButton` from [`winit`] to an [`iced_native`] mouse button.
///
/// [`winit`]: https://github.com/rust-windowing/winit
/// [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
pub fn mouse_button(mouse_button: winit::event::MouseButton) -> mouse::Button {
    match mouse_button {
        winit::event::MouseButton::Left => mouse::Button::Left,
        winit::event::MouseButton::Right => mouse::Button::Right,
        winit::event::MouseButton::Middle => mouse::Button::Middle,
        winit::event::MouseButton::Other(other) => mouse::Button::Other(other),
    }
}

/// Converts some `ModifiersState` from [`winit`] to an [`iced_native`]
/// modifiers state.
///
/// [`winit`]: https://github.com/rust-windowing/winit
/// [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
pub fn modifiers_state(modifiers: winit::event::ModifiersState) -> ModifiersState {
    ModifiersState {
        shift: modifiers.shift(),
        control: modifiers.ctrl(),
        alt: modifiers.alt(),
        logo: modifiers.logo(),
    }
}

/// Converts a physical cursor position to a logical `Point`.
pub fn cursor_position(position: winit::dpi::PhysicalPosition<f64>, scale_factor: f64) -> Point {
    let logical_position = position.to_logical(scale_factor);

    Point::new(logical_position.x, logical_position.y)
}

/// Converts a `VirtualKeyCode` from [`winit`] to an [`iced_native`] key code.
///
/// [`winit`]: https://github.com/rust-windowing/winit
/// [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
pub fn key_code(virtual_keycode: winit::event::VirtualKeyCode) -> Option<KeyCode> {
    match virtual_keycode {
        winit::event::VirtualKeyCode::Key1 => Some(KeyCode::Key1),
        winit::event::VirtualKeyCode::Key2 => Some(KeyCode::Key2),
        winit::event::VirtualKeyCode::Key3 => Some(KeyCode::Key3),
        winit::event::VirtualKeyCode::Key4 => Some(KeyCode::Key4),
        winit::event::VirtualKeyCode::Key5 => Some(KeyCode::Key5),
        winit::event::VirtualKeyCode::Key6 => Some(KeyCode::Key6),
        winit::event::VirtualKeyCode::Key7 => Some(KeyCode::Key7),
        winit::event::VirtualKeyCode::Key8 => Some(KeyCode::Key8),
        winit::event::VirtualKeyCode::Key9 => Some(KeyCode::Key9),
        winit::event::VirtualKeyCode::Key0 => Some(KeyCode::Key0),
        winit::event::VirtualKeyCode::A => Some(KeyCode::A),
        winit::event::VirtualKeyCode::B => Some(KeyCode::B),
        winit::event::VirtualKeyCode::C => Some(KeyCode::C),
        winit::event::VirtualKeyCode::D => Some(KeyCode::D),
        winit::event::VirtualKeyCode::E => Some(KeyCode::E),
        winit::event::VirtualKeyCode::F => Some(KeyCode::F),
        winit::event::VirtualKeyCode::G => Some(KeyCode::G),
        winit::event::VirtualKeyCode::H => Some(KeyCode::H),
        winit::event::VirtualKeyCode::I => Some(KeyCode::I),
        winit::event::VirtualKeyCode::J => Some(KeyCode::J),
        winit::event::VirtualKeyCode::K => Some(KeyCode::K),
        winit::event::VirtualKeyCode::L => Some(KeyCode::L),
        winit::event::VirtualKeyCode::M => Some(KeyCode::M),
        winit::event::VirtualKeyCode::N => Some(KeyCode::N),
        winit::event::VirtualKeyCode::O => Some(KeyCode::O),
        winit::event::VirtualKeyCode::P => Some(KeyCode::P),
        winit::event::VirtualKeyCode::Q => Some(KeyCode::Q),
        winit::event::VirtualKeyCode::R => Some(KeyCode::R),
        winit::event::VirtualKeyCode::S => Some(KeyCode::S),
        winit::event::VirtualKeyCode::T => Some(KeyCode::T),
        winit::event::VirtualKeyCode::U => Some(KeyCode::U),
        winit::event::VirtualKeyCode::V => Some(KeyCode::V),
        winit::event::VirtualKeyCode::W => Some(KeyCode::W),
        winit::event::VirtualKeyCode::X => Some(KeyCode::X),
        winit::event::VirtualKeyCode::Y => Some(KeyCode::Y),
        winit::event::VirtualKeyCode::Z => Some(KeyCode::Z),
        winit::event::VirtualKeyCode::Escape => Some(KeyCode::Escape),
        winit::event::VirtualKeyCode::F1 => Some(KeyCode::F1),
        winit::event::VirtualKeyCode::F2 => Some(KeyCode::F2),
        winit::event::VirtualKeyCode::F3 => Some(KeyCode::F3),
        winit::event::VirtualKeyCode::F4 => Some(KeyCode::F4),
        winit::event::VirtualKeyCode::F5 => Some(KeyCode::F5),
        winit::event::VirtualKeyCode::F6 => Some(KeyCode::F6),
        winit::event::VirtualKeyCode::F7 => Some(KeyCode::F7),
        winit::event::VirtualKeyCode::F8 => Some(KeyCode::F8),
        winit::event::VirtualKeyCode::F9 => Some(KeyCode::F9),
        winit::event::VirtualKeyCode::F10 => Some(KeyCode::F10),
        winit::event::VirtualKeyCode::F11 => Some(KeyCode::F11),
        winit::event::VirtualKeyCode::F12 => Some(KeyCode::F12),
        winit::event::VirtualKeyCode::F13 => Some(KeyCode::F13),
        winit::event::VirtualKeyCode::F14 => Some(KeyCode::F14),
        winit::event::VirtualKeyCode::F15 => Some(KeyCode::F15),
        winit::event::VirtualKeyCode::F16 => Some(KeyCode::F16),
        winit::event::VirtualKeyCode::F17 => Some(KeyCode::F17),
        winit::event::VirtualKeyCode::F18 => Some(KeyCode::F18),
        winit::event::VirtualKeyCode::F19 => Some(KeyCode::F19),
        winit::event::VirtualKeyCode::F20 => Some(KeyCode::F20),
        winit::event::VirtualKeyCode::F21 => Some(KeyCode::F21),
        winit::event::VirtualKeyCode::F22 => Some(KeyCode::F22),
        winit::event::VirtualKeyCode::F23 => Some(KeyCode::F23),
        winit::event::VirtualKeyCode::F24 => Some(KeyCode::F24),
        winit::event::VirtualKeyCode::Snapshot => Some(KeyCode::Snapshot),
        winit::event::VirtualKeyCode::Scroll => Some(KeyCode::Scroll),
        winit::event::VirtualKeyCode::Pause => Some(KeyCode::Pause),
        winit::event::VirtualKeyCode::Insert => Some(KeyCode::Insert),
        winit::event::VirtualKeyCode::Home => Some(KeyCode::Home),
        winit::event::VirtualKeyCode::Delete => Some(KeyCode::Delete),
        winit::event::VirtualKeyCode::End => Some(KeyCode::End),
        winit::event::VirtualKeyCode::PageDown => Some(KeyCode::PageDown),
        winit::event::VirtualKeyCode::PageUp => Some(KeyCode::PageUp),
        winit::event::VirtualKeyCode::Left => Some(KeyCode::Left),
        winit::event::VirtualKeyCode::Up => Some(KeyCode::Up),
        winit::event::VirtualKeyCode::Right => Some(KeyCode::Right),
        winit::event::VirtualKeyCode::Down => Some(KeyCode::Down),
        winit::event::VirtualKeyCode::Back => Some(KeyCode::Backspace),
        winit::event::VirtualKeyCode::Return => Some(KeyCode::Enter),
        winit::event::VirtualKeyCode::Space => Some(KeyCode::Space),
        winit::event::VirtualKeyCode::Compose => Some(KeyCode::Compose),
        winit::event::VirtualKeyCode::Caret => Some(KeyCode::Caret),
        winit::event::VirtualKeyCode::Numlock => Some(KeyCode::Numlock),
        winit::event::VirtualKeyCode::Numpad0 => Some(KeyCode::Numpad0),
        winit::event::VirtualKeyCode::Numpad1 => Some(KeyCode::Numpad1),
        winit::event::VirtualKeyCode::Numpad2 => Some(KeyCode::Numpad2),
        winit::event::VirtualKeyCode::Numpad3 => Some(KeyCode::Numpad3),
        winit::event::VirtualKeyCode::Numpad4 => Some(KeyCode::Numpad4),
        winit::event::VirtualKeyCode::Numpad5 => Some(KeyCode::Numpad5),
        winit::event::VirtualKeyCode::Numpad6 => Some(KeyCode::Numpad6),
        winit::event::VirtualKeyCode::Numpad7 => Some(KeyCode::Numpad7),
        winit::event::VirtualKeyCode::Numpad8 => Some(KeyCode::Numpad8),
        winit::event::VirtualKeyCode::Numpad9 => Some(KeyCode::Numpad9),
        winit::event::VirtualKeyCode::AbntC1 => Some(KeyCode::AbntC1),
        winit::event::VirtualKeyCode::AbntC2 => Some(KeyCode::AbntC2),
        winit::event::VirtualKeyCode::NumpadAdd => Some(KeyCode::Add),
        winit::event::VirtualKeyCode::Apostrophe => Some(KeyCode::Apostrophe),
        winit::event::VirtualKeyCode::Apps => Some(KeyCode::Apps),
        winit::event::VirtualKeyCode::At => Some(KeyCode::At),
        winit::event::VirtualKeyCode::Ax => Some(KeyCode::Ax),
        winit::event::VirtualKeyCode::Backslash => Some(KeyCode::Backslash),
        winit::event::VirtualKeyCode::Calculator => Some(KeyCode::Calculator),
        winit::event::VirtualKeyCode::Capital => Some(KeyCode::Capital),
        winit::event::VirtualKeyCode::Colon => Some(KeyCode::Colon),
        winit::event::VirtualKeyCode::Comma => Some(KeyCode::Comma),
        winit::event::VirtualKeyCode::Convert => Some(KeyCode::Convert),
        winit::event::VirtualKeyCode::NumpadDecimal => Some(KeyCode::Decimal),
        winit::event::VirtualKeyCode::NumpadDivide => Some(KeyCode::Divide),
        winit::event::VirtualKeyCode::Equals => Some(KeyCode::Equals),
        winit::event::VirtualKeyCode::Grave => Some(KeyCode::Grave),
        winit::event::VirtualKeyCode::Kana => Some(KeyCode::Kana),
        winit::event::VirtualKeyCode::Kanji => Some(KeyCode::Kanji),
        winit::event::VirtualKeyCode::LAlt => Some(KeyCode::LAlt),
        winit::event::VirtualKeyCode::LBracket => Some(KeyCode::LBracket),
        winit::event::VirtualKeyCode::LControl => Some(KeyCode::LControl),
        winit::event::VirtualKeyCode::LShift => Some(KeyCode::LShift),
        winit::event::VirtualKeyCode::LWin => Some(KeyCode::LWin),
        winit::event::VirtualKeyCode::Mail => Some(KeyCode::Mail),
        winit::event::VirtualKeyCode::MediaSelect => Some(KeyCode::MediaSelect),
        winit::event::VirtualKeyCode::MediaStop => Some(KeyCode::MediaStop),
        winit::event::VirtualKeyCode::Minus => Some(KeyCode::Minus),
        winit::event::VirtualKeyCode::NumpadMultiply => Some(KeyCode::Multiply),
        winit::event::VirtualKeyCode::Mute => Some(KeyCode::Mute),
        winit::event::VirtualKeyCode::MyComputer => Some(KeyCode::MyComputer),
        winit::event::VirtualKeyCode::NavigateForward => Some(KeyCode::NavigateForward),
        winit::event::VirtualKeyCode::NavigateBackward => Some(KeyCode::NavigateBackward),
        winit::event::VirtualKeyCode::NextTrack => Some(KeyCode::NextTrack),
        winit::event::VirtualKeyCode::NoConvert => Some(KeyCode::NoConvert),
        winit::event::VirtualKeyCode::NumpadComma => Some(KeyCode::NumpadComma),
        winit::event::VirtualKeyCode::NumpadEnter => Some(KeyCode::NumpadEnter),
        winit::event::VirtualKeyCode::NumpadEquals => Some(KeyCode::NumpadEquals),
        winit::event::VirtualKeyCode::OEM102 => Some(KeyCode::OEM102),
        winit::event::VirtualKeyCode::Period => Some(KeyCode::Period),
        winit::event::VirtualKeyCode::PlayPause => Some(KeyCode::PlayPause),
        winit::event::VirtualKeyCode::Power => Some(KeyCode::Power),
        winit::event::VirtualKeyCode::PrevTrack => Some(KeyCode::PrevTrack),
        winit::event::VirtualKeyCode::RAlt => Some(KeyCode::RAlt),
        winit::event::VirtualKeyCode::RBracket => Some(KeyCode::RBracket),
        winit::event::VirtualKeyCode::RControl => Some(KeyCode::RControl),
        winit::event::VirtualKeyCode::RShift => Some(KeyCode::RShift),
        winit::event::VirtualKeyCode::RWin => Some(KeyCode::RWin),
        winit::event::VirtualKeyCode::Semicolon => Some(KeyCode::Semicolon),
        winit::event::VirtualKeyCode::Slash => Some(KeyCode::Slash),
        winit::event::VirtualKeyCode::Sleep => Some(KeyCode::Sleep),
        winit::event::VirtualKeyCode::Stop => Some(KeyCode::Stop),
        winit::event::VirtualKeyCode::NumpadSubtract => Some(KeyCode::Subtract),
        winit::event::VirtualKeyCode::Sysrq => Some(KeyCode::Sysrq),
        winit::event::VirtualKeyCode::Tab => Some(KeyCode::Tab),
        winit::event::VirtualKeyCode::Underline => Some(KeyCode::Underline),
        winit::event::VirtualKeyCode::Unlabeled => Some(KeyCode::Unlabeled),
        winit::event::VirtualKeyCode::VolumeDown => Some(KeyCode::VolumeDown),
        winit::event::VirtualKeyCode::VolumeUp => Some(KeyCode::VolumeUp),
        winit::event::VirtualKeyCode::Wake => Some(KeyCode::Wake),
        winit::event::VirtualKeyCode::WebBack => Some(KeyCode::WebBack),
        winit::event::VirtualKeyCode::WebFavorites => Some(KeyCode::WebFavorites),
        winit::event::VirtualKeyCode::WebForward => Some(KeyCode::WebForward),
        winit::event::VirtualKeyCode::WebHome => Some(KeyCode::WebHome),
        winit::event::VirtualKeyCode::WebRefresh => Some(KeyCode::WebRefresh),
        winit::event::VirtualKeyCode::WebSearch => Some(KeyCode::WebSearch),
        winit::event::VirtualKeyCode::WebStop => Some(KeyCode::WebStop),
        winit::event::VirtualKeyCode::Yen => Some(KeyCode::Yen),
        winit::event::VirtualKeyCode::Copy => Some(KeyCode::Copy),
        winit::event::VirtualKeyCode::Paste => Some(KeyCode::Paste),
        winit::event::VirtualKeyCode::Cut => Some(KeyCode::Cut),
        //
        winit::event::VirtualKeyCode::Asterisk => None,
        winit::event::VirtualKeyCode::Plus => None,
    }
}

// As defined in: http://www.unicode.org/faq/private_use.html
pub(crate) fn is_private_use_character(c: char) -> bool {
    match c {
        '\u{E000}'..='\u{F8FF}' | '\u{F0000}'..='\u{FFFFD}' | '\u{100000}'..='\u{10FFFD}' => true,
        _ => false,
    }
}
