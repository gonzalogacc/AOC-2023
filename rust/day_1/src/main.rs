use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn load_data(filename: String) -> Vec<String> {
    // Load file raw data
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => data.push(line),
            Err(e) => println!("Error: {}", e),
        }
    }
    data
}


fn get_string_calibration(input: String) -> i32 {
    let mut calib: String = String::from("");
    for c in input.chars() {
        if c.is_digit(10) {
            calib.push(c);
        }
    }
    if calib.len() == 0 {
        return 0
    }
    let mut result: String = String::from(calib.chars().nth(0).unwrap());
    result.push(calib.chars().last().unwrap());
    return result.parse::<i32>().unwrap();
}

fn replace_string_numbers(replace: String) -> String {
    let mut string_numbers: HashMap<&str, &str> = HashMap::new();
    string_numbers.insert("one", "1");
    string_numbers.insert("two", "2");
    string_numbers.insert("thr", "3");
    string_numbers.insert("fou", "4");
    string_numbers.insert("fiv", "5");
    string_numbers.insert("six", "6");
    string_numbers.insert("sev", "7");
    string_numbers.insert("eig", "8");
    string_numbers.insert("nin", "9");

    let mut result = String::from("");
    let mut p: usize = 0;
    while p < replace.chars().count()-2 {
        if string_numbers.contains_key(&replace[p..p+3]) {
            result += &string_numbers[&replace[p..p+3]];
        } else {
            result += &replace.chars().nth(p).unwrap().to_string();
        }
        p+=1;
    }
    result += &replace[p..p+2];
    result
}

fn sum_calibrations(calibrations: Vec<String>) -> i32 {
    let mut total: i32 = 0;
    for c in calibrations {
        let cal_value = get_string_calibration(replace_string_numbers(c));
        total += cal_value;
    }
    total
}

fn main()  {
    let inputs: Vec<String> = load_data("input.txt".to_string());
    let total = sum_calibrations(inputs);
    println!("{}", total);
}


#[test]
fn test_string_parsing () {
    assert_eq!(get_string_calibration(replace_string_numbers("two1nine".to_string())), 29);
    assert_eq!(get_string_calibration(replace_string_numbers("eightwothree".to_string())), 83);
    assert_eq!(get_string_calibration(replace_string_numbers("abcone2threexyz".to_string())), 13);
    assert_eq!(get_string_calibration(replace_string_numbers("xtwone3four".to_string())), 24);
    assert_eq!(get_string_calibration(replace_string_numbers("4nineeightseven2".to_string())), 42);
    assert_eq!(get_string_calibration(replace_string_numbers("zoneight234".to_string())), 14);
    assert_eq!(get_string_calibration(replace_string_numbers("7pqrstsixteen".to_string())), 76);
    assert_eq!(get_string_calibration(replace_string_numbers("vfzvds826vtlrcg6rvseven".to_string())), 87);
    assert_eq!(get_string_calibration(replace_string_numbers("qtwonecvbgxqfiveoneeight4five".to_string())), 25);
    assert_eq!(get_string_calibration(replace_string_numbers("eighthrlbmtk4nssknqmxjvjnqsqlfivekzrphrtwo".to_string())), 82);
    assert_eq!(get_string_calibration(replace_string_numbers("fivefourfivefourfivefour".to_string())), 54);
    assert_eq!(get_string_calibration(replace_string_numbers("eightwothreeeightwo".to_string())), 82);
    assert_eq!(get_string_calibration(replace_string_numbers("fivetwone".to_string())), 51);
}

#[test]
fn test_individual_string_calibration() {
    assert_eq!(get_string_calibration("1abc2".to_string()), 12);
    assert_eq!(get_string_calibration("pqr3stu8vwx".to_string()), 38);
    assert_eq!(get_string_calibration("a1b2c3d4e5f".to_string()), 15);
    assert_eq!(get_string_calibration("treb7uchet".to_string()), 77);
}

#[test]
fn test_calibration_sum() {

    let calibrations: Vec<String> = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string()];
    assert_eq!(sum_calibrations(calibrations), 142);
}

#[test]
fn test_calibration_string_numbers_sum() {

    let calibrations: Vec<String> = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
    ];
    assert_eq!(sum_calibrations(calibrations), 281);
}