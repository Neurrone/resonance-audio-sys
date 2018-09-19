use resonance_audio_sys::ResonanceAudioApiImpl as api;
use std::ffi::c_void;

fn main() {
    let mut resonance_api = unsafe {
        api::new(2, 100, 44100);
    };
    println!("Resonance audio api created.");
    // crashes here
    let source = unsafe { resonance_audio_sys::ResonanceAudioApiImpl_CreateStereoSource(&mut resonance_api as *mut _ as *mut c_void, 2) };
    println!("Source created.");
}