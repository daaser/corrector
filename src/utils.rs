use std::cmp::Ordering;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn sort_by_second(left: &(&String, &usize), right: &(&String, &usize)) -> Ordering {
  left.1.cmp(right.1)
}

pub fn map_deletes<'a>(op: &'a (&'a str, &'a str)) -> Option<String> {
  match op.1.len() {
    x if x != 0 => Some(format!("{}{}", &op.0, &op.1[1..])),
    _ => None,
  }
}

pub fn map_transposes<'a>(op: &'a (&'a str, &'a str)) -> Option<String> {
  match op.1.len() {
    x if x > 1 => Some(format!(
      "{}{}{}{}",
      &op.0,
      &op.1[1..=1],
      &op.1[0..=0],
      &op.1[2..]
    )),
    _ => None,
  }
}

pub fn map_replaces<'a>(
  op: &'a (&'a str, &'a str),
) -> impl IntoParallelIterator<Item = String> + Sync + Send + 'a {
  ('a'..='z')
    .into_par_iter()
    .filter_map(move |i| match op.1.len() {
      x if x != 0 => Some(format!("{}{}{}", &op.0, i, &op.1[1..])),
      _ => None,
    })
}

pub fn map_inserts<'a>(
  op: &'a (&'a str, &'a str),
) -> impl IntoParallelIterator<Item = String> + Sync + Send + 'a {
  ('a'..='z')
    .into_par_iter()
    .filter_map(move |i| match op.1.len() {
      x if x != 0 => Some(format!("{}{}{}", &op.0, i, &op.1)),
      _ => None,
    })
}
