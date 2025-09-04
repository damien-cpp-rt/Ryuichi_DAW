use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU64;
use crate::Engine;
use crate::Clip;


#[no_mangle]
pub extern "C" fn rust_sound_add_clip(engine : *mut Engine, number : i32, path: *const c_char, tl_start : u64, tl_len : u64,src:u32) -> bool {
    if engine.is_null(){
        return false;
    }
    let eng : &mut Engine = unsafe {&mut *engine};
    let idx = match usize::try_from(number){
        Ok(number)=> number,
        Err(_) => return false,
    };
    if idx >= eng.track.len() || idx >= eng.rt.len() { return false; }
    let c_str = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();;
    let path_str = c_str.as_str();
     if tl_len == 0 { return false; }
    let clip =Clip {
    file_path : path_str.to_string(),
    src_sr : src,
    tl_start : tl_start,
    tl_len : tl_len,
    };
    if let Some(mut tr_mx) = eng.rt.get(idx) {
        if let Ok(mut tr) = tr_mx.lock() {
            tr.clip.insert(tl_start,clip);
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_move_clip_by_start(engine : *mut Engine, old_track :i32,old_start:u64,new_track:i32,new_start:u64) -> bool {
    if engine.is_null(){
        return false;
    }
    let eng : &mut Engine = unsafe {&mut *engine};
    let old_idx = match usize::try_from(old_track){
        Ok(number)=> number,
        Err(_) => return false,
    };
    let new_idx = match usize::try_from(new_track){
        Ok(number)=> number,
        Err(_) => return false,
    };
    if old_idx >= eng.track.len() || old_idx >= eng.rt.len() { return false; }
    if new_idx >= eng.track.len() || new_idx >= eng.rt.len() { return false; }
    let clip_opt : Option<Clip>;
    if let Some(old_tr_mx) = eng.rt.get(old_idx) {
        if let Ok(mut old_tr) = old_tr_mx.lock() {
            clip_opt = old_tr.clip.remove(&old_start);
        } else { return false; }
    } else { return false; };
    if let Some(clip) = clip_opt {
        let mut new_clip = clip;
        new_clip.tl_start = new_start;
        if let Some(new_tr_mx) = eng.rt.get(new_idx) {
            if let Ok(mut new_tr) = new_tr_mx.lock() {
                new_tr.clip.insert(new_start,new_clip);
            } else { return false; }
        } else { return false; };
    } else { return false; }
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_delete_clip_by_start(engine : *mut Engine, track :i32,start:u64) -> bool {
    if engine.is_null(){
        return false;
    }
    let eng : &mut Engine = unsafe {&mut *engine};
    let idx = match usize::try_from(track){
        Ok(number)=> number,
        Err(_) => return false,
    };
    if idx >= eng.track.len() || idx >= eng.rt.len() { return false; }
    if let Some(tr_mx) = eng.rt.get(idx) {
        if let Ok(mut tr) = tr_mx.lock() {
            tr.clip.remove(&start);
        }
    }
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
