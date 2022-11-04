use std::cmp::Ordering;
use std::time::Instant;

pub fn sort_by_second(left: &(&String, &usize), right: &(&String, &usize)) -> Ordering {
  left.1.cmp(right.1)
}

pub fn map_deletes(op: &(&str, &str)) -> Option<String> {
  match op.1.len() {
    x if x != 0 => {
      let mut st = String::with_capacity(op.0.len() + op.1.len());
      st.push_str(op.0);
      st.push_str(&op.1[1..]);
      Some(st)
    }
    _ => None,
  }
}

pub fn map_transposes(op: &(&str, &str)) -> Option<String> {
  match op.1.len() {
    x if x > 1 => {
      let mut st = String::with_capacity(op.0.len() + op.1.len());
      st.push_str(op.0);
      st.push_str(&op.1[1..=1]);
      st.push_str(&op.1[0..=0]);
      st.push_str(&op.1[2..]);
      Some(st)
    }
    _ => None,
  }
}

pub fn map_replaces<'a>(
  op: &'a (&'a str, &'a str),
) -> impl IntoIterator<Item = String> + Sync + Send + 'a {
  ('a'..='z')
    .into_iter()
    .filter_map(move |i| match op.1.len() {
      x if x != 0 => {
        let mut st = String::with_capacity(op.0.len() + op.1.len());
        st.push_str(op.0);
        st.push(i);
        st.push_str(&op.1[1..]);
        Some(st)
      }
      _ => None,
    })
}

pub fn map_inserts<'a>(
  op: &'a (&'a str, &'a str),
) -> impl IntoIterator<Item = String> + Sync + Send + 'a {
  ('a'..='z')
    .into_iter()
    .filter_map(move |i| match op.1.len() {
      x if x != 0 => {
        let mut st = String::with_capacity(1 + op.0.len() + op.1.len());
        st.push_str(op.0);
        st.push(i);
        st.push_str(op.1);
        Some(st)
      }
      _ => None,
    })
}

pub fn _timeit<F: FnMut() -> T, T>(mut f: F) -> T {
  let start = Instant::now();
  let result = f();
  let end = start.elapsed();
  println!("it took {}ms", end.as_millis());
  result
}
