struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roma = s;
        roma = roma.replace("IV", "IIII");
        roma = roma.replace("IX", "IIIIV");
        roma = roma.replace("XL", "XXXX");
        roma = roma.replace("XC", "LXXXX");
        roma = roma.replace("CD", "CCCC");
        roma = roma.replace("CM", "DCCCC");

        let chars = roma.chars();
        let mut result: i32 = 0;
        for c in chars {
            if c == 'I' {
                result = result + 1;
            } else if c == 'V' {
                result = result + 5;
            } else if c == 'X' {
                result = result + 10;
            } else if c == 'L' {
                result = result + 50;
            } else if c == 'C' {
                result = result + 100;
            } else if c == 'D' {
                result = result + 500;
            } else if c == 'M' {
                result = result + 1000;
            }
        }
        return result;
    }
}
