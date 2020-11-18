use crate::parameter::{ParameterId, Parameters};

/// A number that something can be set to.
///
/// Can either be a fixed number or the current value
/// of a parameter.
#[derive(Debug, Copy, Clone)]
pub enum Value<T: From<f64> + Copy> {
	Fixed(T),
	Parameter(ParameterId),
}

impl<T: From<f64> + Copy> Value<T> {
	pub(crate) fn get(&self, parameters: &Parameters) -> Option<T> {
		match self {
			Value::Fixed(value) => Some(*value),
			Value::Parameter(id) => parameters
				.get(*id)
				.map(|parameter| T::from(parameter.value())),
		}
	}
}

impl<T: From<f64> + Copy> From<T> for Value<T> {
	fn from(value: T) -> Self {
		Self::Fixed(value)
	}
}

impl<T: From<f64> + Copy> From<ParameterId> for Value<T> {
	fn from(id: ParameterId) -> Self {
		Self::Parameter(id)
	}
}

#[derive(Debug, Clone)]
pub struct CachedValue<T: From<f64> + Copy> {
	value: Value<T>,
	last_value: T,
}

impl<T: From<f64> + Copy> CachedValue<T> {
	pub fn new(value: Value<T>, default_value: T) -> Self {
		Self {
			value,
			last_value: default_value,
		}
	}

	pub fn set(&mut self, value: Value<T>) {
		self.value = value;
	}

	pub fn update(&mut self, parameters: &Parameters) {
		if let Some(value) = self.value.get(parameters) {
			self.last_value = value;
		}
	}

	pub fn value(&self) -> T {
		self.last_value
	}
}
