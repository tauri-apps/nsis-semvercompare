#![no_std]

use nsis_plugin_api::*;

nsis_plugin!();

include!(concat!(env!("OUT_DIR"), "/combined_libs.rs"));
