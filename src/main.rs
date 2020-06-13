use std::env;

fn main() {
    // get the command line argument
    let args: Vec<String> = env::args().collect();

    // first command line arg is the program
    if args.len() < 2 {
        panic!("must supply a command line argument of the number to convert");
    }

    // parse or fail
    let i = args[1].parse::<i32>().expect("Unable to parse command line argument to int");

    // compute the roman numeral
    let rn = make_roman_numeral(i);
    println!("made roman numeral for {:?} - {:?}", i, rn);
}

// associate a roman numeral with its value
struct RomanNumeral {
    txt: String,
    value: i32,
}

fn get_roman_numerals_descending() -> Vec<RomanNumeral> {
    vec![
        RomanNumeral {txt: "M".to_owned(),value: 1000},
        RomanNumeral {txt: "D".to_owned(),value: 500},
        RomanNumeral {txt: "C".to_owned(),value: 100},
        RomanNumeral {txt: "L".to_owned(),value: 50},
        RomanNumeral {txt: "X".to_owned(),value: 10},
        RomanNumeral {txt: "V".to_owned(),value: 5},
        RomanNumeral {txt: "I".to_owned(),value: 1},
    ]
}

fn is_factor_of_ten(i: i32) -> bool {
    if i == 1 {
        return true;
    }

    let div_ten = i / 10;

    // values will floor to zero if they're not a factor of 10
    if div_ten == 0 {
        return false;
    }

    return is_factor_of_ten(div_ten);
}

fn subtractor_value(desc_numerals: &Vec<RomanNumeral>, current_pos: usize) -> Option<RomanNumeral> {
    let curr = desc_numerals.get(current_pos).unwrap();
    let mut next_numeral = String::new();

    match desc_numerals.get(current_pos + 1) {
        None => {
            return None;
        },
        Some(next_rn) => {
            if is_factor_of_ten(next_rn.value) {
                next_numeral.push_str(&next_rn.txt);
                next_numeral.push_str(&curr.txt);
                return Some(RomanNumeral{
                    txt: next_numeral,
                    value: curr.value - next_rn.value
                });
            }

            match desc_numerals.get(current_pos + 2) {
                None => None,
                Some(next2_rn) => {
                    if is_factor_of_ten(next2_rn.value) {
                        next_numeral.push_str(&next2_rn.txt);
                        next_numeral.push_str(&curr.txt);
                        return Some(RomanNumeral{
                            txt: next_numeral,
                            value: curr.value - next2_rn.value
                        });
                    }
                    return None;
                }
            }
        }
    }
}

fn make_roman_numeral(my_int: i32) -> String {
    let descending = get_roman_numerals_descending();
    let mut roman_numeral = String::new();
    let mut remainder = my_int;

    // find the largest roman numeral smaller than remainder,
    // append the roman character, and deduct from the remainder,
    // if roman numeral is not smaller, try the next valid subtractor.
    // e.g. if remainder is smaller than 10, X, then check if remainder
    // is smaller than 9, IX, else go down to next numeral, V.
    while remainder > 0 {
        if remainder == 0 {
            break;
        }

        for (i, rn) in descending.iter().enumerate() {
            if rn.value <= remainder {
                roman_numeral.push_str(&rn.txt);
                remainder -= rn.value;
                break;
            }

            let next_valid_subtractor = subtractor_value(&descending, i);
            if next_valid_subtractor.is_some() {
                let next_sub = next_valid_subtractor.unwrap();
                if next_sub.value <= remainder {
                    roman_numeral.push_str(&next_sub.txt);
                    remainder -= next_sub.value;
                    break;
                }
            }
        }
    }

    roman_numeral
}
