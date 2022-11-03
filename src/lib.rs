pub mod util;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use rayon::prelude::*;

use crate::util::*;

type Dictionary = HashMap<String, usize>;

pub struct Corrector {
  dictionary: Dictionary,
}

#[rustfmt::skip]
impl Corrector {

pub fn new() -> Self {
  Self {
    dictionary: HashMap::new(),
  }
}

pub fn load<F: AsRef<str>>(&mut self, filename: F) -> io::Result<()> {
  let file = File::open(filename.as_ref())?;
  let buf_reader = BufReader::new(file);

  for line in buf_reader.lines() {
    let line = line?;
    for word in line.split_whitespace() {
      let word = word.to_ascii_lowercase();
      let entry = self.dictionary.entry(word);
      entry.and_modify(|e| *e += 1).or_insert(1);
    }
  }

  Ok(())
}

pub fn correct(&self, word: &String) -> Option<String> {
  let mut results = Vec::with_capacity(1024);
  let mut candidates = Dictionary::new();

  if self.dictionary.contains_key(word) {
    return None;
  }

  self.edits(word, &mut results);
  self.known(&results, &mut candidates);

  if !candidates.is_empty() {
    return match candidates.par_iter().max_by(sort_by_second) {
      Some((key, _)) => Some(key.clone()),
      _ => None,
    };
  }

  for result in results {
    let mut sub_results = Vec::with_capacity(512);
    self.edits(&result, &mut sub_results);
    self.known(&sub_results, &mut candidates);
  }

  if !candidates.is_empty() {
    return match candidates.par_iter().max_by(sort_by_second) {
      Some((key, _)) => Some(key.clone()),
      _ => None,
    };
  }

  None
}

fn edits(&self, word: &String, results: &mut Vec<String>) {
  let splits = (0..word.len())
    .into_par_iter()
    .map(|i| (&word[0..i], &word[i..]))
    .collect::<Vec<_>>();

  results.extend(splits.iter().filter_map(map_deletes));
  results.extend(splits.iter().filter_map(map_transposes));
  results.extend(splits.iter().flat_map(map_replaces));
  results.extend(splits.iter().flat_map(map_inserts));
}

fn known(&self, results: &Vec<String>, candidates: &mut Dictionary) {
  for result in results {
    let value = self.dictionary.get_key_value(result);
    if let Some((key, value)) = value {
      candidates.insert(key.clone(), *value);
    }
  }
}

} // impl Corrector

#[rustfmt::skip]
impl Default for Corrector {

fn default() -> Self {
  Self::new()
}

} // impl Default for Corrector
