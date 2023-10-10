/*
This question is asked by Google. Given a string, reverse all of its characters and return the resulting string.
Ex: Given the following strings...
“Cat”, return “taC”
“The Daily Byte”, return "etyB yliaD ehT”
“civic”, return “civic”
 */

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

/*
This question is asked by Amazon. Given a string representing the sequence of moves a robot vacuum makes, return whether or not it will return to its original position. The string will only contain L, R, U, and D characters, representing left, right, up, and down respectively.
Ex: Given the following strings...
"LR", return true
"URURD", return false
"RUULLDRD", return true
 */

 fn vacuum_cleaner(path: String) -> bool {
  let mut x = 0;
  let mut y = 0; 
  let iterable_str: Vec<char> = path.chars().collect();


  for direction in iterable_str {

    match direction {
      'L' => x -= 1,
      'R' => x += 1,
      'D' => y -= 1,
      'U' => y += 1,
      _ => (),
    }
  }

  x == 0 && y ==0
 }


 /*
  This question is asked by Google. Given a string, return whether or not it uses capitalization correctly. A string correctly uses capitalization if all letters are capitalized, no letters are capitalized, or only the first letter is capitalized.

  Ex: Given the following strings...
  "USA", return true
  "Calvin", return true
  "compUter", return false
  "coding", return true
  */

  fn is_capitalization_correct(word: String) -> bool {
    let iter: Vec<char> = word.chars().collect();

    if iter[0].is_ascii_uppercase() && iter[1].is_ascii_uppercase() {
      let mut i = 2; 

      while i < iter.len() { 
        if !iter[i].is_ascii_uppercase() {
          return false
        }

        i+=1;

      }
    }

    else if iter[0].is_ascii_uppercase() && !iter[1].is_ascii_uppercase() {
     let mut i = 2; 

     while i < iter.len() {
      if iter[i].is_ascii_uppercase() {
        return false
      }
      
      i += 1 ;
     } 
      
    }

    else if !iter[0].is_ascii_uppercase() {
      for c in iter {
        if c.is_ascii_uppercase() {
          return false
        }
      }
      
    }


    true
  }

  /*
     This question is asked by Apple. Given two binary strings (strings containing only 1s and 0s) return their sum (also as a binary string). 
    Note: neither binary string will contain leading 0s unless the string itself is 0 

    Ex: Given the following binary strings...
    "100" + "1", return "101"
    "11" + "1", return "100"
    "1" + "0", return  "1"
   */

  fn binary_add(binary_a: String, binary_b: String) -> String {

    let array_a: Vec<char> = binary_a.chars().collect();
    let array_b: Vec<char> = binary_b.chars().collect();

    let mut result = Vec::new();
    let mut carry = 0;

    let max_len = array_a.len().max(array_b.len());

    for i in 0..max_len { 

      let a_digit = if i < array_a.len() {
        array_a[array_a.len() - 1 - i].to_digit(2).unwrap()
      } else {
        0
      };

      let b_digit = if i < array_b.len() {
        array_b[array_b.len() - 1 - i].to_digit(2).unwrap()
      } else {
        0
      };

      let sum = a_digit + b_digit + carry;
      result.push((sum % 2).to_string());
      carry = sum/2;
    }

    if carry > 0 {
      result.push("1".to_string())
    }

    result.reverse();
    result.join("")
  } 

  /*
    This question is asked by Microsoft. Given an array of strings, return the longest common prefix that is shared amongst all strings. 
    Note: you may assume all strings only contain lowercase alphabetical characters. 

    Ex: Given the following arrays...
    ["colorado", "color", "cold"], return "col"
    ["a", "b", "c"], return ""
    ["spot", "spotty", "spotted"], return "spot"
   */


 // fn longest_common_prefix(words: Array<String>) -> String {

 // }



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

   #[test]
   fn test_vaccum_cleaner() {
    assert_eq!(vacuum_cleaner(String::from("LR")), true);
    assert_eq!(vacuum_cleaner(String::from("URURD")), false);
    assert_eq!(vacuum_cleaner(String::from("RUULLDRD")), true)
   }

   #[test]
   fn test_is_capitalize_correctly() { 
    assert_eq!(is_capitalization_correct(String::from("USA")), true);
    assert_eq!(is_capitalization_correct(String::from("Calvin")), true);
    assert_eq!(is_capitalization_correct(String::from("compUter")), false);
    assert_eq!(is_capitalization_correct(String::from("coding")), true);
   }

   #[test]
   fn test_binary_add() {
    assert_eq!(binary_add(String::from("100"), String::from("1")), "101");
    assert_eq!(binary_add(String::from("11"), String::from("1")), "100");
    assert_eq!(binary_add(String::from("1"), String::from("0")), "1");
   }
 }
    