// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for conversions between values and words

use crate::sample::{
    altitude_from_meter, co2_from_ppm, hectopascal_from_pressure, humidity_from_number,
    meter_from_altitude, ppm_from_co2, temperature_from_celsius, Sample,
};
use crate::{sample::celsius_from_temperature, Altitude, Co2, Humidity, Pressure, Temperature};

/// Convert three words to a serial number
pub(crate) fn words_to_serial_number(word0: u16, word1: u16, word2: u16) -> u64 {
    (u64::from(word0) << 32) + (u64::from(word1) << 16) + u64::from(word2)
}

/// Convert three words to a sample
pub(crate) fn words_to_sample(word0: u16, word1: u16, word2: u16) -> Sample {
    let co2 = f32::from(word0);
    let temperature = word_to_temperature(word1);
    let humidity = word_to_humidity(word2);

    Sample {
        co2: co2_from_ppm(co2),
        temperature,
        humidity,
    }
}

/// Convert a word to a humidity value
pub(crate) fn word_to_humidity(word: u16) -> Humidity {
    let humidity = (100_f32 * f32::from(word)) / 65536_f32;
    humidity_from_number(humidity)
}

/// Convert a word to an altitude value
pub(crate) fn word_to_altitude(word: u16) -> Altitude {
    let meter = f32::from(word);
    altitude_from_meter(meter)
}

/// Convert an altitude value to a word
pub(crate) fn altitude_to_word(altitude: Altitude) -> u16 {
    let meter = meter_from_altitude(altitude);

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let word = meter as u16;

    word
}

/// Convert a word to a temperature offset value
pub(crate) fn word_to_temperature_offset(word: u16) -> Temperature {
    let celsius = (175_f32 * f32::from(word)) / 65536_f32;
    temperature_from_celsius(celsius)
}

/// Convert a temperature offset value to a word
pub(crate) fn temperature_offset_to_word(temperature_offset: Temperature) -> u16 {
    let celsius = celsius_from_temperature(temperature_offset);

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let word = ((65536_f32 * celsius) / 175_f32) as u16;
    word
}

/// Convert a word to a temperature value
pub(crate) fn word_to_temperature(word: u16) -> Temperature {
    let celsius = -45_f32 + (175_f32 * f32::from(word)) / 65536_f32;
    temperature_from_celsius(celsius)
}

/// Convert an ambient pressure value to a word
pub(crate) fn ambient_pressure_to_word(ambient_pressure: Pressure) -> u16 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let word = hectopascal_from_pressure(ambient_pressure) as u16;

    word
}

/// Convert a signed word to a CO₂ value
pub(crate) fn signed_word_to_co2(word: i16) -> Co2 {
    let ppm = f32::from(word);
    co2_from_ppm(ppm)
}

/// Convert a CO₂ value to a word
pub(crate) fn co2_to_word(co2: Co2) -> u16 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let word = ppm_from_co2(co2) as u16;

    word
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic_in_result_fn)]

    use super::*;

    #[test]
    fn test_words_to_serial_number() {
        let (word0, word1, word2) = (0xf896, 0x9f07, 0x3bbe);
        let actual = words_to_serial_number(word0, word1, word2);
        let expected = 273_325_796_834_238; // 0xF8_96_9F_07_3B_BE

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_words_to_reading() {
        let (word0, word1, word2) = (0x01f4, 0x6667, 0x5eb9);
        let actual = words_to_sample(word0, word1, word2);
        let expected = Sample {
            co2: co2_from_ppm(500.0),
            temperature: temperature_from_celsius(25.001_602),
            humidity: humidity_from_number(37.001_038),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_word_to_temperature_offset() {
        let word = 0x0912;
        let actual = word_to_temperature_offset(word);
        let expected = temperature_from_celsius(6.200_409);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_temperature_offset_to_word() {
        let temperature_offset = 5.4;
        let actual = temperature_offset_to_word(temperature_from_celsius(temperature_offset));
        let expected = 0x07e6;

        assert_eq!(actual, expected);
    }
}
