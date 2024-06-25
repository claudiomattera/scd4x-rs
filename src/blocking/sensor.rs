// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for SCD4x sensor interface

use core::marker::PhantomData;

use log::debug;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

use crate::constants::DEFAULT_ADDRESS;
use crate::sample::Sample;
use crate::Altitude;
use crate::Co2;
use crate::Error;
use crate::Idle;
use crate::Measuring;
use crate::Pressure;
use crate::State;
use crate::Temperature;

use super::commands;
use super::Command;

/// Interface to SCD4x sensor over I²C
pub struct Scd4x<I2c, Delay, State> {
    /// I²C device
    i2c: I2c,

    /// I²C address
    address: u8,

    /// Delay function
    delay: Delay,

    /// State for type-state pattern
    _state: PhantomData<State>,
}

impl<I2C, D> Scd4x<I2C, D, Idle>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Create a new sensor using an I²C interface and a delay function using
    /// the sensor's default address [`DEFAULT_ADDRESS`])
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self::new_with_address(i2c, DEFAULT_ADDRESS, delay)
    }

    /// Create a new sensor using an I²C interface and a delay function using
    /// a custom address
    pub fn new_with_address(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
            _state: PhantomData,
        }
    }

    /// Start periodic measurement
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn start_periodic_measurement(mut self) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'start_periodic_measurement'");

        commands::StartPeriodicMeasurement.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            (),
        )?;

        Ok(Scd4x {
            i2c: self.i2c,
            address: self.address,
            delay: self.delay,
            _state: PhantomData,
        })
    }

    /// Set temperature offset
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn set_temperature_offset(&mut self, temperature_offset: Temperature) -> Result<(), Error> {
        debug!("Send command 'set_temperature_offset'");

        commands::SetTemperatureOffset.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            temperature_offset,
        )
    }

    /// Get temperature offset
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn get_temperature_offset(&mut self) -> Result<Temperature, Error> {
        debug!("Send command 'get_temperature_offset'");

        commands::GetTemperatureOffset.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Set sensor altitude
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn set_sensor_altitude(&mut self, sensor_altitude: Altitude) -> Result<(), Error> {
        debug!("Send command 'set_sensor_altitude'");

        commands::SetSensorAltitude.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            sensor_altitude,
        )
    }

    /// Get sensor altitude
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn get_sensor_altitude(&mut self) -> Result<Altitude, Error> {
        debug!("Send command 'get_sensor_altitude'");

        commands::GetSensorAltitude.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Perform forced recalibration
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn perform_forced_recalibration(&mut self, co2: Co2) -> Result<Option<Co2>, Error> {
        debug!("Send command 'perform_forced_recalibration'");

        commands::PerformForcedRecalibration.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            co2,
        )
    }

    /// Set whether automatic self-calibration is enabled
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn set_automatic_self_calibration_enabled(&mut self, enabled: bool) -> Result<(), Error> {
        debug!("Send command 'set_automatic_self_calibration_enabled'");

        commands::SetAutomaticSelfCalibrationEnabled.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            enabled,
        )
    }

    /// Query whether automatic self-calibration is enabled
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn get_automatic_self_calibration_enabled(&mut self) -> Result<bool, Error> {
        debug!("Send command 'get_automatic_self_calibration_enabled'");

        commands::GetAutomaticSelfCalibrationEnabled.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            (),
        )
    }

    /// Start low-power periodic measurement
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn start_low_power_periodic_measurement(
        mut self,
    ) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'start_low_power_periodic_measurement'");

        commands::StartLowPowerPeriodicMeasurement.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            (),
        )?;

        Ok(Scd4x {
            i2c: self.i2c,
            address: self.address,
            delay: self.delay,
            _state: PhantomData,
        })
    }

    /// Persist settings to EEPROM
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn persist_settings(&mut self) -> Result<(), Error> {
        debug!("Send command 'persist_settings'");

        commands::PersistSettings.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Obtain the serial number
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn get_serial_number(&mut self) -> Result<u64, Error> {
        debug!("Send command 'get_serial_number'");

        commands::GetSerialNumber.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Perform self-test
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn perform_self_test(&mut self) -> Result<bool, Error> {
        debug!("Send command 'perform_self_test'");

        commands::PerformSelfTest.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Perform factory reset
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn perform_factory_reset(&mut self) -> Result<(), Error> {
        debug!("Send command 'perform_factory_reset'");

        commands::PerformFactoryReset.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Reinitialize the sensor
    ///
    /// Send a soft-reset signal, obtain the calibration coefficients, and set
    /// default sampling configuration.
    ///
    /// Note that the default sampling configuration disables measurement of
    /// temperature, pressure and humidity.
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn reinit(&mut self) -> Result<(), Error> {
        debug!("Send command 'reinit'");

        commands::Reinitialize.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Read a single-shot measurement
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn measure_single_shot(mut self) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'measure_single_shot'");

        commands::MeasureSingleShot.execute(self.address, &mut self.i2c, &mut self.delay, ())?;

        Ok(Scd4x {
            i2c: self.i2c,
            address: self.address,
            delay: self.delay,
            _state: PhantomData,
        })
    }

    /// Read a single-shot measurement of humidity and temperature
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn measure_single_shot_rht_only(mut self) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'measure_single_shot_rht_only'");

        commands::MeasureSingleShotRhtOnly.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            (),
        )?;

        Ok(Scd4x {
            i2c: self.i2c,
            address: self.address,
            delay: self.delay,
            _state: PhantomData,
        })
    }
}

impl<I2C, D> Scd4x<I2C, D, Measuring>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Create a new sensor in measuring state using an I²C interface and a
    /// delay function using the sensor's default address [`DEFAULT_ADDRESS`])
    pub fn new_in_measuring(i2c: I2C, delay: D) -> Self {
        Self::new_in_measuring_with_address(i2c, DEFAULT_ADDRESS, delay)
    }

    /// Create a new sensor in measuring state  using an I²C interface and a
    /// delay function
    pub fn new_in_measuring_with_address(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
            _state: PhantomData,
        }
    }

    /// Read a measurement from the sensor
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn read_measurement(&mut self) -> Result<Sample, Error> {
        debug!("Send command 'read_measurement'");

        commands::ReadMeasurement.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }

    /// Query whether data is available to be read
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn get_data_ready_status(&mut self) -> Result<bool, Error> {
        debug!("Send command 'get_data_ready_status'");

        commands::GetDataReadyStatus.execute(self.address, &mut self.i2c, &mut self.delay, ())
    }
}

impl<I2C, D, S> Scd4x<I2C, D, S>
where
    I2C: I2c,
    D: DelayNs,
    S: State,
{
    /// Release the I²C interface
    pub fn release(self) -> I2C {
        self.i2c
    }

    /// Stop periodic measurement
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn stop_periodic_measurement(mut self) -> Result<Scd4x<I2C, D, Idle>, Error> {
        debug!("Send command 'stop_periodic_measurement'");

        commands::StopPeriodicMeasurement.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            (),
        )?;

        Ok(Scd4x {
            i2c: self.i2c,
            address: self.address,
            delay: self.delay,
            _state: PhantomData,
        })
    }

    /// Set ambient pressure
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub fn set_ambient_pressure(&mut self, ambient_pressure: Pressure) -> Result<(), Error> {
        debug!("Send command 'set_ambient_pressure'");

        commands::SetAmbientPressure.execute(
            self.address,
            &mut self.i2c,
            &mut self.delay,
            ambient_pressure,
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic_in_result_fn)]

    use super::*;

    use embedded_hal_mock::eh1::delay::NoopDelay as DelayMock;
    use embedded_hal_mock::eh1::i2c::Mock as I2cMock;
    use embedded_hal_mock::eh1::i2c::Transaction as I2cTransaction;

    use crate::sample::altitude_from_meter;
    use crate::sample::co2_from_ppm;
    use crate::sample::humidity_from_number;
    use crate::sample::pressure_from_hectopascal;
    use crate::sample::temperature_from_celsius;
    use crate::Error;

    #[test]
    fn test_get_serial_number() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0x36, 0x82]),
            I2cTransaction::read(
                DEFAULT_ADDRESS,
                vec![0xf8, 0x96, 0x31, 0x9f, 0x07, 0xc2, 0x3b, 0xbe, 0x89],
            ),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        let serial_number = scd4x.get_serial_number()?;

        assert_eq!(serial_number, 273_325_796_834_238);

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_reinit() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x36, 0x46])];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        scd4x.reinit()?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_measure_single_shot() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x21, 0x9d])];
        let i2c = I2cMock::new(&expectations);

        let scd4x = Scd4x::new(i2c, DelayMock);

        let scd4x = scd4x.measure_single_shot()?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_measure_single_shot_rht_only() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x21, 0x96])];
        let i2c = I2cMock::new(&expectations);

        let scd4x = Scd4x::new(i2c, DelayMock);

        let scd4x = scd4x.measure_single_shot_rht_only()?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_perform_factory_reset() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x36, 0x32])];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        scd4x.perform_factory_reset()?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_perform_self_test() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0x36, 0x39]),
            I2cTransaction::read(DEFAULT_ADDRESS, vec![0x00, 0x00, 0x81]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        let result = scd4x.perform_self_test()?;
        assert!(result);

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_persist_settings() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x36, 0x15])];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        scd4x.persist_settings()?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_get_data_ready_status() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0xe4, 0xb8]),
            I2cTransaction::read(DEFAULT_ADDRESS, vec![0x80, 0x00, 0xa2]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new_in_measuring(i2c, DelayMock);

        let ready = scd4x.get_data_ready_status()?;
        assert!(!ready);

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_start_low_power_periodic_measurement() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x21, 0xac])];
        let i2c = I2cMock::new(&expectations);

        let scd4x = Scd4x::new(i2c, DelayMock);

        let scd4x = scd4x.start_low_power_periodic_measurement()?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_get_automatic_self_calibration_enabled() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0x23, 0x13]),
            I2cTransaction::read(DEFAULT_ADDRESS, vec![0x00, 0x00, 0x81]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        let enabled = scd4x.get_automatic_self_calibration_enabled()?;
        assert!(!enabled);

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_set_automatic_self_calibration_enabled() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(
            DEFAULT_ADDRESS,
            vec![0x24, 0x16, 0x00, 0x01, 0xb0],
        )];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        scd4x.set_automatic_self_calibration_enabled(true)?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_perform_forced_recalibration() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0x36, 0x2f, 0x01, 0xe0, 0xb4]),
            I2cTransaction::read(DEFAULT_ADDRESS, vec![0x7f, 0xce, 0x7b]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        let correction = scd4x.perform_forced_recalibration(co2_from_ppm(480.0))?;
        assert_eq!(correction, Some(co2_from_ppm(-50.0)));

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_perform_forced_recalibration_failure() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0x36, 0x2f, 0x01, 0xe0, 0xb4]),
            I2cTransaction::read(DEFAULT_ADDRESS, vec![0xff, 0xff, 0xac]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        let correction = scd4x.perform_forced_recalibration(co2_from_ppm(480.0))?;
        assert_eq!(correction, None);

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_set_ambient_pressure() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(
            DEFAULT_ADDRESS,
            vec![0xe0, 0x00, 0x03, 0xdb, 0x42],
        )];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        scd4x.set_ambient_pressure(pressure_from_hectopascal(987.0))?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_get_sensor_altitude() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0x23, 0x22]),
            I2cTransaction::read(DEFAULT_ADDRESS, vec![0x04, 0x4c, 0x42]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        let sensor_altitude = scd4x.get_sensor_altitude()?;
        assert_eq!(sensor_altitude, altitude_from_meter(1100.0));

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_set_sensor_altitude() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(
            DEFAULT_ADDRESS,
            vec![0x24, 0x27, 0x07, 0x9e, 0x09],
        )];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        scd4x.set_sensor_altitude(altitude_from_meter(1950.0))?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_get_temperature_offset() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0x23, 0x18]),
            I2cTransaction::read(DEFAULT_ADDRESS, vec![0x09, 0x12, 0x63]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        let temperature_offset = scd4x.get_temperature_offset()?;
        assert_eq!(temperature_offset, temperature_from_celsius(6.200_409));

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_set_temperature_offset() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(
            DEFAULT_ADDRESS,
            vec![0x24, 0x1d, 0x07, 0xe6, 0x48],
        )];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new(i2c, DelayMock);

        scd4x.set_temperature_offset(temperature_from_celsius(5.4))?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_stop_periodic_measurement() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x3f, 0x86])];
        let i2c = I2cMock::new(&expectations);

        let scd4x = Scd4x::new(i2c, DelayMock);

        let scd4x = scd4x.stop_periodic_measurement()?;

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_read_measurement() -> Result<(), Error> {
        let expectations = [
            I2cTransaction::write(DEFAULT_ADDRESS, vec![0xec, 0x05]),
            I2cTransaction::read(
                DEFAULT_ADDRESS,
                vec![0x01, 0xf4, 0x33, 0x66, 0x67, 0xa2, 0x5e, 0xb9, 0x3c],
            ),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut scd4x = Scd4x::new_in_measuring(i2c, DelayMock);

        let sample = scd4x.read_measurement()?;
        let expected = Sample {
            co2: co2_from_ppm(500.0),
            temperature: temperature_from_celsius(25.001_602),
            humidity: humidity_from_number(37.001_038),
        };

        assert_eq!(sample, expected);

        scd4x.release().done();
        Ok(())
    }

    #[test]
    fn test_start_periodic_measurement() -> Result<(), Error> {
        let expectations = [I2cTransaction::write(DEFAULT_ADDRESS, vec![0x21, 0xb1])];
        let i2c = I2cMock::new(&expectations);

        let scd4x = Scd4x::new(i2c, DelayMock);

        let scd4x = scd4x.start_periodic_measurement()?;

        scd4x.release().done();

        Ok(())
    }
}
