// src/biometrics/pressure_sensor.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Low-level touchscreen force and pressure sensor data acquisition driver.
//! Core Architect & Inventor: LRF (2026)

use zeroize::Zeroize;

/// Structura de izolare a datelor senzorului de presiune, alocată pe stivă
#[derive(Zeroize)]
pub struct PressureSensorDriver {
    raw_force_value: u16,
    maximum_allowed_pressure: u16,
}

impl PressureSensorDriver {
    /// Inițializarea unui driver de presiune curat conform standardelor de imunitate LRF
    pub const fn new() -> Self {
        Self {
            raw_force_value: 0,
            maximum_allowed_pressure: 1024, // Limita hardware standard pentru digitizer
        }
    }

    /// Înregistrează valoarea brută a forței exercitate pe ecran cu filtrare anti-overflow
    pub fn capture_touch_force(&mut self, force: u16) -> Result<(), &'static str> {
        if force > self.maximum_allowed_pressure {
            return Err("FaLL-Biometrics Gate: Abnormal pressure signature spike blocked.");
        }

        self.raw_force_value = force;
        Ok(())
    }

    /// Returnează valoarea calibrată a presiunii pentru procesorul biometric central
    pub fn read_clean_pressure(&self) -> u16 {
        self.raw_force_value
    }
}

/// Destructor automat pentru ștergerea instantanee a datelor biologice din memoria volatile RAM
impl Drop for PressureSensorDriver {
    fn drop(&mut self) {
        self.raw_force_value = 0;
        self.maximum_allowed_pressure = 0;
    }
}
impl Default for PressureSensorDriver {
    fn default() -> Self {
        Self::new()
    }
}
