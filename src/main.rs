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
use qt_widgets::main_window::MainWindow;
use qt_widgets::widget::Widget;
use qt_widgets::line_edit::LineEdit;
use qt_widgets::layout::Layout;
use qt_widgets::grid_layout::GridLayout;
use qt_widgets::frame::Frame;
use qt_core::string::String as QString;
use qt_core::timer::Timer;
use qt_core::slots::SlotNoArgs;
use qt_core::connection::Signal;
use qt_core::qt::FocusPolicy;

use std::process;

fn main() {
    // Setup logger
    env_logger::init().unwrap();
    debug!("Creating app...");

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

        // Create main window
        let mut main_win = MainWindow::new();
        main_win.set_window_title(&QString::from("QT CEF POC"));
        main_win.set_focus_policy(FocusPolicy::Strong);
        main_win.resize((800, 600));

        // Create CEF widget
        let mut cef_widg = Widget::new();
        cef_widg.show();

        // Create url widget
        let url_widg = LineEdit::new(());
        let ret_press_slot = SlotNoArgs::new(|| {
            debug!("You typed: {}", url_widg.text().to_std_string());
        });
        url_widg.signals().return_pressed().connect(&ret_press_slot);

        // Layout main win
        let mut layout = GridLayout::new();
        unsafe { layout.add_widget((url_widg.as_mut_ptr() as *mut Widget, 0, 0)); }
        unsafe { layout.add_widget((cef_widg.into_raw() as *mut Widget, 1, 0)); }
        layout.set_contents_margins((0, 0, 0, 0));
        layout.set_spacing(0);
        layout.set_row_stretch(0, 0);
        layout.set_row_stretch(1, 1);
        let mut frame = Frame::new();
        unsafe { frame.set_layout(layout.into_raw() as *mut Layout); }
        unsafe { main_win.set_central_widget(frame.into_raw() as *mut Widget); }

        // Show main win
        main_win.show();
        main_win.activate_window();
        main_win.raise();

        let exit_code = Application::exec();
        timer.stop();
        c.shutdown();

        exit_code
    });
}
