#[cfg(test)]
mod tests {
  use crate::tests::helpers::tests::assert_get;

  #[test]
  fn test_health() {
    assert_get("/health");
  }
}
