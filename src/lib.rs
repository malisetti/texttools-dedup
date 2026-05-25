#![forbid(unsafe_code)]

/// Drop consecutive duplicate lines from `input` (like `uniq`).
pub fn dedup_consecutive(input: &str) -> String {
    let mut lines = input.lines();
    let Some(first) = lines.next() else {
        return String::new();
    };

    let mut out = String::from(first);
    let mut prev = first;
    for line in lines {
        if line != prev {
            out.push('\n');
            out.push_str(line);
            prev = line;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_consecutive_duplicate_lines() {
        assert_eq!(dedup_consecutive("a\na\nb\nc\nc"), "a\nb\nc");
    }

    #[test]
    fn empty_input() {
        assert_eq!(dedup_consecutive(""), "");
    }

    #[test]
    fn single_line() {
        assert_eq!(dedup_consecutive("only"), "only");
    }

    #[test]
    fn no_duplicates() {
        assert_eq!(dedup_consecutive("x\ny\nz"), "x\ny\nz");
    }
}
