//! Things that can go wrong with sounds.

use thiserror::Error;

/// Something that can go wrong when loading a sound
/// from a file.
#[derive(Debug, Error)]
pub enum SoundFromFileError {
	/// The sound has more than two channels.
	#[error("Only mono and stereo audio is supported")]
	UnsupportedChannelConfiguration,

	/// The sound is in an unsupported file format.
	#[error("Only .mp3, .ogg, .flac, and .wav files are supported")]
	UnsupportedAudioFileFormat,

	/// An error occurred when interacting with the filesystem.
	#[error("{0}")]
	IoError(#[from] std::io::Error),

	/// An error occurred when reading an mp3 file.
	#[cfg(feature = "mp3")]
	#[error("{0}")]
	Mp3Error(#[from] minimp3::Error),

	/// The mp3 file has multiple sample rates.
	#[cfg(feature = "mp3")]
	#[error("mp3s with variable sample rates are not supported")]
	VariableMp3SampleRate,

	/// The sample rate of the mp3 could not be determined.
	#[cfg(feature = "mp3")]
	#[error("Could not get the sample rate of the mp3")]
	UnknownMp3SampleRate,

	/// An error occurred when reading an ogg file.
	#[cfg(feature = "ogg")]
	#[error("{0}")]
	OggError(#[from] lewton::VorbisError),

	/// An error occurred when reading a flac file.
	#[cfg(feature = "flac")]
	#[error("{0}")]
	FlacError(#[from] claxon::Error),

	/// An error occurred when reading a wav file.
	#[cfg(feature = "wav")]
	#[error("{0}")]
	WavError(#[from] hound::Error),
}
