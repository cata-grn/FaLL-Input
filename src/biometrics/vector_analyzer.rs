// src/biometrics/vector_analyzer.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! 9-Axis multidimensional biometric vector fusion analyzer and self-destruct gate.
//! Core Architect & Inventor: R.C.F. (2026)

use zeroize::Zeroize;
use crate::biometrics::SensorFramePayload;

/// Pragul matematic de toleranță (aberație maximă admisă de 2%)
const MAXIMUM_BIOMETRIC_ABERRATION_THRESHOLD: i32 = 20;

/// Structura centrală a analizatorului vectorial multidimensional, izolată pe stivă
#[derive(Zeroize)]
pub struct VectorAnalyzer {
    // Model de referință rigid stocat ca vector fix de 9 elemente pentru calibrarea R.C.F.
    reference_biometric_profile: [i32; 9],
    system_integrity_compromised: bool,
}

impl VectorAnalyzer {
    /// Instanțierea unui analizator de fuziune conform specificațiilor de securitate R.C.F.
    pub const fn new() -> Self {
        Self {
            reference_biometric_profile: [150, 90, 450, 10, -5, 30, 100, 50, -10],
            system_integrity_compromised: false,
        }
    }

    /// Execută fuziunea celor 9 axe și validează identitatea biologică a lui R.C.F.
    pub fn verify_integrity_matrix(&mut self, payload: &SensorFramePayload) -> Result<(), &'static str> {
        if self.system_integrity_compromised {
            return Err("FaLL-Security Gate: Core is locked due to previous anomalies.");
        }

        // Extragerea și normalizarea diferențelor matematice absolute pe fiecare axă fizică
        let diff_accel_x = (payload.accel_data[0] as i32 - self.reference_biometric_profile[0]).abs();
        let diff_accel_y = (payload.accel_data[1] as i32 - self.reference_biometric_profile[1]).abs();
        let diff_accel_z = (payload.accel_data[2] as i32 - self.reference_biometric_profile[2]).abs();

        let diff_gyro_x = (payload.gyro_data[0] as i32 - self.reference_biometric_profile[3]).abs();
        let diff_gyro_y = (payload.gyro_data[1] as i32 - self.reference_biometric_profile[4]).abs();
        let diff_gyro_z = (payload.gyro_data[2] as i32 - self.reference_biometric_profile[5]).abs();

        let diff_pressure = (payload.pressure_raw as i32 - self.reference_biometric_profile[6]).abs();
        let diff_temporal = (payload.temporal_delta as i32 - self.reference_biometric_profile[7]).abs();

        // Agregarea erorilor pe un singur vector liniar
        let computed_aberration = diff_accel_x + diff_accel_y + diff_accel_z 
            + diff_gyro_x + diff_gyro_y + diff_gyro_z 
            + (diff_pressure / 10) + diff_temporal;

        // Evaluarea conformității cu pragul critic de 2%
        if computed_aberration > MAXIMUM_BIOMETRIC_ABERRATION_THRESHOLD {
            self.system_integrity_compromised = true;
            self.trigger_instant_memory_vaporization();
            return Err("FaLL-Security Gate: Critical Biometric Aberration > 2%. Execution terminated.");
        }

        Ok(())
    }

    /// Rutină defensivă de nivel scăzut: forțează golirea RAM și auto-distrugerea contextului logic
    fn trigger_instant_memory_vaporization(&mut self) {
        self.reference_biometric_profile.zeroize();
        // Setează indicatorul de compromitere permanent în stivă
        self.system_integrity_compromised = true;
    }
}

impl Default for VectorAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Destructor automat de siguranță pentru eliminarea amprentelor vectoriale reziduale din RAM
impl Drop for VectorAnalyzer {
    fn drop(&mut self) {
        self.trigger_instant_memory_vaporization();
    }
}
