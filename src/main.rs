pub struct Defer(Vec<Box<dyn FnOnce()>>);

impl Defer {
  pub fn new() -> Self {
    Self(vec![])
  }

  pub fn defer(&mut self, f: impl FnOnce() + 'static) -> &mut Self {
    self.0.push(Box::new(f));
    self
  }

  pub fn clear(&mut self) {
    self.0.clear();
  }
}

impl Drop for Defer {
  fn drop(&mut self) {
    for def_fn in std::mem::take(&mut self.0).into_iter().rev() {
      def_fn();
    }
  }
}

fn main() {
  let mut def = Defer::new();
  def
    .defer(|| println!("First Call!"))
    .defer(|| println!("Second Call!"))
    .defer(|| println!("Third Call!"));

  drop(def);
}
