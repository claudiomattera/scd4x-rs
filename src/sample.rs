// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for SCD4x sensor samples

#[cfg(feature = "uom")]
use uom::si::f32::Length as UomAltitude;
#[cfg(feature = "uom")]
use uom::si::f32::Pressure as UomPressure;
#[cfg(feature = "uom")]
use uom::si::f32::Ratio as UomHumidity;
#[cfg(feature = "uom")]
use uom::si::f32::Ratio as UomCo2;
#[cfg(feature = "uom")]
use uom::si::f32::ThermodynamicTemperature as UomTemperature;
#[cfg(feature = "uom")]
use uom::si::length::meter;
#[cfg(feature = "uom")]
use uom::si::pressure::hectopascal;
#[cfg(feature = "uom")]
use uom::si::ratio::part_per_million;
#[cfg(feature = "uom")]
use uom::si::ratio::percent;
#[cfg(feature = "uom")]
use uom::si::thermodynamic_temperature::degree_celsius;

#[cfg(feature = "uom")]
/// Type for CO₂ concentration values
pub type Co2 = UomCo2;

#[cfg(feature = "uom")]
/// Type for temperature values
pub type Temperature = UomTemperature;

#[cfg(feature = "uom")]
/// Type for humidity values
pub type Humidity = UomHumidity;

#[cfg(feature = "uom")]
/// Type for pressure values
pub type Pressure = UomPressure;

#[cfg(feature = "uom")]
/// Type for altitude values
pub type Altitude = UomAltitude;

#[cfg(feature = "uom")]
/// Convert a raw value in PPM to a CO₂ concentration
pub(crate) fn co2_from_ppm(raw: f32) -> Co2 {
    Co2::new::<part_per_million>(raw)
}

#[cfg(feature = "uom")]
/// Convert a CO₂ concentration to a raw value in PPM
pub(crate) fn ppm_from_co2(co2: Co2) -> f32 {
    co2.get::<part_per_million>()
}

#[cfg(feature = "uom")]
/// Convert a raw value in Celsius to a temperature
pub(crate) fn temperature_from_celsius(raw: f32) -> Temperature {
    Temperature::new::<degree_celsius>(raw)
}

#[cfg(feature = "uom")]
/// Convert a temperature to a raw value in Celsius
pub(crate) fn celsius_from_temperature(temperature: Temperature) -> f32 {
    temperature.get::<degree_celsius>()
}

#[cfg(feature = "uom")]
/// Convert a raw value to a humidity
pub(crate) fn humidity_from_number(raw: f32) -> Humidity {
    Humidity::new::<percent>(raw)
}

#[cfg(feature = "uom")]
/// Convert a pressure to a raw value in hectoPascal
pub(crate) fn hectopascal_from_pressure(pressure: Pressure) -> f32 {
    pressure.get::<hectopascal>()
}

#[cfg(all(feature = "uom", feature = "blocking"))]
#[cfg(test)]
/// Convert a raw value in hectoPascal to a pressure
pub(crate) fn pressure_from_hectopascal(raw: f32) -> Pressure {
    Pressure::new::<hectopascal>(raw)
}

#[cfg(feature = "uom")]
/// Convert a raw value in meter to an altitude
pub(crate) fn altitude_from_meter(raw: f32) -> Altitude {
    Altitude::new::<meter>(raw)
}

#[cfg(feature = "uom")]
/// Convert an altitude to a raw value in meter
pub(crate) fn meter_from_altitude(altitude: Altitude) -> f32 {
    altitude.get::<meter>()
}

#[cfg(not(feature = "uom"))]
/// Type for CO₂ values
pub type Co2 = f32;

#[cfg(not(feature = "uom"))]
/// Type for temperature values
pub type Temperature = f32;

#[cfg(not(feature = "uom"))]
/// Type for temperature values
pub type Humidity = f32;

#[cfg(not(feature = "uom"))]
/// Type for pressure values
pub type Pressure = f32;

#[cfg(not(feature = "uom"))]
/// Type for altitude values
pub type Altitude = f32;

#[cfg(not(feature = "uom"))]
/// Convert a raw value in Celsius to a temperature
pub(crate) fn co2_from_ppm(raw: f32) -> Co2 {
    raw
}

#[cfg(not(feature = "uom"))]
/// Convert a CO₂ concentration to a raw value in PPM
pub(crate) fn ppm_from_co2(co2: Co2) -> f32 {
    co2
}

#[cfg(not(feature = "uom"))]
/// Convert a raw value in Celsius to a temperature
pub(crate) fn temperature_from_celsius(raw: f32) -> Temperature {
    raw
}

#[cfg(not(feature = "uom"))]
/// Convert a temperature to a raw value in Celsius
pub(crate) fn celsius_from_temperature(temperature: Temperature) -> f32 {
    temperature
}

#[cfg(not(feature = "uom"))]
/// Convert a raw value to a humidity
pub(crate) fn humidity_from_number(raw: f32) -> Humidity {
    raw
}

#[cfg(not(feature = "uom"))]
/// Convert a pressure to a raw value in hectoPascal
pub(crate) fn hectopascal_from_pressure(pressure: Pressure) -> f32 {
    pressure
}

#[cfg(all(not(feature = "uom"), feature = "blocking"))]
#[cfg(test)]
/// Convert a raw value in hectoPascal to a pressure
pub(crate) fn pressure_from_hectopascal(raw: f32) -> Pressure {
    raw
}

#[cfg(not(feature = "uom"))]
/// Convert a raw value in meter to an altitude
pub(crate) fn altitude_from_meter(raw: f32) -> Altitude {
    raw
}

#[cfg(not(feature = "uom"))]
/// Convert an altitude to a raw value in meter
pub(crate) fn meter_from_altitude(altitude: Altitude) -> f32 {
    altitude
}

/// A full sample: CO₂ concentration, temperature and humidity
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Sample {
    /// CO₂ concentration
    pub co2: Co2,

    /// Temperature
    pub temperature: Temperature,

    /// Humidity
    pub humidity: Humidity,
}
