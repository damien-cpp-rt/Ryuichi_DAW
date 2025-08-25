use std::ffi::CStr;
use std::os::raw::c_char;
use crate::Engine;

#[no_mangle]
pub extern "C" fn rust_sound_file_update(engine:*mut Engine , path: *const c_char) -> bool {
    if path.is_null(){
        return false;
    }
    if engine.is_null(){
        return false;
    }
    let c_str = unsafe {CStr::from_ptr(path).to_str()};
    let path_string = match c_str {
        Ok(s) => s.to_string(),
        Err(_) => return false,
    };
    let eng : &mut Engine = unsafe { &mut *engine };
    eng.track.file_path.push(path_string.to_owned());
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_volume_update() { 

}

#[no_mangle]
pub extern "C" fn rust_sound_mute_update() { 

}

#[no_mangle]
pub extern "C" fn rust_sound_pan_update() { 

}

#[no_mangle]
pub extern "C" fn rust_sound_balance_update() { 

}

#[no_mangle]
pub extern "C" fn rust_sound_reverb_update() { 

}


#[no_mangle]
pub extern "C" fn rust_sound_delay_update() { 

}