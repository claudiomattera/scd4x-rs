// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Example of continuous operation of SCD4x sensor through a FT232H board

use core::time::Duration;

use std::error::Error;

use env_logger::init as init_logger;

use log::info;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;
use ftdi_embedded_hal::Delay;
use ftdi_embedded_hal::FtHal;

use uom::si::ratio::part_per_million;
use uom::si::ratio::percent;
use uom::si::thermodynamic_temperature::degree_celsius;

use scd4x_rs::Error as Scd4xError;
use scd4x_rs::Scd4x;

/// Number of measurements to read
const MEASUREMENTS_COUNT: usize = 10;

/// Time between each measurement
const SLEEP_INTERVAL: Duration = Duration::from_secs(5);

/// Main entry point
fn main() -> Result<(), Box<dyn Error>> {
    init_logger();

    info!("Create FTDI device");
    let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
        .interface(ftdi::Interface::A)
        .open()?;

    info!("Initialize I²C bus");
    let hal = FtHal::init_freq(device, 400_000)?;
    let i2c = hal.i2c()?;

    let _i2c = handle_scd4x(i2c)?;

    Ok(())
}

/// Execute operations on sensor SCD4x
fn handle_scd4x<I2C>(i2c: I2C) -> Result<I2C, Box<dyn Error>>
where
    I2C: I2c,
{
    let mut delay = Delay::new();

    let sensor = Scd4x::new(i2c, delay);

    let mut sensor = sensor.stop_periodic_measurement()?;
    sensor.reinit()?;

    let serial_number = sensor.get_serial_number()?;
    info!("Serial number: 0x{serial_number:x}");

    let mut sensor = sensor.start_periodic_measurement()?;

    info!("Taking {MEASUREMENTS_COUNT} measurements");

    let samples: Result<Vec<_>, Scd4xError> = (0..MEASUREMENTS_COUNT)
        .map(|_| {
            info!("Waiting {SLEEP_INTERVAL:?} before next measurement");
            #[allow(clippy::cast_possible_truncation)]
            delay.delay_ms(SLEEP_INTERVAL.as_millis() as u32);

            let sample = sensor.read_measurement()?;

            info!(
                "Sample: ┳ CO₂: {} ppm",
                sample.co2.get::<part_per_million>()
            );
            info!(
                "        ┣ Temperature: {} C",
                sample.temperature.get::<degree_celsius>()
            );
            info!("        ┗ Humidity: {} %", sample.humidity.get::<percent>());

            Ok(sample)
        })
        .collect();

    let sensor = sensor.stop_periodic_measurement()?;

    let _samples = samples?;

    let i2c = sensor.release();

    Ok(i2c)
}
