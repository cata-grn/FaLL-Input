// tests/common.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Central test harness and hardware simulation sandbox environment.
//! Core Architect & Inventor: LRF (2026)

use fall_input::hal::{RawHardwareTouch, HardwareAbstractionLayer, DisplayMode};
use fall_input::biometrics::SensorFramePayload;

/// Fișier de test modular ce simulează comportamentul unui microprocesor mobil
pub struct MockHardwareFixture {
    pub hal: HardwareAbstractionLayer,
    pub simulated_time_ms: u64,
}

impl MockHardwareFixture {
    /// Inițializarea unui mediu izolat de test conform ghidului de audit LRF
    pub fn setup_sandbox() -> Self {
        let mut hal_layer = HardwareAbstractionLayer::new();
        hal_layer.enforce_display_mode(DisplayMode::BlindGeometry);
        
        Self {
            hal: hal_layer,
            simulated_time_ms: 1_700_000_000_000, // Timp Epoch simulat stabil static
        }
    }

    /// Generează un impuls tactil hardware simulat curat
    pub fn create_mock_touch(&self, x: u32, y: u32, pressure: u16) -> RawHardwareTouch {
        RawHardwareTouch {
            coordinate_x: x,
            coordinate_y: y,
            active_pressure: pressure,
        }
    }

    /// Generează un cadru de senzori perfect calibrat pe amprenta biologică master LRF
    pub fn generate_master_biometric_frame(&self) -> SensorFramePayload {
        SensorFramePayload {
            accel_data: [150, 90, 450],
            gyro_data: [10, -5, 30],
            pressure_raw: 450,
            temporal_delta: 100,
        }
    }

    /// Incrementează ceasul simulat pentru a testa ferestrele haptice efemere
    pub fn advance_clock_seconds(&mut self, seconds: u64) {
        self.simulated_time_ms += seconds * 1000;
    }
