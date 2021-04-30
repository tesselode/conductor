use std::ops::{Div, DivAssign, Mul, MulAssign};

use crate::tempo::Tempo;

/// A duration of time.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
	feature = "serde_support",
	derive(serde::Serialize, serde::Deserialize)
)]
pub enum Duration {
	/// A duration of time in seconds.
	Seconds(f64),
	/// A duration of time in beats.
	Beats(f64),
}

impl Duration {
	/// Gets the duration in seconds.
	pub fn in_seconds(&self, tempo: Tempo) -> f64 {
		match self {
			Duration::Seconds(seconds) => *seconds,
			Duration::Beats(beats) => tempo.beats_to_seconds(*beats),
		}
	}
}

impl Mul<f64> for Duration {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		match self {
			Duration::Seconds(seconds) => Duration::Seconds(seconds * rhs),
			Duration::Beats(beats) => Duration::Beats(beats * rhs),
		}
	}
}

impl MulAssign<f64> for Duration {
	fn mul_assign(&mut self, rhs: f64) {
		match self {
			Duration::Seconds(seconds) => {
				*seconds *= rhs;
			}
			Duration::Beats(beats) => {
				*beats *= rhs;
			}
		}
	}
}

impl Div<f64> for Duration {
	type Output = Self;

	fn div(self, rhs: f64) -> Self::Output {
		match self {
			Duration::Seconds(seconds) => Duration::Seconds(seconds / rhs),
			Duration::Beats(beats) => Duration::Beats(beats / rhs),
		}
	}
}

impl DivAssign<f64> for Duration {
	fn div_assign(&mut self, rhs: f64) {
		match self {
			Duration::Seconds(seconds) => {
				*seconds /= rhs;
			}
			Duration::Beats(beats) => {
				*beats /= rhs;
			}
		}
	}
}
