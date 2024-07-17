use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

pub struct SoundManager {
	playing_sounds: HashMap<String, Sink>,
	stream_handles: HashMap<String, (OutputStream, OutputStreamHandle)>,
	file_registry: HashMap<String, String>
}

impl SoundManager {
	pub fn new() -> SoundManager {
		SoundManager {
			playing_sounds: HashMap::new(),
			stream_handles: HashMap::new(),
			file_registry: HashMap::new(),
		}
	}

	pub fn play(&mut self, name: &str, source: Decoder<BufReader<File>>) -> &mut Sink {
		// First, check if the key exists
		if !self.playing_sounds.contains_key(name) {
			let (stream, stream_handle) = OutputStream::try_default().unwrap();
			let new_sink = match Sink::try_new(&stream_handle){
			//let new_sink = match Sink::try_new(&self.stream_handle){
				Ok(handle)=>handle,
				Err(why) => {
					panic!("{}",why);
				}
			};
			self.playing_sounds.insert(name.to_string(), new_sink);
			self.stream_handles.insert(name.to_string(), (stream, stream_handle));
		}

		let sink_obj = self.playing_sounds.get_mut(name).unwrap();
		if !sink_obj.empty() {
			sink_obj.clear();
		}
		sink_obj.append(source);
		sink_obj.play();
		sink_obj
	}

	pub fn play_registered(&mut self, name: &str, register_name: &str) -> &mut Sink {
		// First, check if the key exists
		if !self.playing_sounds.contains_key(name) {
			let (stream, stream_handle) = OutputStream::try_default().unwrap();
			let new_sink = match Sink::try_new(&stream_handle){
				//let new_sink = match Sink::try_new(&self.stream_handle){
				Ok(handle)=>handle,
				Err(why) => {
					panic!("{}",why);
				}
			};
			self.playing_sounds.insert(name.to_string(), new_sink);
			self.stream_handles.insert(name.to_string(), (stream, stream_handle));
		}

		let sink_obj = self.playing_sounds.get_mut(name).unwrap();
		if !sink_obj.empty() {
			sink_obj.clear();
		}
		sink_obj.append(
			Decoder::new(
				BufReader::new(
					File::open(
						Path::new(
							self.file_registry.get(register_name).unwrap()
						)
					).unwrap()
				)
			).unwrap()
		);
		sink_obj.play();
		sink_obj
	}

	pub fn play_registered_looping(&mut self, name: &str, register_name: &str) -> &mut Sink {
		// First, check if the key exists
		if !self.playing_sounds.contains_key(name) {
			let (stream, stream_handle) = OutputStream::try_default().unwrap();
			let new_sink = match Sink::try_new(&stream_handle){
				//let new_sink = match Sink::try_new(&self.stream_handle){
				Ok(handle)=>handle,
				Err(why) => {
					panic!("{}",why);
				}
			};
			self.playing_sounds.insert(name.to_string(), new_sink);
			self.stream_handles.insert(name.to_string(), (stream, stream_handle));
		}

		let sink_obj = self.playing_sounds.get_mut(name).unwrap();
		if !sink_obj.empty() {
			sink_obj.clear();
		}
		sink_obj.append(
			Decoder::new(
				BufReader::new(
					File::open(
						Path::new(
							self.file_registry.get(register_name).unwrap()
						)
					).unwrap()
				)
			).unwrap().repeat_infinite()
		);
		sink_obj.play();
		sink_obj
	}

	pub fn register_file(&mut self, name: &str, filepath: String){
		self.file_registry.insert(String::from(name), filepath);
	}

	pub fn get(&self, name: &str) -> Option<&Sink> {
		self.playing_sounds.get(name)
	}
	pub fn get_mut(&mut self, name: &str) -> Option<&mut Sink> {
		//used if the player needs direct, mutable access to the sink object
		self.playing_sounds.get_mut(name)
	}
	pub fn cleanup(&mut self) {
		//remove all sinks and handles for empty sinks
		//call this periodically to mitigate memory leaks
		//if we had an automatic cleanup callback when sounds end, we wouldn't need this
		let sounds_to_remove: Vec<String> = self.playing_sounds
			.iter()
			.filter(|(_, sink)| sink.empty())
			.map(|(name, _)| name.clone())
			.collect();
		for name in sounds_to_remove {
			self.playing_sounds.remove(&name);
			self.stream_handles.remove(&name);
		}
	}

}
