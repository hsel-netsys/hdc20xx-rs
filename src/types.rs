use crate::BASE_ADDR;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C communication error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Measurement result
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// Temperature (°C)
    pub temperature: f32,
    /// Relative Humidity (%RH)
    ///
    /// Optionally read depending on the measurement configuration
    pub humidity: Option<f32>,
    /// Last status
    pub status: Status,
}

/// Status
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    /// Whether data is ready
    pub data_ready: bool,
    /// Whether the temperature high threshold was exceeded
    pub high_temp_threshold_exceeded: bool,
    /// Whether the temperature low threshold was exceeded
    pub low_temp_threshold_exceeded: bool,
    /// Whether the humidity high threshold was exceeded
    pub high_humidity_threshold_exceeded: bool,
    /// Whether the humidity low threshold was exceeded
    pub low_humidity_threshold_exceeded: bool,
}

/// Measurement mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementMode {
    /// Temperature and humidity (default)
    TemperatureAndHumidity,
    /// Temperature only
    TemperatureOnly,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        MeasurementMode::TemperatureAndHumidity
    }
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for the SDO pin
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => BASE_ADDR,
            SlaveAddr::Alternative(false) => BASE_ADDR,
            SlaveAddr::Alternative(true) => BASE_ADDR | 1,
        }
    }
}

/// Possible automatic measurement mode choices.
#[repr(u8)]
#[derive(Copy, Debug, PartialEq, Eq, Clone)]
pub enum AutomaticMeasurementMode {
    /// Disable automatic measurement
    Disabled    = 0b00000000,
    /// Measure once every two minutes
    TwoMinutes  = 0b00010100,
    /// Measure once every minute
    OneMinute   = 0b00100100,
    /// Measure once every ten seconds
    TenSeconds  = 0b00110100,
    /// Measure once every five seconds
    FiveSeconds = 0b01000100,
    /// Measure once per second
    OneHertz    = 0b01010100,
    /// Measure twice per second
    TwoHertz    = 0b01100100,
    /// Measure five times per second
    FiveHertz   = 0b01110100,
}

#[cfg(test)]
mod tests {
    use super::BASE_ADDR as ADDR;
    use super::{MeasurementMode, SlaveAddr};

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(ADDR, addr.addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(ADDR, SlaveAddr::Alternative(false).addr());
        assert_eq!(ADDR | 1, SlaveAddr::Alternative(true).addr());
    }

    #[test]
    fn can_get_default_measurement_mode() {
        assert_eq!(
            MeasurementMode::TemperatureAndHumidity,
            MeasurementMode::default()
        );
    }
}
