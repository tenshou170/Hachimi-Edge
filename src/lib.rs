#![allow(static_mut_refs)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate cstr;

rust_i18n::i18n!("assets/locales", fallback = "en");

#[macro_use]
pub mod core;
pub mod il2cpp;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{game_impl, gui_impl, hachimi_impl, interceptor_impl, log_impl, symbols_impl};

#[cfg(not(target_os = "windows"))]
mod stub {
    #![allow(dead_code)]
    pub mod game_impl {
        use crate::core::game::Region;
        use std::path::PathBuf;

        pub fn get_package_name() -> String {
            "com.example.game".to_owned()
        }
        pub fn get_region(_: &str) -> Region {
            Region::Japan
        }
        pub fn get_data_dir(_: &str) -> PathBuf {
            PathBuf::from(".")
        }
        pub fn init() {}
        pub fn is_steam_release(_: &str) -> bool {
            false
        }
    }

    pub mod gui_impl {
        pub fn init() {}
    }

    pub mod hachimi_impl {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Clone, Default)]
        pub struct Config {
            #[serde(default)]
            pub vsync_count: i32,
            #[serde(default)]
            pub window_always_on_top: bool,
            #[serde(default)]
            pub discord_rpc: bool,
        }

        pub fn is_il2cpp_lib(_: &str) -> bool {
            false
        }
        pub fn is_criware_lib(_: &str) -> bool {
            false
        }
        pub fn on_hooking_finished(_: &crate::core::hachimi::Hachimi) {}
    }

    pub mod interceptor_impl {
        use crate::core::{interceptor::HookHandle, Error};
        pub unsafe fn unhook(_: &HookHandle) -> Result<(), Error> {
            Ok(())
        }
        pub unsafe fn unhook_vtable(_: &HookHandle) -> Result<(), Error> {
            Ok(())
        }
        pub unsafe fn hook(_: usize, _: usize) -> Result<usize, Error> {
            Ok(0)
        }
        pub unsafe fn hook_vtable(_: *mut usize, _: usize, _: usize) -> Result<HookHandle, Error> {
            Ok(HookHandle {
                orig_addr: 0,
                trampoline_addr: 0,
                hook_type: crate::core::interceptor::HookType::Vtable,
            })
        }
        pub unsafe fn get_vtable_from_instance(_: usize) -> *mut usize {
            std::ptr::null_mut()
        }
        pub unsafe fn find_symbol_by_name(_: &str, _: &str) -> Result<usize, Error> {
            Ok(0)
        }
    }

    pub mod log_impl {
        pub fn init(_: log::LevelFilter) {}
    }

    pub mod symbols_impl {
        pub fn init() {}
        pub unsafe fn dlsym(_: *mut std::ffi::c_void, _: &str) -> usize {
            0
        }
    }
}

#[cfg(not(target_os = "windows"))]
use stub::{game_impl, gui_impl, hachimi_impl, interceptor_impl, log_impl, symbols_impl};
