use std::collections::HashMap;





fn main(){
    let mut letters = HashMap::new();
    for ch in "acdddeaaaddawttw".chars() {
        let le = letters.entry(ch).or_insert(0);
        *le += 1;
    }

}
