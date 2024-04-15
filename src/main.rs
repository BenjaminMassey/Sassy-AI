mod gpt;
mod record;
mod transcribe;

use tts::*;
use whisper_rs::{WhisperContext, WhisperContextParameters};

const MODEL_BIN: &str = "C:\\Users\\benjamin.massey\\Downloads\\ggml-base.en.bin";
const WORKING_WAV_FILE: &str = "temp.wav";

fn main() {
    let whisper_ctx =
        WhisperContext::new_with_params(MODEL_BIN, WhisperContextParameters::default()).unwrap();
    let mut whisper = whisper_ctx.create_state().unwrap();
    let mut tts = Tts::default().unwrap();
    loop {
        println!("Press ENTER to start recording, or ESCAPE to quit.");
        loop {
            // TODO: unhappy with this structure, but stdin().read_line(..) not halting on non-first iterations
            if mki::are_pressed(&[mki::Keyboard::Enter]) {
                break;
            } else if mki::are_pressed(&[mki::Keyboard::Escape]) {
                std::process::exit(0);
            }
        }
        record::output_to_wav(WORKING_WAV_FILE); // will inform to press enter to stop
        let read = transcribe::wav_to_text(&mut whisper, WORKING_WAV_FILE);
        if read.is_ok() {
            let input = read.unwrap();
            println!("Your query: {}", &input);
            let message = format!("A user has entered the prompt '{input}'. Respond with the fact that it would be more useful for them
                to not use an AI for this task, and instead figure things out for themselves. If they want an answer, they should learn to
                research themselves. If they want code, they should learn to program properly. Be sassy, and assert that they need to
                do it themselves, while keeping context to their original question. Respond to me as if you were responding directly to them.
                Keep the repsonse rather short and simple, just conveying that they should do it themselves.");
            let query = gpt::local_gpt_chat(&message, 200);
            if let Some(response) = query {
                println!("\n{}\n\n", &response);
                let _ = tts.speak(&response, true);
            }
        }
    }
}
