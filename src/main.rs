use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

use corrector::Corrector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut corrector = Corrector::new();
  let fallback = "/usr/share/dict/words".to_string();
  let filepath = env::args().nth(1).unwrap_or(fallback);
  corrector.load(filepath)?;

  let test_set = load_test_set("spell-testset1.txt")?;
  for (expected, words) in test_set {
    println!("{expected}:");
    for word in words {
      println!(
        "\t{:?} {:?}",
        word,
        corrector.correct(&word).unwrap_or_default()
      );
    }
  }

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

pub fn load_test_set<F: AsRef<str>>(filename: F) -> io::Result<Vec<(String, Vec<String>)>> {
  let file = File::open(filename.as_ref())?;
  let buf_reader = BufReader::new(file);
  let mut ret = Vec::new();

  for line in buf_reader.lines() {
    let line = line?;
    ret.extend(line.split_once(':').map(|(expected, rest)| {
      let rest = rest.split_ascii_whitespace().map(|x| x.to_string());
      (expected.to_string(), rest.collect())
    }))
    // for word in line.split_whitespace() {
    //   let word = word.to_ascii_lowercase();
    //   let entry = self.dictionary.entry(word);
    //   entry.and_modify(|e| *e += 1).or_insert(1);
    // }
  }

  Ok(ret)
}
