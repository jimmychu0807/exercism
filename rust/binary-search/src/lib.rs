use std::cmp::{Ord, Ordering};

pub fn find<T: Ord, C: AsRef<[T]>>(container: C, key: T) -> Option<usize> {
  let array = container.as_ref();
  if array.is_empty() { return None; }

  let mut s_ind: usize = 0;
  let mut e_ind: usize = array.len() - 1;

  loop {
    let m_ind: usize = (e_ind + s_ind) / 2;
    if key.cmp(&array[m_ind]) == Ordering::Equal { return Some(m_ind); }
    else if s_ind == e_ind { return None; }
    else if key.cmp(&array[m_ind]) == Ordering::Greater { s_ind = m_ind + 1; }
    else if m_ind > 0 { e_ind = m_ind - 1; }
    else { return None; }
  }
}
