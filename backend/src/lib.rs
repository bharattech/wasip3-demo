mod bindings {
    wit_bindgen::generate!({
        world: "component",
        generate_all,
    });
}

use oxiwhisper::{WhisperModel, TranscribeOptions};
use std::path::Path;

struct Component;

impl bindings::exports::wasmcloud::p3_multi_comps_demo::receiver::Guest for Component {
    fn invoke() -> Result<String, String> {
        println!("receiver call : invoke()");
        let model = WhisperModel::from_file(Path::new("data/ggml-tiny.bin")).unwrap();
        let audio = oxiwhisper::audio::load_wav(Path::new("data/jfk.wav")).unwrap();
        let text = model.transcribe(&audio, &TranscribeOptions::default()).unwrap();
        println!("Transcribe text: {:?}", text);
        
        Ok(text)
    }
}

bindings::export!(Component with_types_in bindings);
