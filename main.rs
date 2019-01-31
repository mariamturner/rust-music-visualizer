extern crate hound;
extern crate num;
extern crate rustfft;
extern crate find_folder;
extern crate portaudio;
extern crate sample;

mod audio;

use audio::*;
use std::f32::consts::PI;
use std::i16;
use std::env;


// Writing Sine Wave for testing purposes
fn write_sin_wav(note: f32) {
	let sampling_freq = 44100;
	let sampling_bits = 16;
	let amplitude = i16::MAX as f32;
	let note_freq = note;
	let length = 2;

	let no_of_samples = sampling_freq * length;
	let normalized_sample_indices = (0 .. no_of_samples).
		map(|x| x as f32 / sampling_freq as f32);

	let spec = hound::WavSpec {
		channels: 1,
		sample_rate: sampling_freq,
		bits_per_sample: sampling_bits
	};

	let maybe_writer = hound::WavWriter::create("sine.wav", spec);
	let mut xs: Vec<f32> = Vec::with_capacity(no_of_samples as usize);
	let mut ys: Vec<f32> = Vec::with_capacity(no_of_samples as usize);

	match maybe_writer {
		Ok(writer_obj) => {
			let mut writer = writer_obj;
			for t in normalized_sample_indices {
				let sample = (t * note_freq * 2.0 * PI).sin();
				xs.push(t);
				ys.push(sample);
				writer.write_sample((sample * amplitude) as i16).unwrap();
			}
		},
		Err(e) => {
			println!("No");
			println!("{}", e);
		}
	}

	}


fn main() {
	
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let filename = &args[1];
		println!("Song choice is: {}", filename);
		let my_song = &filename;
		let song: &str = &my_song;
		if let Some(peak) = find_spectral_peak(song) {
			println!("Max frequency: {} Hz", song);
		}
		return_rms(song);
		playback(song);
	} else {
		println!("Please input one filename in quotation marks.");
	}
		
	/* Testing with sine wave
	write_sin_wav(450.0);
	if let Some(peak) = find_spectral_peak("sine.wav") {
		println!("Max frequency: {} Hz", peak);
	}
	return_rms("sine.wav");
	playback("sine.wav");*/
}
