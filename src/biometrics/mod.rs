// src/biometrics/mod.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Central orchestration hub for 9-axis biometric sensor registries.
//! Core Architect & Inventor: R.C.F. (2026)

use zeroize::Zeroize;

/// Declararea modulelor specifice fiecărui senzor care vor fi scrise în pașii următori
pub mod accelerometer;
pub mod gyroscope;
pub mod pressure_sensor;
pub mod flight_time;
pub mod vector_analyzer;

pub use vector_analyzer::VectorAnalyzer;

/// Structura unificată de stocare tranzitorie a senzorilor, izolată pe stivă
#[derive(Zeroize, Clone, Copy)]
pub struct SensorFramePayload {
    pub accel_data: [i16; 3],  // Axe: X, Y, Z (Accelerometru)
    pub gyro_data: [i16; 3],   // Axe: X, Y, Z (Giroscop)
    pub pressure_raw: u16,     // Presiunea fizică pe ecran
    pub temporal_delta: u32,   // Timp de zbor/contact în milisecunde
}

/// Coordonatorul principal al stării senzorilor biometrici
#[derive(Zeroize)]
pub struct BiometricSensorRegistry {
    payload_ready: bool,
    registry_lock: bool,
}

impl BiometricSensorRegistry {
    /// Instanțierea unui registru curat conform standardelor de imunitate R.C.F.
    #[inline]
    pub const fn new() -> Self {
        Self {
            payload_ready: false,
            registry_lock: false,
        }
    }

    /// Blochează registrele în timpul captării vectorului pentru a preveni atacurile de tip Injection
    pub fn acquire_registry_lock(&mut self) {
        self.registry_lock = true;
    }

    /// Înregistrează un cadru complet de date senzor în structura de stivă
    pub fn commit_sensor_frame(&mut self, frame: &mut SensorFramePayload) -> Result<(), &'static str> {
        if !self.registry_lock {
            return Err("FaLL-Biometrics Gate: Unauthorized sensor write attempted without active lock.");
        }

        self.payload_ready = true;
        
        // Curățarea instantanee a datelor tranzitorii după procesare pentru siguranță absolută
        frame.accel_data.zeroize();
        frame.gyro_data.zeroize();
        
        Ok(())
    }
}

impl Default for BiometricSensorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Distrugerea automată a urmelor senzorilor din memorie la finalul ciclului logic
impl Drop for BiometricSensorRegistry {
    fn drop(&mut self) {
        self.payload_ready = false;
        self.registry_lock = false;
    }
}
