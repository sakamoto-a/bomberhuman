pub struct Buttons {
  pub button: Vec<&'static str>,
}

impl Buttons {
  pub fn new() -> Buttons {
    Buttons {
      button: Vec::new(),
    }
  }
}
