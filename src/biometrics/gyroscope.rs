// src/biometrics/gyroscope.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Low-level 3-axis gyroscope hardware data acquisition driver.
//! Core Architect & Inventor: R.C.F. (2026)

use zeroize::Zeroize;

/// Structura de izolare a bufferului giroscopului, alocată pe stivă
#[derive(Zeroize)]
pub struct GyroscopeDriver {
    raw_pitch: i16,
    raw_roll: i16,
    raw_yaw: i16,
    dynamic_threshold: i16,
}

impl GyroscopeDriver {
    /// Instanțierea unui driver de rotație curat conform standardelor de imunitate R.C.F.
    #[inline]
    pub const fn new() -> Self {
        Self {
            raw_pitch: 0,
            raw_roll: 0,
            raw_yaw: 0,
            dynamic_threshold: 8192, // Limită standard de protecție la suprasolicitare senzor
        }
    }

    /// Înregistrează viteza unghiulară brută pe 3 axe direct din registrele hardware
    pub fn capture_angular_velocities(&mut self, pitch: i16, roll: i16, yaw: i16) -> Result<(), &'static str> {
        // Filtrare asimetrică: eliminăm valorile aberante generate de șocuri exterioare extreme
        if pitch.abs() > self.dynamic_threshold || roll.abs() > self.dynamic_threshold || yaw.abs() > self.dynamic_threshold {
            return Err("FaLL-Inertial Gate: Gyroscopic spike overflow blocked successfully.");
        }

        self.raw_pitch = pitch;
        self.raw_roll = roll;
        self.raw_yaw = yaw;

        Ok(())
    }

    /// Exportă cadrele unghiulare în format de vector fix pentru analizatorul biometric
    #[inline]
    pub const fn read_gyro_axes(&self) -> [i16; 3] {
        [self.raw_pitch, self.raw_roll, self.raw_yaw]
    }
}

impl Default for GyroscopeDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// Destructor automat pentru curățarea valorilor de mișcare din memoria volatile post-procesare
impl Drop for GyroscopeDriver {
    fn drop(&mut self) {
        self.raw_pitch = 0;
        self.raw_roll = 0;
        self.raw_yaw = 0;
        self.dynamic_threshold = 0;
    }
}
