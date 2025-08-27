use crate::Engine;

#[no_mangle]
pub extern "C" fn rust_sound_play(engine : *mut Engine) -> bool {
    if engine.is_null(){
        return false;
    }
    let eng = unsafe { &mut *engine};

    true
}

#[no_mangle]
pub extern "C" fn rust_sound_stop(eng : *mut Engine) -> bool {
true
}