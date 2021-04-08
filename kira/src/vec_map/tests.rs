use super::VecMap;

#[test]
fn inserts_key_value_tuples() {
	let mut vec_map = VecMap::new(10);
	vec_map.insert(0, 0).unwrap();
	vec_map.insert(1, 1).unwrap();
	vec_map.insert(2, 2).unwrap();
	assert!(
		vec_map.vec[0] == (0, 0),
		"VecMap should insert key value tuples in order"
	);
	assert!(
		vec_map.vec[1] == (1, 1),
		"VecMap should insert key value tuples in order"
	);
	assert!(
		vec_map.vec[2] == (2, 2),
		"VecMap should insert key value tuples in order"
	);
}

#[test]
fn replaces_previous_value_with_key_on_insert() {
	let mut vec_map = VecMap::new(10);
	vec_map.insert(1, 1).unwrap();
	vec_map.insert(1, 2).unwrap();
	assert!(
		vec_map.vec.len() == 1 && vec_map.vec[0] == (1, 2),
		"VecMap should not retain old value associated with key"
	);
}

#[test]
fn returns_previous_value_with_key_on_insert() {
	let mut vec_map = VecMap::new(10);
	vec_map.insert(1, 1).unwrap();
	let previous = vec_map.insert(1, 2).unwrap();
	assert!(
		previous == Some(1),
		"VecMap should return the value previously associated with the key"
	);
}

#[test]
fn returns_error_on_exceeded_capacity() {
	let mut vec_map = VecMap::new(1);
	assert!(
		vec_map.insert(1, 1).is_ok(),
		"insert should return Ok when the VecMap is not full"
	);
	assert!(
		vec_map.insert(1, 2).is_ok(),
		"previous value with key should be removed before trying to add a new one"
	);
	assert!(
		vec_map.insert(2, 3).is_err(),
		"insert should return Err when the VecMap is full"
	);
}

#[test]
fn removes_values_by_key() {
	let mut vec_map = VecMap::new(10);
	vec_map.insert(1, 1).unwrap();
	vec_map.insert(2, 2).unwrap();
	vec_map.remove(&1);
	assert!(
		vec_map.vec.len() == 1 && vec_map.vec[0] == (2, 2),
		"VecMap should remove values by key"
	);
}

#[test]
fn returns_removed_value() {
	let mut vec_map = VecMap::new(10);
	vec_map.insert(1, 1).unwrap();
	vec_map.insert(2, 2).unwrap();
	assert!(
		vec_map.remove(&1) == Some(1),
		"VecMap should return the value it removed"
	);
}

#[test]
fn retains_values() {
	let mut vec_map = VecMap::new(10);
	vec_map.insert(5, 1).unwrap();
	vec_map.insert(4, 2).unwrap();
	vec_map.insert(3, 3).unwrap();
	vec_map.insert(2, 4).unwrap();
	vec_map.insert(1, 5).unwrap();
	vec_map.retain(|value| *value > 3);
	assert!(
		vec_map.len() == 2 && vec_map.vec[0] == (2, 4) && vec_map.vec[1] == (1, 5),
		"VecMap::retain should remove values by the predicate"
	);
}
