// Modification of https://github.com/Picovoice/pvrecorder/blob/main/demo/rust/src/main.rs

use pv_recorder::PvRecorderBuilder;

const SAMPLE_RATE: usize = 16000;

pub fn output_to_wav(wav_file: &str) {
    let audio_device_index = 0i32;
    let output_wav_path = wav_file.to_owned();

    let recorder = PvRecorderBuilder::new(512)
        .device_index(audio_device_index)
        .init()
        .expect("Failed to initialize pvrecorder");

    recorder.start().expect("Failed to start audio recording");

    println!("Press ENTER to stop recording.");
    let mut audio_data = Vec::new();
    loop {
        let frame = recorder.read().expect("Failed to read audio frame");
        audio_data.extend_from_slice(&frame);
        if mki::are_pressed(&[mki::Keyboard::Enter]) {
            break;
        }
    }

    recorder.stop().expect("Failed to stop audio recording");

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(output_wav_path, spec).unwrap();
    for sample in audio_data {
        writer.write_sample(sample).unwrap();
    }
}
