// src/hal/pocket_mode.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Pocket stealth mode and proximity hardware integration sensor controller.
//! Core Architect & Inventor: LRF (2026)

use zeroize::Zeroize;

/// Structura de control a stării oarbe din buzunar, izolată pe stivă
#[derive(Zeroize)]
pub struct PocketModeController {
    proximity_sensor_active: bool,
    hardware_screen_power: u8, // 0 = Oprit, 1 = Pornit (Mod Geometric)
}

impl PocketModeController {
    /// Instanțierea unui configurator de buzunar curat sub parametrii de design ai lui LRF
    pub const fn new() -> Self {
        Self {
            proximity_sensor_active: false,
            hardware_screen_power: 1,
        }
    }

    /// Evaluează starea senzorului hardware de proximitate și oprește ecranul la nivel electric
    pub fn evaluate_proximity_state(&mut self, sensor_value_close: bool) -> Result<u8, &'strong str> {
        self.proximity_sensor_active = sensor_value_close;

        if sensor_value_close {
            // Senzorul confirmă că telefonul este acoperit/în buzunar. Tăiem curentul din pixeli instant.
            self.hardware_screen_power = 0;
            Ok(0) // Ecran dezactivat complet (Stealth Mode)
        } else {
            self.hardware_screen_power = 1;
            Ok(1) // Ecran în mod geometric neutru
        }
    }

    /// Verifică dacă driverul HAL are voie să ignore randarea vizuală
    pub fn is_stealth_execution_enforced(&self) -> bool {
        self.proximity_sensor_active && self.hardware_screen_power == 0
    }
}

/// Destructor automat pentru ștergerea configurărilor energetice ale ecranului din memoria volatile
impl Drop for PocketModeController {
    fn drop(&mut self) {
        self.proximity_sensor_active = false;
        self.hardware_screen_power = 1;
    }
}
