//！ 
//! # Test_release
//! 
//! my_crate 是一系列

/// 将传入的数字加1
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer =  test_release::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}

#[test]
fn aaa() {
  let arg = 5;
  let answer = crate::add_one(arg);
  assert_eq!(6, answer)
}