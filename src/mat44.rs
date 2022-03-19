pub struct Mat44 {
  data: [[f64; 4]; 4],
}

impl ops::Index<usize> for Mat44 {
  type Output = [f64; 4];
  fn index(&self, index: usize) -> &[f64; 4] {
    &self.data[index]
  }
}
