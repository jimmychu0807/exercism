use std::cmp::{Ord, Ordering};

pub fn find<T: Ord, C: AsRef<[T]>>(container: C, key: T) -> Option<usize> {
	let array = container.as_ref();
	if array.is_empty() {
		return None;
	}

	let mut left_idx: usize = 0;
	let mut right_idx: usize = array.len() - 1;
	let last_idx = right_idx;

	while left_idx <= right_idx {
		let m_idx: usize = (right_idx + left_idx) / 2;
		match key.cmp(&array[m_idx]) {
			Ordering::Equal => {
				return Some(m_idx);
			}
			Ordering::Less if m_idx == 0 => {
				return None;
			}
			Ordering::Less => {
				right_idx = m_idx - 1;
			}
			Ordering::Greater if m_idx == last_idx => {
				return None;
			}
			Ordering::Greater => {
				left_idx = m_idx + 1;
			}
		}
	}

	None
}
