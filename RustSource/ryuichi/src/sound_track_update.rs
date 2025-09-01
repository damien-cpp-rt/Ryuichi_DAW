use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::atomic::Ordering;
use crate::Engine;

#[no_mangle]
pub extern "C" fn rust_sound_file_update(engine:*mut Engine , path: *const c_char, number : i32) -> bool {
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
      let idx = match usize::try_from(number) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let eng : &mut Engine = unsafe { &mut *engine };
    if idx >= eng.track.len() {
        return false;
    }
    eng.track[idx].file_path.push(path_string.to_owned());
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_volume_update(engine : *mut Engine , volume : f32, number : i32) -> bool {
     if engine.is_null() {
        return false;
     }
     let eng : &mut Engine = unsafe {&mut *engine};
     let idx= match usize::try_from(number) {
        Ok(number) => number,
        Err(_) => return false,
     };
     let v = volume.clamp(0.0, 1.0);
     eng.track[idx].volume=v;
     eng.params.volume[idx].store(v.to_bits(),Ordering::Relaxed); //실시간 반영
     true
}

#[no_mangle]
pub extern "C" fn rust_sound_mute_update(engine : *mut Engine, mute : bool, number :i32)->bool { 
    if engine.is_null(){
        return false;
    }
    let eng : &mut Engine = unsafe {&mut *engine};
    let idx = match usize::try_from(number){
        Ok(number)=> number,
        Err(_) => return false,
    };
    if idx >= eng.track.len() { return false; }
    eng.track[idx].muted = mute;
    eng.params.muted[idx].store(mute,Ordering::Relaxed); //실시간 반영
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_pan_update(engine : *mut Engine, pan : f32, number :i32)->bool{ 
       if engine.is_null(){
        return false;
    }
    let eng : &mut Engine = unsafe {&mut *engine};
    let idx = match usize::try_from(number){
        Ok(number)=> number,
        Err(_) => return false,
    };
    if idx >= eng.track.len() { return false; }
    eng.track[idx].pan = pan.clamp(-1.0, 1.0);
    eng.params.pan[idx].store(pan.to_bits(),Ordering::Relaxed); //실시간 반영
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_bpm_update(engine : *mut Engine, bpm : f32) -> bool {
            if engine.is_null(){
        return false;
    }
    let eng : &mut Engine = unsafe {&mut *engine};
    let b = bpm.clamp(20.0, 300.0);
    eng.params.bpm.store(b.to_bits(),Ordering::Relaxed); //실시간 반영
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_reverb_update() { 

}


#[no_mangle]
pub extern "C" fn rust_sound_delay_update() { 

}

#[no_mangle]
pub extern "C" fn rust_sound_file_all_delete(engine : *mut Engine, number : i32) -> bool{
    if engine.is_null(){
        return false;
    }
    let eng : &mut Engine = unsafe {&mut *engine};
    let idx = match usize::try_from(number){
        Ok(number)=> number,
        Err(_) => return false,
    };
    if idx >= eng.track.len() { return false; }
    eng.track[idx].file_path.clear();
    true
}