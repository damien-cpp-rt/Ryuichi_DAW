
mod waveform_generation_module;
pub use waveform_generation_module::*;
mod sound_track_update;
pub use sound_track_update::*;

enum TrackNumber {
    Zero,
    One,
    Two,
    Three,
}

struct TrackDatas {
    track_number : TrackNumber,
    file_path : Vec<String>,
    volume : f32,
    muted : bool,
    pan : f32,
    sound_balance : f32,
    reverb : bool,
    delay : bool,
}

impl TrackDatas {
    fn new(number : i32) -> Result<Self,String> {
      let track_num = match number {
        0 => TrackNumber::Zero,
        1 => TrackNumber::One,
        2 => TrackNumber::Two,
        3 => TrackNumber::Three,
        _ => return Err("not a valid track number".to_string()),
      };
      Ok (Self {
        track_number : track_num,
        file_path : Vec::new(),
        volume : 0.5,
        muted : false,
        pan : 0.0,
        sound_balance : 0.0,
        reverb : false,
        delay : false,
      })
    }
}
pub struct Engine {
    track : TrackDatas,
}

#[no_mangle]
pub extern "C" fn rust_audio_engine_new(number : i32) -> *mut Engine {
    let track = match TrackDatas::new(number) {
        Ok(data) => data,
        Err(_) => return std::ptr::null_mut(),
    };

    let eng = Engine {track};
    Box::into_raw(Box::new(eng))
}

#[no_mangle]
pub extern  "C" fn rust_audio_engine_free( engine :*mut Engine) {
    if engine.is_null() {
        return;
    }
   unsafe { drop(Box::from_raw(engine));}
}