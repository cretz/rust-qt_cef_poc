
extern crate cef_sys;
extern crate kernel32;

use std::ptr;
use std::os::raw;
use std::mem;

pub struct Cef {

}

pub enum InitErr {
    ChildWithExitCode(i32),
    InitFail
}

fn cef_str_opt<'a>(s: Option<&'a str>) -> cef_sys::cef_string_t {
    match s {
        Some(_) => panic!("CEF strings not handled yet"),
        None => unsafe { mem::zeroed() }
    }
}

impl Cef {
    pub fn new() -> Cef {
        Cef { }
    }

    pub fn init(&self) -> Result<(), InitErr> {
        unsafe {
            cef_sys::cef_enable_highdpi_support();

            // Start main CEF process
            let main_args = cef_sys::cef_main_args_t {
                instance: kernel32::GetModuleHandleW(ptr::null()) as cef_sys::HINSTANCE
            };
            let exit_code = cef_sys::cef_execute_process(&main_args, ptr::null_mut(), ptr::null_mut());
            debug!("Exit code from exec process: {}", exit_code);
            if exit_code >= 0 { return Err(InitErr::ChildWithExitCode(exit_code)); }

            let settings = cef_sys::cef_settings_t {
                size: mem::size_of::<cef_sys::cef_settings_t>(),
                single_process: false as raw::c_int,
                no_sandbox: true as raw::c_int,
                browser_subprocess_path: cef_str_opt(None),
                framework_dir_path: cef_str_opt(None),
                multi_threaded_message_loop: false as raw::c_int,
                external_message_pump: false as raw::c_int,
                windowless_rendering_enabled: false as raw::c_int,
                command_line_args_disabled: false as raw::c_int,
                cache_path: cef_str_opt(None),
                user_data_path: cef_str_opt(None),
                persist_session_cookies: false as raw::c_int,
                persist_user_preferences: false as raw::c_int,
                user_agent: cef_str_opt(None),
                product_version: cef_str_opt(None),
                locale: cef_str_opt(None),
                log_file: cef_str_opt(None),
                log_severity: cef_sys::cef_log_severity_t::LOGSEVERITY_DEFAULT,
                javascript_flags: cef_str_opt(None),
                resources_dir_path: cef_str_opt(None),
                locales_dir_path: cef_str_opt(None),
                pack_loading_disabled: false as raw::c_int,
                remote_debugging_port: 0,
                uncaught_exception_stack_size: 0,
                ignore_certificate_errors: false as raw::c_int,
                enable_net_security_expiration: false as raw::c_int,
                background_color: 0,
                accept_language_list: cef_str_opt(None)
            };
            if cef_sys::cef_initialize(&main_args, &settings, ptr::null_mut(), ptr::null_mut()) != 1 {
                return Err(InitErr::InitFail);
            }
        }
        Ok(())
    }

    pub fn tick(&self) {
        unsafe {
            cef_sys::cef_do_message_loop_work();
        }
    }

    pub fn shutdown(&self) {
        unsafe {
            cef_sys::cef_shutdown();
        }
    }
}