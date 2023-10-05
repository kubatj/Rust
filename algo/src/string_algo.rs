/*
This question is asked by Google. Given a string, reverse all of its characters and return the resulting string.
Ex: Given the following strings...
“Cat”, return “taC”
“The Daily Byte”, return "etyB yliaD ehT”
“civic”, return “civic”
 */

use std::ascii::AsciiExt;

 pub fn reverse_string(word: String) -> String {

   let mut res: Vec<char> = word.chars().collect();
   let mut i: usize = 0 ;
   let mut k: usize = res.len() -1;

   while i < k {

      let temp = res[i];
      res[i] = res[k];
      res[k] = temp;

      i += 1;
      k -= 1;
   }

   res.iter().collect()
 }


 /*
 This question is asked by Facebook. Given a string, return whether or not it forms a palindrome ignoring case and non-alphabetical characters. 
Note: a palindrome is a sequence of characters that reads the same forwards and backwards. 

Ex: Given the following strings...
"level", return true
"algorithm", return false
"A man, a plan, a canal: Panama.", return true
*/


pub fn is_palindrome(str: String) -> bool {
  // base solution
  // two pointer and compare while i < k 

  let mut i: usize = 0; 
  let mut k: usize = str.len() -1 ;
  let array: Vec<char> = str.chars().collect();


  while i < k {

    if !array[i].is_alphabetic() || array[i].is_digit(10) {
      i+= 1;
    }

    else if !array[k].is_alphabetic() || array[k].is_digit(10) {
      k-= 1;
    }

    else if array[i].to_ascii_lowercase() != array[k].to_ascii_lowercase() {
      return false
    }

    else {

      i+=1;
      k-=1;
    }
  }

  true

}


 // test module
 #[cfg(test)]
 mod tests {
   use super::*;

   #[test]
   fn test_reserse_string() {

      assert_eq!(reverse_string(String::from("Cat")), "taC");
      assert_eq!(reverse_string(String::from("The Daily Byte")), "etyB yliaD ehT");
      assert_eq!(reverse_string(String::from("civic")), "civic");
   }

   #[test]
   fn test_is_palindrome() {

    assert_eq!(is_palindrome(String::from("level")), true);
    assert_eq!(is_palindrome(String::from("algorithm")), false);
    assert_eq!(is_palindrome(String::from("A man, a plan, a canal: Panama.")), true)
   }
 }
    