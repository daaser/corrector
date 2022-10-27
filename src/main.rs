use std::env;

use corrector::Corrector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut corrector = Corrector::new();
  let fallback = "/usr/share/dict/words".to_string();
  let filepath = env::args().nth(1).unwrap_or(fallback);
  corrector.load(filepath)?;

  let word = "speling".to_string();
  println!("{:?}", corrector.correct(&word));
  let word = "korrectud".to_string();
  println!("{:?}", corrector.correct(&word));

  // print!("Type one word: ");
  // loop {
  //   let request: String = text_io::read!("{}\n");
  //   match corrector.correct(&request) {
  //     Some(correct) => println!("Did you mean: {correct}?"),
  //     None => println!("No correction available"),
  //   }
  //   print!("Type one word: ");
  // }
  Ok(())
}

#[cfg(test)]
mod tests {
  use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

  use super::*;
  use std::fs::File;
  use std::io::{self, BufRead, BufReader};
  use std::sync::atomic::{AtomicI64, Ordering};
  use std::time::Instant;

  fn load_test_set<F: AsRef<str>>(filename: F) -> io::Result<Vec<(String, Vec<String>)>> {
    let file = File::open(filename.as_ref())?;
    let buf_reader = BufReader::new(file);
    let mut ret = Vec::new();

    for line in buf_reader.lines() {
      let line = line?;
      ret.extend(line.split_once(':').map(|(expected, rest)| {
        let rest = rest.split_ascii_whitespace().map(|x| x.to_string());
        (expected.to_string(), rest.collect())
      }))
    }

    Ok(ret)
  }

  #[test]
  fn benchmark() {
    let mut corrector = Corrector::new();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let big = manifest_dir.clone() + "/big.txt";
    let test_set = manifest_dir.clone() + "/spell-testset1.txt";
    corrector.load(big).unwrap();

    let test_set = load_test_set(test_set).unwrap();
    let good = AtomicI64::new(0);
    let n = AtomicI64::new(0);
    let start = Instant::now();

    test_set.par_iter().for_each(|(expected, words)| {
      // for (expected, words) in test_set {
      for word in words {
        let w = corrector.correct(word).unwrap_or("\"nothing\"".to_string());
        if w == *expected {
          good.fetch_add(1, Ordering::Relaxed);
        }
        n.fetch_add(1, Ordering::Relaxed);
        println!("correct({}) => {}; expected {}", word, w, expected,);
      }
    });

    let elapsed = start.elapsed();
    let good = good.load(Ordering::Relaxed) as f64;
    let n = n.load(Ordering::Relaxed) as f64;
    println!(
      "{:.2}% of {:.0} correct at {:.3} words per second",
      (good / n) * 100.0,
      n,
      (n / elapsed.as_secs_f64()),
    );
  }
}
