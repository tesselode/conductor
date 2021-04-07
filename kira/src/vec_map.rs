use std::{
	iter::Map,
	slice::{Iter, IterMut},
};

pub struct VecMap<K: Eq, V> {
	vec: Vec<(K, V)>,
}

impl<K: Eq, V> VecMap<K, V> {
	pub fn new(capacity: usize) -> Self {
		Self {
			vec: Vec::with_capacity(capacity),
		}
	}

	pub fn len(&self) -> usize {
		self.vec.len()
	}

	pub fn capacity(&self) -> usize {
		self.vec.capacity()
	}

	pub fn get(&self, key: K) -> Option<&V> {
		self.vec.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
	}

	pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
		self.vec.iter_mut().find(|(k, _)| *k == key).map(|(_, v)| v)
	}

	pub fn iter(&self) -> Map<Iter<(K, V)>, fn(&(K, V)) -> &V> {
		self.vec.iter().map(|(_, v)| v)
	}

	pub fn iter_mut(&mut self) -> Map<IterMut<(K, V)>, fn(&mut (K, V)) -> &mut V> {
		self.vec.iter_mut().map(|(_, v)| v)
	}

	pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, V> {
		if self.len() >= self.capacity() {
			return Err(value);
		}
		let previous_item = self.remove(&key);
		self.vec.push((key, value));
		Ok(previous_item)
	}

	pub fn remove(&mut self, key: &K) -> Option<V> {
		self.vec
			.iter()
			.position(|(k, _)| k == key)
			.map(|index| self.vec.remove(index).1)
	}

	pub fn retain(&mut self, mut f: impl FnMut(&V) -> bool) {
		self.vec.retain(|(_, v)| f(v));
	}
}

impl<'a, K: Eq, V> IntoIterator for &'a VecMap<K, V> {
	type Item = &'a V;

	type IntoIter = Map<Iter<'a, (K, V)>, fn(&(K, V)) -> &V>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, K: Eq, V> IntoIterator for &'a mut VecMap<K, V> {
	type Item = &'a mut V;

	type IntoIter = Map<IterMut<'a, (K, V)>, fn(&mut (K, V)) -> &mut V>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
