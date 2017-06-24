// #![windows_subsystem = "windows"]
// TODO: re-enable the above for prod, leaving disabled now during dev

extern crate qt_widgets;
extern crate qt_core;
#[macro_use]
extern crate log;
extern crate env_logger;

mod cef;

use cef::{Cef, InitErr};
use qt_widgets::application::Application;
use qt_widgets::push_button::PushButton;
use qt_core::string::String;
use qt_core::timer::Timer;
use qt_core::slots::SlotNoArgs;
use qt_core::connection::Signal;

use std::process;

fn main() {
    // Setup logger
    env_logger::init().unwrap();

    // Init CEF
    let c = Cef::new();
    match c.init() {
        Ok(c) => c,
        Err(InitErr::ChildWithExitCode(exit_code)) => process::exit(exit_code),
        Err(InitErr::InitFail) => panic!("Unable to init")
    };

    // Create Qt app
    Application::create_and_exit(|_app| {

        // Need a timer for CEF message loop
        let mut timer = Timer::new();
        timer.signals().timeout().connect(&SlotNoArgs::new(|| {
            c.tick();
        }));
        timer.start(10);

        let mut button = PushButton::new(&String::from("Hello, World"));
        button.show();

        let exit_code = Application::exec();
        timer.stop();
        c.shutdown();

        exit_code
    })
}
