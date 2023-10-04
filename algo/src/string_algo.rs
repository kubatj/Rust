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
 }
    