mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut count = 1;
        let mut letter = text.chars().next().unwrap();
        let mut str = String::new();
        println!("{}", letter);
        for i in text.chars().skip(1) {
            println!("{}", i);
            if i == letter {
                count = count + 1;
                if count == 10 {
                    str.push_str(&format!("{}{}", count - 1, letter));
                    count = 1;
                }
            } else {
                str.push_str(&format!("{}{}", count, letter));
                println!("{}", letter);
                letter = i;
                count = 1;
            }
        }
        str.push_str(&format!("{}{}", count, letter));
        println!("{}", str);
        return str;
    }

    pub fn decode(text: &str) -> String {
        let mut str = String::new();
        let mut count = 0;
        let mut rep = 0;
        for i in text.chars() {
            if count % 2 == 0 {
                rep = u32::from(i) - 48;
                println!("{}{}", rep, i);
                //str.push_str(&format!("{}", i));
            } else {
                let mut j = 0;
                while j < rep {
                    str.push_str(&format!("{}", i));
                    j = j + 1;
                }
            }
            count = count + 1;
        }
        return str;
    }
}

fn main() {
    //
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
