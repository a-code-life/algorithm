impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;
        let mut length_last_word = 0;
        for c in s.chars() {
            if (c == ' ') {
                if (length != 0){
                    length_last_word = length;
                }
                length = 0;
            } else {
                length = length + 1;
                length_last_word = length;
            }
        }
        return length_last_word;
    }
}
