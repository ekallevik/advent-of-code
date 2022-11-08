use fancy_regex::Regex;
use serde_json::Value;

use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> String {
    let input = get_input_string(filename);

    let re = Regex::new(r#"(-*\d+)"#).unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(&*input) {
        let value = &cap.unwrap().get(1).unwrap().as_str();
        sum += value.parse::<i32>().unwrap();
    }

    sum.to_string()
}

pub fn solve_2(filename: &str) -> String {

    let input = get_input_string(filename);
    let json: Value = serde_json::from_str(&*input).unwrap();

    sum_json(json).to_string()
}

fn sum_json(json: Value) -> i64 {
    match json {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(arr) => arr.into_iter().map(sum_json).sum(),
        Value::Object(obj) => {

            let mut sum = 0;

            for element in obj.values() {
                if is_red(element) {
                    return 0
                }
                sum += sum_json(element.clone());
            };

            sum
        }
    }
}


fn is_red(json: &Value) -> bool {
    match json {
        Value::String(prop) => prop == "red",
        _ => false
    }
}
