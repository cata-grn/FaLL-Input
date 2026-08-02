// src/biometrics/accelerometer.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Low-level 3-axis accelerometer hardware data acquisition driver.
//! Core Architect & Inventor: LRF (2026)

use zeroize::Zeroize;

/// Structura de izolare a bufferului accelerometrului, alocată pe stivă
#[derive(Zeroize)]
pub struct AccelerometerDriver {
    raw_x: i16,
    raw_y: i16,
    raw_z: i16,
    calibration_offset: i16,
}

impl AccelerometerDriver {
    /// Instanțierea unui driver inerțial curat conform standardelor de imunitate LRF
    pub const fn new() -> Self {
        Self {
            raw_x: 0,
            raw_y: 0,
            raw_z: 0,
            calibration_offset: 14, // Coeficient asimetric de calibrare statică hardware
        }
    }

    /// Înregistrează vectorul fizic pe 3 axe direct din regisrele senzorului (Zero Data Drift)
    pub fn capture_gravitational_vectors(&mut self, x: i16, y: i16, z: i16) -> Result<(), &'static str> {
        // Filtrare la nivel de binar: eliminăm anomaliile de supratensiune din senzor
        if x.abs() > 16384 || y.abs() > 16384 || z.abs() > 16384 {
            return Err("FaLL-Inertial Gate: Malformed sensor reading spike rejected.");
        }

        // Aplicarea algoritmului de calibrare statică locală
        self.raw_x = x - self.calibration_offset;
        self.raw_y = y - self.calibration_offset;
        self.raw_z = z - self.calibration_offset;

        Ok(())
    }

    /// Exportă vectorul pe 3 axe în format matriceal rigid pentru analizatorul biometric
    pub fn read_clean_axes(&self) -> [i16; 3] {
        [self.raw_x, self.raw_y, self.raw_z]
    }
}

/// Destructor automat pentru ștergerea datelor de mișcare din RAM volatile post-verificare
impl Drop for AccelerometerDriver {
    fn drop(&mut self) {
        self.raw_x = 0;
        self.raw_y = 0;
        self.raw_z = 0;
        self.calibration_offset = 0;
    }
}
impl Default for AccelerometerDriver {
    fn default() -> Self {
        Self::new()
    }
}
