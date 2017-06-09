use super::tripletizer::Tripletizer;

const SIZES: &'static [&'static str] = &["", "thousand", "million", "billion", "trillion"];

fn parse_sign(n: i32) -> String {
    return String::from(if n < 0 { "minus " } else { "" });
}

fn parse_num(n: i32) -> String {
    let triplets = Tripletizer::new(n);
    return triplets
               .map(|x| x.to_string())
               .collect::<Vec<_>>()
               .join(",");
}

pub fn num_to_string(n: i32) -> String {
    return format!("{}{}", parse_sign(n), parse_num(n.abs()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_string() {
        assert_eq!("zero", num_to_string(0));
        assert_eq!("one", num_to_string(1));
        assert_eq!("minus one", num_to_string(-1));
        assert_eq!("ten", num_to_string(10));
        assert_eq!("twelve", num_to_string(12));
        assert_eq!("twenty two", num_to_string(22));
        assert_eq!("one million, three hundred fifty five thousand, eight hundred twenty three",
                   num_to_string(1355823));
        assert_eq!("one thousand", num_to_string(1000));
        assert_eq!("one million", num_to_string(1000000));
        assert_eq!("one million, one thousand", num_to_string(1001000));
        assert_eq!("minus one million, three hundred fifty five thousand, eight hundred twenty three",
                   num_to_string(-1355823));

    }
}
