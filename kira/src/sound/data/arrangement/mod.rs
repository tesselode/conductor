//! Combines individual sounds into larger pieces.
//!
//! `Arrangement`s are containers of `SoundClip`s, which are portions of
//! a sound that can be positioned in time, stretched, trimmed, and
//! reversed. You can play instances of an arrangement just like you would
//! play instances of a sound.
//!
//! `Arrangement`s are a lot like arrangement views in DAWs, like the
//! playlist view in FL Studio. In fact, the playlist view in FL Studio
//! will be used to illustrate the contents of `Arrangement`s in the
//! following examples.
//!
//! This image represents an arrangement that plays the same sound
//! four times: once normally, once trimmed, once stretched out,
//! and once reversed.
//!
//! ![arrangements 1](https://i.imgur.com/1p4W1Ld.png)
//!
//! ## Motivating example: seamless loops
//!
//! Oftentimes, game composers need to write pieces that loop forever.
//! These pieces may also have an intro section that plays once
//! before the main part of the song loops forever. `Instance`s allow
//! you to set a loop start point so when the playback position reaches
//! the end, it jumps back to an arbitrary point in the sound.
//!
//! The problem is this doesn't account for parts of the sound that
//! bleed into the next section. For example, at the end of an orchestral
//! piece, there may be a cymbal swell that transitions the song back
//! to the beginning of the loop. To preserve the musical timing of the
//! piece, the playback position needs to jump back to the start point
//! as soon as the last measure of music ends, which would cut off
//! the cymbal, creating a jarring sound. If the song has an intro section
//! with trailing sound, then that sound will cut in when the song
//! loops, which is also jarring.
//!
//! There's a couple possible solutions:
//! - Use a [`Sequence`](crate::sequence) to play separate
//!   intro and loop sounds at the right time. This works, but you
//!   can't reverse or change the playback rate of a sequence, which you may
//!   want in some circumstances.
//! - You can edit your intro and loop audio in a specific way to create a
//!   larger piece that will seamlessly loop. This is tedious, and you have
//!   to store a larger audio as part of the game's assets.
//!
//! Arrangements let you use the latter solution without having to store
//! a larger audio file, and as you'll see, they can do the work of setting
//! up seamless loops for you.
//!
//! ### Setting up a simple loop
//!
//! Let's say we have a short drum loop with a cymbal swell at the end
//! that will act as a transition as we go back to the beginning of the loop.
//!
//! ![arrangements 2](https://i.imgur.com/TOpa9Zq.png)
//!
//! We can set up a seamless loop by placing the same sound in an arrangement
//! twice, once with the cymbal swell preserved and once with it cut off.
//! The red region at the top shows the portion of the arrangement
//! that will be looped.
//!
//! ![arrangements 3](https://i.imgur.com/Xoti30y.png)
//!
//! When the playback position jumps back to the loop point, the trailing sound
//! from the first sound clip will seamlessly connect to the trailing
//! sound that was cut off from the second clip.
//!
//! You can set up this arrangement manually:
//!
//! ```no_run
//! # use kira::{
//! # 	arrangement::{Arrangement, ArrangementSettings, SoundClip},
//! # 	manager::{AudioManager, AudioManagerSettings},
//! # 	sound::{Sound, SoundSettings}, Tempo,
//! # };
//! #
//! # let mut audio_manager = AudioManager::new(AudioManagerSettings::default())?;
//! # let sound_handle = audio_manager.add_sound(Sound::from_file(
//! # 	std::env::current_dir()?.join("assets/loop.wav"),
//! # 	SoundSettings::default(),
//! # )?)?;
//! #
//! let tempo = Tempo(140.0);
//! let mut arrangement = Arrangement::new(
//! 	ArrangementSettings::new().default_loop_start(tempo.beats_to_seconds(16.0)),
//! );
//! arrangement
//! 	.add_clip(SoundClip::new(&sound_handle, 0.0))
//! 	.add_clip(
//! 		SoundClip::new(&sound_handle, tempo.beats_to_seconds(16.0))
//! 			.trim(tempo.beats_to_seconds(16.0)),
//! 	);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Or you can just use [`Arrangement::new_loop`], which will do the work for you:
//!
//! ```no_run
//! # use kira::{
//! # 	arrangement::{Arrangement, LoopArrangementSettings, SoundClip},
//! # 	manager::{AudioManager, AudioManagerSettings},
//! # 	sound::{Sound, SoundSettings}, Tempo,
//! # };
//! #
//! # let mut audio_manager = AudioManager::new(AudioManagerSettings::default())?;
//! let tempo = Tempo(140.0);
//! let sound_handle = audio_manager.add_sound(Sound::from_file(
//! 	std::env::current_dir()?.join("assets/loop.wav"),
//! 	SoundSettings {
//! 		semantic_duration: Some(tempo.beats_to_seconds(16.0)),
//! 		..Default::default()
//! 	},
//! )?)?;
//! let arrangement = Arrangement::new_loop(&sound_handle, LoopArrangementSettings::default());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Loops with intros
//!
//! Loops with intros can be set up in a similar way:
//!
//! ![arrangements 4](https://i.imgur.com/EM7P7Ry.png)
//!
//! For brevity, we'll just say you can use [`Arrangement::new_loop_with_intro`]
//! to create these.

mod clip;

pub use clip::SoundClip;

use crate::{sound::handle::SoundHandle, Frame};

use super::SoundData;

/// An arrangement of sound clips to play at specific times.
#[derive(Clone)]
pub struct Arrangement {
	clips: Vec<SoundClip>,
	duration: f64,
}

impl Arrangement {
	/// Creates a new, empty arrangement.
	pub fn new() -> Self {
		Self {
			clips: vec![],
			duration: 0.0,
		}
	}

	/// Creates a new arrangement that seamlessly loops a sound.
	///
	/// If the sound has a semantic duration, it will be used to
	/// set the point where the sound loops. Any audio after the
	/// semantic end of the sound will be preserved when the sound
	/// loops.
	pub fn new_loop(sound_handle: &SoundHandle) -> Self {
		let duration = sound_handle
			.semantic_duration()
			.unwrap_or(sound_handle.duration());
		let mut arrangement = Self::new();
		arrangement
			.add_clip(SoundClip::new(sound_handle, 0.0))
			.add_clip(SoundClip::new(sound_handle, duration).trim(duration));
		arrangement
	}

	/// Creates a new arrangement that plays an intro sound, then
	/// seamlessly loops another sound.
	///
	/// If the intro has a semantic duration, it will be used to determine
	/// when the loop sound starts. If the loop sound has a semantic duration,
	/// it will be used to set the point where the sound repeats. Any audio
	/// after the semantic end of the sound will be preserved when the sound
	/// loops.
	pub fn new_loop_with_intro(
		intro_sound_handle: &SoundHandle,
		loop_sound_handle: &SoundHandle,
	) -> Self {
		let intro_duration = intro_sound_handle
			.semantic_duration()
			.unwrap_or(intro_sound_handle.duration());
		let loop_duration = loop_sound_handle
			.semantic_duration()
			.unwrap_or(loop_sound_handle.duration());
		let mut arrangement = Self::new();
		arrangement
			.add_clip(SoundClip::new(intro_sound_handle, 0.0))
			.add_clip(SoundClip::new(loop_sound_handle, intro_duration))
			.add_clip(
				SoundClip::new(loop_sound_handle, intro_duration + loop_duration)
					.trim(loop_duration),
			);
		arrangement
	}

	/// Adds a sound clip to the arrangement.
	pub fn add_clip(&mut self, clip: SoundClip) -> &mut Self {
		self.duration = self.duration.max(clip.clip_time_range.1);
		self.clips.push(clip);
		self
	}

	/// Gets the duration of the arrangement.
	///
	/// The duration is always the end of the last playing sound clip.
	pub fn duration(&self) -> f64 {
		self.duration
	}
}

impl SoundData for Arrangement {
	fn duration(&self) -> f64 {
		self.duration
	}

	fn frame_at_position(&self, position: f64) -> Frame {
		self.clips
			.iter()
			.fold(Frame::from_mono(0.0), |frame, clip| {
				frame + clip.frame_at_position(position)
			})
	}
}
