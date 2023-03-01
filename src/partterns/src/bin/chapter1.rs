///
/// 使用借用类型作为参数
/// 

fn main() {
  let f = "abc".to_string();
  three_vowels(&f);
}

fn three_vowels(word: &str) -> bool {
  let mut count = 0;
  for char in word.chars() {
    match char {
        'a' | 'e' | 'i' | 'o' | 'u' => {
          count += 1;
          if count >= 3 {
            return true
          }
        },
        _ => count = 0 
    }
  }
  false
}
