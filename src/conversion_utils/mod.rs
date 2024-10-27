pub mod conversion {

    // Declare temp units
    #[derive(Debug, Copy, Clone)]
    pub enum TemperatureUnit {
        Celsius,
        Fahrenheit,
        Kelvin,
    }
    // Declare distance units
    #[derive(Debug, Copy, Clone)]
    pub enum DistanceUnit {
        Milimeter,
        Centimeter,
        Meter,
        Kilometer,
        Mile,
        Inch,
        Feet,
        Yard,
    }
    // Declare weight units
    #[derive(Debug, Copy, Clone)]
    pub enum WeightUnit {
        Gram,
        Kilogram,
        Pound,
        Ounce,
    }
    // Declare liquid units
    #[derive(Debug, Copy, Clone)]
    pub enum LiquidUnit {
        Liter,
        Mililiter,
        Gallon,
        Ounce,
        Cup,
        Tablespoon,
        Teaspoon,
    }

    // Convert temp units
    pub fn convert_temperature(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
        let kelvin: f64 = 273.15;
        let result = match (&from, &to) {
            (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => (value * 9.0 / 5.0) + 32.0,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (value - 32.0) * 5.0 / 9.0,
            (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => value + kelvin,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => {
                (value - 32.0) * 5.0 / 9.0 + kelvin
            }
            (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => value - kelvin,
            (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => {
                ((value - kelvin) * 9.0 / 5.0) + 32.0
            }
            _ => panic!("Unsupported unit conversion: {:?} to {:?}", from, to),
        };
        round_to(result, 2)
    }
    // Convert weight units
    pub fn convert_weight(value: f64, from: WeightUnit, to: WeightUnit) -> f64 {
        let result = match (&from, &to) {
            (WeightUnit::Kilogram, WeightUnit::Pound) => value * 2.2046,
            (WeightUnit::Kilogram, WeightUnit::Gram) => value * 1000.0,
            (WeightUnit::Kilogram, WeightUnit::Ounce) => value * 35.274,
            (WeightUnit::Pound, WeightUnit::Kilogram) => value / 2.2046,
            (WeightUnit::Pound, WeightUnit::Gram) => (value / 2.2046) * 1000.0,
            (WeightUnit::Pound, WeightUnit::Ounce) => value * 16.0,
            (WeightUnit::Ounce, WeightUnit::Gram) => (value / 35.274) * 1000.0,
            (WeightUnit::Ounce, WeightUnit::Kilogram) => value / 35.274,
            (WeightUnit::Ounce, WeightUnit::Pound) => value / 16.0,
            (WeightUnit::Gram, WeightUnit::Kilogram) => value / 1000.0,
            (WeightUnit::Gram, WeightUnit::Pound) => (value / 1000.0) * 2.2046,
            (WeightUnit::Gram, WeightUnit::Ounce) => (value / 1000.0) * 35.274,
            _ => panic!("Unsupported unit conversion: {:?} to {:?}", from, to),
        };
        round_to(result, 2)
    }
    // Convert distance units
    pub fn convert_distance(value: f64, from: DistanceUnit, to: DistanceUnit) -> f64 {
        let result = match (&from, &to) {
            (DistanceUnit::Mile, DistanceUnit::Kilometer) => value * 1.60934,
            (DistanceUnit::Mile, DistanceUnit::Meter) => value * 1609.34,
            (DistanceUnit::Mile, DistanceUnit::Centimeter) => (value / 10000.0) * 1.60934,
            (DistanceUnit::Mile, DistanceUnit::Milimeter) => (value / 100000.0) * 1.60934,
            (DistanceUnit::Mile, DistanceUnit::Feet) => value * 5280.0,
            (DistanceUnit::Mile, DistanceUnit::Yard) => value * 1760.0,
            (DistanceUnit::Mile, DistanceUnit::Inch) => value * 63360.0,
            (DistanceUnit::Kilometer, DistanceUnit::Mile) => value / 1.60934,
            (DistanceUnit::Kilometer, DistanceUnit::Feet) => (value / 1.60934) * 5280.0,
            (DistanceUnit::Kilometer, DistanceUnit::Yard) => (value / 1.60934) * 1760.0,
            (DistanceUnit::Kilometer, DistanceUnit::Inch) => (value / 1.60934) * 63360.0,
            (DistanceUnit::Kilometer, DistanceUnit::Meter) => value * 1000.0,
            (DistanceUnit::Kilometer, DistanceUnit::Centimeter) => value * 10000.0,
            (DistanceUnit::Kilometer, DistanceUnit::Milimeter) => value * 1e+6,
            (DistanceUnit::Meter, DistanceUnit::Mile) => (value / 1000.0) / 1.60934,
            (DistanceUnit::Meter, DistanceUnit::Feet) => ((value / 1000.0) / 1.60934) * 5280.0,
            (DistanceUnit::Meter, DistanceUnit::Yard) => value * 1.094,
            (DistanceUnit::Meter, DistanceUnit::Inch) => value * 39.37,
            (DistanceUnit::Meter, DistanceUnit::Kilometer) => value / 1000.0,
            (DistanceUnit::Meter, DistanceUnit::Centimeter) => value * 100.0,
            (DistanceUnit::Meter, DistanceUnit::Milimeter) => value * 1000.0,
            (DistanceUnit::Centimeter, DistanceUnit::Kilometer) => value / 1e-5,
            (DistanceUnit::Centimeter, DistanceUnit::Meter) => value / 100.0,
            (DistanceUnit::Centimeter, DistanceUnit::Milimeter) => value * 10.0,
            (DistanceUnit::Centimeter, DistanceUnit::Mile) => value / 160900.0,
            (DistanceUnit::Centimeter, DistanceUnit::Feet) => value * 30.48,
            (DistanceUnit::Centimeter, DistanceUnit::Yard) => value * 91.44,
            (DistanceUnit::Centimeter, DistanceUnit::Inch) => value / 2.54,
            (DistanceUnit::Milimeter, DistanceUnit::Centimeter) => value / 10.0,
            (DistanceUnit::Milimeter, DistanceUnit::Meter) => value / 1000.0,
            (DistanceUnit::Milimeter, DistanceUnit::Kilometer) => value / 1e+6,
            (DistanceUnit::Milimeter, DistanceUnit::Mile) => value / 1.609e+6,
            (DistanceUnit::Milimeter, DistanceUnit::Feet) => value / 304.8,
            (DistanceUnit::Milimeter, DistanceUnit::Yard) => value / 914.4,
            (DistanceUnit::Milimeter, DistanceUnit::Inch) => value / 25.4,
            (DistanceUnit::Feet, DistanceUnit::Milimeter) => value * 304.8,
            (DistanceUnit::Feet, DistanceUnit::Centimeter) => value * 30.48,
            (DistanceUnit::Feet, DistanceUnit::Meter) => value * 3.048,
            (DistanceUnit::Feet, DistanceUnit::Kilometer) => value * 0.3048,
            (DistanceUnit::Feet, DistanceUnit::Mile) => value / 5280.0,
            (DistanceUnit::Feet, DistanceUnit::Yard) => value / 3.0,
            (DistanceUnit::Feet, DistanceUnit::Inch) => value * 12.0,
            (DistanceUnit::Yard, DistanceUnit::Milimeter) => value * 914.4,
            (DistanceUnit::Yard, DistanceUnit::Centimeter) => value * 91.44,
            (DistanceUnit::Yard, DistanceUnit::Meter) => value * 9.144,
            (DistanceUnit::Yard, DistanceUnit::Kilometer) => value * 0.9144,
            (DistanceUnit::Yard, DistanceUnit::Mile) => value / 1760.0,
            (DistanceUnit::Yard, DistanceUnit::Feet) => value * 3.0,
            (DistanceUnit::Yard, DistanceUnit::Inch) => value * 36.0,
            (DistanceUnit::Inch, DistanceUnit::Milimeter) => value * 25.4,
            (DistanceUnit::Inch, DistanceUnit::Centimeter) => value * 2.54,
            (DistanceUnit::Inch, DistanceUnit::Meter) => value * 0.254,
            (DistanceUnit::Inch, DistanceUnit::Kilometer) => value * 0.0254,
            (DistanceUnit::Inch, DistanceUnit::Mile) => value / 63360.0,
            (DistanceUnit::Inch, DistanceUnit::Yard) => value / 36.0,
            (DistanceUnit::Inch, DistanceUnit::Feet) => value / 12.0,
            _ => panic!("Unsupported conversion type: {:?} to {:?}", from, to),
        };
        round_to(result, 2)
    }
    // Convert liquid units
    pub fn convert_liquid(value: f64, from: LiquidUnit, to: LiquidUnit) -> f64 {
        let result = match (&from, &to) {
            (LiquidUnit::Liter, LiquidUnit::Mililiter) => value * 1000.0,
            (LiquidUnit::Liter, LiquidUnit::Gallon) => value / 3.7850,
            (LiquidUnit::Liter, LiquidUnit::Ounce) => value * 33.8140,
            (LiquidUnit::Liter, LiquidUnit::Cup) => value * 4.227,
            (LiquidUnit::Gallon, LiquidUnit::Liter) => value * 3.78541,
            (LiquidUnit::Gallon, LiquidUnit::Mililiter) => value * 3785.41,
            (LiquidUnit::Gallon, LiquidUnit::Ounce) => value * 128.0,
            (LiquidUnit::Gallon, LiquidUnit::Cup) => value * 16.0,
            (LiquidUnit::Ounce, LiquidUnit::Gallon) => value / 128.0,
            (LiquidUnit::Ounce, LiquidUnit::Liter) => value / 33.814,
            (LiquidUnit::Ounce, LiquidUnit::Mililiter) => value * 29.574,
            (LiquidUnit::Ounce, LiquidUnit::Cup) => value / 8.0,
            (LiquidUnit::Ounce, LiquidUnit::Tablespoon) => value * 2.0,
            (LiquidUnit::Ounce, LiquidUnit::Teaspoon) => value * 6.0,
            (LiquidUnit::Mililiter, LiquidUnit::Liter) => value / 1000.0,
            (LiquidUnit::Mililiter, LiquidUnit::Gallon) => value / 3785.41,
            (LiquidUnit::Mililiter, LiquidUnit::Ounce) => value / 29.574,
            (LiquidUnit::Mililiter, LiquidUnit::Cup) => value / 236.6,
            (LiquidUnit::Mililiter, LiquidUnit::Tablespoon) => value / 14.787,
            (LiquidUnit::Mililiter, LiquidUnit::Teaspoon) => value / 4.929,
            (LiquidUnit::Cup, LiquidUnit::Liter) => value / 4.227,
            (LiquidUnit::Cup, LiquidUnit::Mililiter) => value * 236.588,
            (LiquidUnit::Cup, LiquidUnit::Gallon) => value / 16.0,
            (LiquidUnit::Cup, LiquidUnit::Ounce) => value * 8.0,
            (LiquidUnit::Cup, LiquidUnit::Tablespoon) => value * 16.0,
            (LiquidUnit::Cup, LiquidUnit::Teaspoon) => value * 48.0,
            (LiquidUnit::Tablespoon, LiquidUnit::Cup) => value / 16.0,
            (LiquidUnit::Tablespoon, LiquidUnit::Ounce) => value / 2.0,
            (LiquidUnit::Tablespoon, LiquidUnit::Teaspoon) => value * 3.0,
            (LiquidUnit::Tablespoon, LiquidUnit::Mililiter) => value * 14.787,
            (LiquidUnit::Teaspoon, LiquidUnit::Cup) => value / 48.0,
            (LiquidUnit::Teaspoon, LiquidUnit::Ounce) => value / 6.0,
            (LiquidUnit::Teaspoon, LiquidUnit::Tablespoon) => value / 3.0,
            (LiquidUnit::Teaspoon, LiquidUnit::Mililiter) => value * 4.929,
            _ => panic!("Unsupported conversion type: {:?} to {:?}", from, to),
        };
        round_to(result, 2)
    }

    // Round to nth decimal place
    pub fn round_to(value: f64, decimals: u32) -> f64 {
        let factor = 10f64.powi(decimals as i32);
        (value * factor).round() / factor
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::conversion::round_to;

    #[test]
    fn test_convert_temperature() {
        assert_eq!(
            conversion::convert_temperature(
                0.0,
                conversion::TemperatureUnit::Celsius,
                conversion::TemperatureUnit::Fahrenheit
            ),
            32.0
        )
    }
    #[test]
    fn test_convert_weight() {
        assert_eq!(
            conversion::convert_weight(
                1.0,
                conversion::WeightUnit::Kilogram,
                conversion::WeightUnit::Pound
            ),
            round_to(2.2046, 2)
        )
    }
}
