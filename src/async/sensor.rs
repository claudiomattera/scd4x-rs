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

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    constants::DEFAULT_ADDRESS, sample::Sample, Altitude, Co2, Error, Idle, Measuring, Pressure,
    State, Temperature,
};

use super::{commands, Command};

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
    pub async fn start_periodic_measurement(mut self) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'start_periodic_measurement'");

        commands::StartPeriodicMeasurement
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await?;

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
    pub async fn set_temperature_offset(
        &mut self,
        temperature_offset: Temperature,
    ) -> Result<(), Error> {
        debug!("Send command 'set_temperature_offset'");

        commands::SetTemperatureOffset
            .execute(
                self.address,
                &mut self.i2c,
                &mut self.delay,
                temperature_offset,
            )
            .await
    }

    /// Get temperature offset
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn get_temperature_offset(&mut self) -> Result<Temperature, Error> {
        debug!("Send command 'get_temperature_offset'");

        commands::GetTemperatureOffset
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Set sensor altitude
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn set_sensor_altitude(&mut self, sensor_altitude: Altitude) -> Result<(), Error> {
        debug!("Send command 'set_sensor_altitude'");

        commands::SetSensorAltitude
            .execute(
                self.address,
                &mut self.i2c,
                &mut self.delay,
                sensor_altitude,
            )
            .await
    }

    /// Get sensor altitude
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn get_sensor_altitude(&mut self) -> Result<Altitude, Error> {
        debug!("Send command 'get_sensor_altitude'");

        commands::GetSensorAltitude
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Perform forced recalibration
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn perform_forced_recalibration(&mut self, co2: Co2) -> Result<Option<Co2>, Error> {
        debug!("Send command 'perform_forced_recalibration'");

        commands::PerformForcedRecalibration
            .execute(self.address, &mut self.i2c, &mut self.delay, co2)
            .await
    }

    /// Set whether automatic self-calibration is enabled
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn set_automatic_self_calibration_enabled(
        &mut self,
        enabled: bool,
    ) -> Result<(), Error> {
        debug!("Send command 'set_automatic_self_calibration_enabled'");

        commands::SetAutomaticSelfCalibrationEnabled
            .execute(self.address, &mut self.i2c, &mut self.delay, enabled)
            .await
    }

    /// Query whether automatic self-calibration is enabled
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn get_automatic_self_calibration_enabled(&mut self) -> Result<bool, Error> {
        debug!("Send command 'get_automatic_self_calibration_enabled'");

        commands::GetAutomaticSelfCalibrationEnabled
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Start low-power periodic measurement
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn start_low_power_periodic_measurement(
        mut self,
    ) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'start_low_power_periodic_measurement'");

        commands::StartLowPowerPeriodicMeasurement
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await?;

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
    pub async fn persist_settings(&mut self) -> Result<(), Error> {
        debug!("Send command 'persist_settings'");

        commands::PersistSettings
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Obtain the serial number
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn get_serial_number(&mut self) -> Result<u64, Error> {
        debug!("Send command 'get_serial_number'");

        commands::GetSerialNumber
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Perform self-test
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn perform_self_test(&mut self) -> Result<bool, Error> {
        debug!("Send command 'perform_self_test'");

        commands::PerformSelfTest
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Perform factory reset
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn perform_factory_reset(&mut self) -> Result<(), Error> {
        debug!("Send command 'perform_factory_reset'");

        commands::PerformFactoryReset
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
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
    pub async fn reinit(&mut self) -> Result<(), Error> {
        debug!("Send command 'reinit'");

        commands::Reinitialize
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Read a single-shot measurement
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn measure_single_shot(mut self) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'measure_single_shot'");

        commands::MeasureSingleShot
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await?;

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
    pub async fn measure_single_shot_rht_only(mut self) -> Result<Scd4x<I2C, D, Measuring>, Error> {
        debug!("Send command 'measure_single_shot_rht_only'");

        commands::MeasureSingleShotRhtOnly
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await?;

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
    pub async fn read_measurement(&mut self) -> Result<Sample, Error> {
        debug!("Send command 'read_measurement'");

        commands::ReadMeasurement
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
    }

    /// Query whether data is available to be read
    ///
    /// # Errors
    ///
    /// Return an error if it cannot communicate with the sensor.
    pub async fn get_data_ready_status(&mut self) -> Result<bool, Error> {
        debug!("Send command 'get_data_ready_status'");

        commands::GetDataReadyStatus
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await
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
    pub async fn stop_periodic_measurement(mut self) -> Result<Scd4x<I2C, D, Idle>, Error> {
        debug!("Send command 'stop_periodic_measurement'");

        commands::StopPeriodicMeasurement
            .execute(self.address, &mut self.i2c, &mut self.delay, ())
            .await?;

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
    pub async fn set_ambient_pressure(&mut self, ambient_pressure: Pressure) -> Result<(), Error> {
        debug!("Send command 'set_ambient_pressure'");

        commands::SetAmbientPressure
            .execute(
                self.address,
                &mut self.i2c,
                &mut self.delay,
                ambient_pressure,
            )
            .await
    }
}
