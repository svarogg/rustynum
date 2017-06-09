use super::tripletizer::*;

const SIZES: &'static [&'static str] = &["", " thousand", " million", " billion", " trillion"];

fn parse_sign(n: i32) -> String {
    return String::from(if n < 0 { "minus " } else { "" });
}

fn parse_atomic(n: i32) -> Option<String> {
    let result = match n {
        0 => Some("zero"),
        1 => Some("one"),
        2 => Some("two"),
        3 => Some("three"),
        4 => Some("four"),
        5 => Some("five"),
        6 => Some("six"),
        7 => Some("seven"),
        8 => Some("eight"),
        9 => Some("nine"),
        10 => Some("ten"),
        11 => Some("eleven"),
        12 => Some("twelve"),
        13 => Some("thirteen"),
        14 => Some("fourteen"),
        15 => Some("fifteen"),
        16 => Some("sixteen"),
        17 => Some("seventeen"),
        18 => Some("eighteen"),
        19 => Some("nineteen"),
        20 => Some("twenty"),
        30 => Some("thirty"),
        40 => Some("forty"),
        50 => Some("fifty"),
        60 => Some("sixty"),
        70 => Some("seventy"),
        80 => Some("eighty"),
        90 => Some("ninety"),
        _ => None,
    };
    match result {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

fn parse_duplet(n: i32) -> String {
    fn parse_non_atomic(n: i32) -> String {
        format!("{} {}",
                parse_atomic(n - (n % 10)).unwrap(),
                parse_atomic(n % 10).unwrap())
    }

    parse_atomic(n).unwrap_or(parse_non_atomic(n))
}

fn parse_triplet(mut n: i32) -> String {
    if n == 0 {
        return "".to_string();
    }

    let mut result = Vec::new();
    if n >= 100 {
        result.push(format!("{} hundred", parse_atomic(n / 100).unwrap()));
        n = n % 100;
    }

    result.push(parse_duplet(n));

    result.join(" ")
}


fn parse_num(n: i32) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let triplets = Tripletizer::new(n);

    let mut parsed_triplets = triplets
        .enumerate()
        .filter(|triplet_with_index| (*triplet_with_index).1 != 0)  // filter all 0 triplets
        .map(|(i, triplet)| format!("{}{}", parse_triplet(triplet), SIZES[i]))
        .collect::<Vec<_>>();

    parsed_triplets.reverse();

    parsed_triplets.join(", ")
}

pub fn num_to_string(n: i32) -> String {
    format!("{}{}", parse_sign(n), parse_num(n.abs()))
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
        assert_eq!("two billion, one hundred forty seven million, four hundred eighty three thousand, six hundred forty seven",
                   num_to_string(2147483647))
    }
}
