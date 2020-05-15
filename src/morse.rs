use regex::Regex;

pub fn is_morse(query: &str) -> bool {
    Regex::new("^[ \t\n-.]*$").unwrap().is_match(query)
}

pub fn translate_morse(query: &str) -> String {
    let query = &query.replace("\t", "  ");
    let morse = Regex::new("  +").unwrap().replace_all(query, "  ");
    let mut latin = String::new();

    for line in morse.lines() {
        if !latin.is_empty() {
            latin.push_str("\n");
        }
        for word in line.split("  ") {
            if !latin.is_empty() && !latin.ends_with("\n") {
                latin.push_str(" ");
            }
            for letter in word.split(" ") {
                latin.push_str(match letter {
                    ".-" => "A",
                    "-..." => "B",
                    "-.-." => "C",
                    "-.." => "D",
                    "." => "E",
                    "..-." => "F",
                    "--." => "G",
                    "...." => "H",
                    ".." => "I",
                    ".---" => "J",
                    "-.-" => "K",
                    ".-.." => "L",
                    "--" => "M",
                    "-." => "N",
                    "---" => "O",
                    ".--." => "P",
                    "--.-" => "Q",
                    ".-." => "R",
                    "..." => "S",
                    "-" => "T",
                    "..-" => "U",
                    "...-" => "V",
                    ".--" => "W",
                    "-..-" => "X",
                    "-.--" => "Y",
                    "--.." => "Z",

                    ".----" => "1",
                    "..---" => "2",
                    "...--" => "3",
                    "....-" => "4",
                    "....." => "5",
                    "-...." => "6",
                    "--..." => "7",
                    "---.." => "8",
                    "----." => "9",
                    "-----" => "0",

                    _ => "#",
                });
            }
        }
    }

    latin
}

pub fn translate_latin(query: &str) -> String {
    let query = Regex::new("[ \t]+")
        .unwrap()
        .replace_all(&query, " ")
        .to_uppercase();
    let latin = Regex::new("[^ \nA-Z0-9]").unwrap().replace_all(&query, "");
    let mut morse = String::new();

    for line in latin.lines() {
        if !morse.is_empty() {
            morse.push_str("\n");
        }
        for word in line.split(" ") {
            if !(morse.is_empty() || morse.ends_with("\n")) {
                morse.push_str("   ");
            }
            for letter in word.chars() {
                if !(morse.is_empty() || morse.ends_with("\n") || morse.ends_with(" ")) {
                    morse.push_str(" ");
                }
                morse.push_str(match letter {
                    'A' => ".-",
                    'B' => "-...",
                    'C' => "-.-.",
                    'D' => "-..",
                    'E' => ".",
                    'F' => "..-.",
                    'G' => "--.",
                    'H' => "....",
                    'I' => "..",
                    'J' => ".---",
                    'K' => "-.-",
                    'L' => ".-..",
                    'M' => "--",
                    'N' => "-.",
                    'O' => "---",
                    'P' => ".--.",
                    'Q' => "--.-",
                    'R' => ".-.",
                    'S' => "...",
                    'T' => "-",
                    'U' => "..-",
                    'V' => "...-",
                    'W' => ".--",
                    'X' => "-..-",
                    'Y' => "-.--",
                    'Z' => "--..",

                    '1' => ".----",
                    '2' => "..---",
                    '3' => "...--",
                    '4' => "....-",
                    '5' => ".....",
                    '6' => "-....",
                    '7' => "--...",
                    '8' => "---..",
                    '9' => "----.",
                    '0' => "-----",

                    _ => "#",
                });
            }
        }
    }

    morse
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_morse() {
        assert!(is_morse("--.  .--\t\t..."));
        assert!(!is_morse("abc"));
    }

    #[test]
    fn test_translate_morse() {
        assert_eq!("THE QUICK BROWN FOX JUMPED OVER THE LAZY DOGS BACK\n1234567890", translate_morse("- .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-   .--- ..- -- .--. . -..   --- ...- . .-.   - .... .   .-.. .- --.. -.--   -.. --- --. ...   -... .- -.-. -.-\n.---- ..--- ...-- ....- ..... -.... --... ---.. ----. -----"));
    }

    #[test]
    fn test_translate_latin() {
        assert_eq!("- .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-   .--- ..- -- .--. . -..   --- ...- . .-.   - .... .   .-.. .- --.. -.--   -.. --- --. ...   -... .- -.-. -.-\n.---- ..--- ...-- ....- ..... -.... --... ---.. ----. -----", translate_latin("THE QUICK BROWN FOX JUMPED OVER THE LAZY DOGS BACK\n1234567890"));
    }
}
