use std::env;

fn is_morse(query: &str) -> bool {
    query
        .replace(" ", "")
        .replace("\t", "")
        .replace("-", "")
        .replace(".", "")
        .is_empty()
}

fn main() {
    let mut args_iter = env::args();
    args_iter.next();

    let query = args_iter
        .collect::<Vec<String>>()
        .join(" ")
        .replace("_", "-");

    if is_morse(&query) {
        println!("valid morse");
    } else {
        println!("not morse");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_morse() {
        assert!(is_morse("--.  .--\t\t..."));
        assert!(!is_morse("abc"));
    }
}
