struct Solution {}

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    let mut istr:Vec<char> = format!("{}", x).chars().collect();
    let mut rstr = istr.clone();
    rstr.reverse();
    istr == rstr
  }
}

fn main() {
    let target = 121_i32;
    println!("result : {}", Solution::is_palindrome(target));
}
