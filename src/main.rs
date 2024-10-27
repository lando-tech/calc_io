mod conversion_utils;
use conversion_utils::conversion::{self};
use conversion_utils::conversion::{DistanceUnit, LiquidUnit, TemperatureUnit, WeightUnit};
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let amount = args[1].parse::<f64>().expect("Invalid amount");
    let from = &args[2];
    let to = &args[3];

    let result = match from.as_str() {
        "C" | "F" | "K" => temp(amount, from, to),
        "l" | "ml" | "gal" | "loz" | "cup" | "tbsp" | "tsp" => liquid(amount, from, to),
        "kg" | "g" | "lb" | "oz" => weight(amount, from, to),
        "km" | "m" | "cm" | "mm" | "M" | "ft" | "yd" | "inch" => distance(amount, from, to),
        _ => {
            eprintln!("Unsupported unit type");
            return;
        }
    };

    println!("Converted amount: {} {}", result, to)
}

fn temp(arg1: f64, from: &str, to: &str) -> f64 {
    let temp: HashMap<&str, TemperatureUnit> = [
        ("C", TemperatureUnit::Celsius),
        ("F", TemperatureUnit::Fahrenheit),
        ("K", TemperatureUnit::Kelvin),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
    let from_unit = *temp.get(from).expect("Incorrect unit type");
    let to_unit = *temp.get(to).expect("Incorrect unit type");

    conversion::convert_temperature(arg1, from_unit, to_unit)
}

fn liquid(arg1: f64, from: &str, to: &str) -> f64 {
    let liquid: HashMap<&str, LiquidUnit> = [
        ("l", LiquidUnit::Liter),
        ("ml", LiquidUnit::Mililiter),
        ("gal", LiquidUnit::Gallon),
        ("loz", LiquidUnit::Ounce),
        ("cup", LiquidUnit::Cup),
        ("tbsp", LiquidUnit::Tablespoon),
        ("tsp", LiquidUnit::Teaspoon),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
    let from_unit = *liquid.get(from).expect("Incorrect unit type");
    let to_unit = *liquid.get(to).expect("Incorrect unit type");

    conversion::convert_liquid(arg1, from_unit, to_unit)
}

fn weight(arg1: f64, from: &str, to: &str) -> f64 {
    let weight: HashMap<&str, WeightUnit> = [
        ("kg", WeightUnit::Kilogram),
        ("g", WeightUnit::Gram),
        ("lb", WeightUnit::Pound),
        ("oz", WeightUnit::Ounce),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let from_unit = *weight.get(from).expect("Incorrect unit type");
    let to_unit = *weight.get(to).expect("Incorrect unit type");

    conversion::convert_weight(arg1, from_unit, to_unit)
}

fn distance(arg1: f64, from: &str, to: &str) -> f64 {
    let distance: HashMap<&str, DistanceUnit> = [
        ("km", DistanceUnit::Kilometer),
        ("m", DistanceUnit::Meter),
        ("cm", DistanceUnit::Centimeter),
        ("mm", DistanceUnit::Milimeter),
        ("M", DistanceUnit::Mile),
        ("ft", DistanceUnit::Feet),
        ("yd", DistanceUnit::Yard),
        ("inch", DistanceUnit::Inch),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let from_unit = *distance.get(from).expect("Incorrect unit type");
    let to_unit = *distance.get(to).expect("Incorrect unit type");

    conversion::convert_distance(arg1, from_unit, to_unit)
}
