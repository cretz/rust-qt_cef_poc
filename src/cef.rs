
extern crate cef_sys;
extern crate kernel32;
extern crate winapi;

use std::ptr;
use std::os::raw;
use std::mem;
use std::sync::atomic;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ffi::CString;

pub struct Cef {

}

pub enum InitErr {
    ChildWithExitCode(i32),
    InitFail
}

pub fn cef_str_opt<'a>(s: Option<&'a str>) -> cef_sys::cef_string_t {
    match s {
        None => unsafe { mem::zeroed() },
        Some(s) => unsafe {
            let mut new_s: cef_sys::cef_string_utf16_t = mem::zeroed();
            let cstr = CString::new(s).unwrap();
            // TODO: I am expected to call "dtor" on this during release
            let ret = cef_sys::cef_string_utf8_to_utf16(cstr.as_ptr(), s.len(), &mut new_s);
            assert_eq!(ret, 1);
            new_s
        }
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

    pub fn create_browser(&self, parent: winapi::HWND, width: i32, height: i32) -> *mut cef_sys::cef_browser_t {
        let win_info = cef_sys::cef_window_info_t {
            ex_style: 0,
            window_name: cef_str_opt(None),
            style: cef_sys::WS_CHILD | cef_sys::WS_CLIPCHILDREN | cef_sys::WS_CLIPSIBLINGS | cef_sys::WS_TABSTOP | cef_sys::WS_VISIBLE,
            x: 0,
            y: 0,
            width: width,
            height: height,
            parent_window: parent as cef_sys::HWND,
            menu: ptr::null_mut(),
            windowless_rendering_enabled: false as raw::c_int,
            window: ptr::null_mut()
        };
        /*
        let cef_base: *mut cef_sys::cef_base_ref_counted_t = unsafe { mem::transmute(&CefBaseRefCounted::new()) };
        let cef_client = cef_sys::cef_client_t {
            base: unsafe { *cef_base },
            get_context_menu_handler: None,
            get_dialog_handler: None,
            get_display_handler: None,
            get_download_handler: None,
            get_drag_handler: None,
            get_find_handler: None,
            get_focus_handler: None,
            get_geolocation_handler: None,
            get_jsdialog_handler: None,
            get_keyboard_handler: None,
            get_life_span_handler: None,
            get_load_handler: None,
            get_render_handler: None,
            get_request_handler: None,
            on_process_message_received: Some(Cef::on_process_message_received)
        };
        */
        let mut settings: cef_sys::cef_browser_settings_t = unsafe { mem::zeroed() };
        settings.size = mem::size_of::<cef_sys::cef_browser_settings_t>();
        unsafe {
            cef_sys::cef_browser_host_create_browser_sync(&win_info,
                                                          ptr::null_mut(),
                                                          &cef_str_opt(Some("http://google.com")),
                                                          &settings,
                                                          ptr::null_mut())
        }
    }

    unsafe extern "C" fn on_process_message_received(self_: *mut cef_sys::cef_client_t,
                                                     browser: *mut cef_sys::cef_browser_t,
                                                     source_process: cef_sys::cef_process_id_t,
                                                     message: *mut cef_sys::cef_process_message_t) -> raw::c_int {
        false as raw::c_int
    }
}

#[repr(C)]
struct CefBaseRefCounted {
    base: cef_sys::cef_base_ref_counted_t,
    refs: AtomicUsize
}

// Help from https://github.com/dylanede/cef-rs/blob/e9adc70485f592d0783ef65af6abc9a38bf049e0/src/lib.rs#L129
impl CefBaseRefCounted {
    fn new() -> CefBaseRefCounted {
        CefBaseRefCounted {
            base: cef_sys::cef_base_ref_counted_t {
                size: mem::size_of::<CefBaseRefCounted>(),
                add_ref: Some(CefBaseRefCounted::add_ref),
                release: Some(CefBaseRefCounted::release),
                has_one_ref: Some(CefBaseRefCounted::has_one_ref)
            },
            refs: AtomicUsize::new(1)
        }
    }

    fn from_raw(self_: *mut cef_sys::cef_base_ref_counted_t) -> *mut CefBaseRefCounted {
        unsafe {
            let v: *mut CefBaseRefCounted = mem::transmute(self_);
            v
        }
    }

    unsafe extern "C" fn add_ref(self_: *mut cef_sys::cef_base_ref_counted_t) {
        (*CefBaseRefCounted::from_raw(self_)).refs.fetch_add(1, Ordering::Relaxed);
    }

    unsafe extern "C" fn release(self_: *mut cef_sys::cef_base_ref_counted_t) -> raw::c_int {
        let v = CefBaseRefCounted::from_raw(self_);
        let old_count = (*v).refs.fetch_sub(1, Ordering::Release);
        if old_count == 1 {
            atomic::fence(Ordering::Acquire);
            let v: Box<CefBaseRefCounted> = mem::transmute(v);
            drop(v);
        }
        if old_count == 1 { 1 } else { 0 }
    }

    unsafe extern "C" fn has_one_ref(self_: *mut cef_sys::cef_base_ref_counted_t) -> raw::c_int {
        if (*CefBaseRefCounted::from_raw(self_)).refs.load(Ordering::SeqCst) == 1 { 1 } else { 0 }
    }
}