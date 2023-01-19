/// Divides the sum of the list by the length of the given list, returning its mean (average)
fn mean(list: Vec<u128>) -> f64 {
  let mut sum = 0;
  for x in list.clone() {
    sum += x
  }
  sum as f64 / (list.len() as f64)
}

/// Custom Enumerator to help with code reuse
#[derive(Clone, Copy)]
pub enum TestType {
  BF1,
  BF2,
  OE,
  SE,
}

/// Custom data type to handle the data from the tests
#[derive(Clone)]
pub struct Test {
  pub times: Vec<u128>,
  pub maximum: u128,
  pub minimum: u128,
  pub mean: f64,
  pub median: u128
}

impl Test {
  pub fn new(t: Vec<u128>, max: u128, min: u128, mean: f64, med: u128) -> Test {
    Test { times: t, maximum: max, minimum: min, mean: mean, median: med }
  }

  pub fn times(&self) -> Vec<u128>{
    self.times.clone()
  }

  pub fn maximum(&self) -> u128{
    self.maximum
  }

  pub fn minimum(&self) -> u128{
    self.minimum
  }

  pub fn mean(&self) -> f64{
    self.mean
  }

  pub fn median(&self) -> u128{
    self.median
  }

  /// Returns the amount of items in &self that are better than the right hand side and the average time saved from those in the given Test
  pub fn compare_to(&self, rhs: Test) -> (u32, f64) {
    let main = self.times();
    let comp = rhs.times();
    let mut count = 0;
    let mut times_saved = Vec::<u128>::new();
    for i in 0..main.len() {
      if main.get(i).unwrap() <= comp.get(i).unwrap() {
        count += 1;
        times_saved.push(comp.get(i).unwrap()-main.get(i).unwrap());
      }
    }
    
    (count, mean(times_saved))
  }
}
