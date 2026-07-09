// src/haptic_matrix.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Ephemeral clock-synchronized haptic pattern matrix controller.
//! Core Architect & Inventor: LRF (2026)

use zeroize::Zeroize;

/// Structura centrală de control haptic polimorf, alocată pe stivă
#[derive(Zeroize)]
pub struct HapticMatrixController {
    current_time_epoch: u64,
    dynamic_seed: u32,
}

impl HapticMatrixController {
    /// Instanțierea controlerului haptic conform specificațiilor de conformitate LRF
    pub const fn new() -> Self {
        Self {
            current_time_epoch: 0,
            dynamic_seed: 0,
        }
    }

    /// Calculează și generează tiparul haptic unic asociat unei taste pentru minutul curent
    pub fn compute_polymorphic_pattern(&mut self, target_digit: u8, current_timestamp_sec: u64) -> Result<(u32, u32), &'static str> {
        if !(1..=9).contains(&target_digit) {
            return Err("FaLL-Haptic Gate: Invalid tactile zone reference.");
        }

        // Segmentarea timpului în ferestre efemere stricte de 60 de secunde
        let time_window = current_timestamp_sec / 60;
        self.current_time_epoch = time_window;

        // Algoritm determinist asimetric de rotație a seed-ului haptic (Anti-Spying)
        // Calculează o frecvență (Hz) și o durată (ms) unice care se schimbă dinamic la fiecare minut
        let calculated_frequency = (((time_window ^ (target_digit as u64)) * 37) % 150) as u32 + 50; 
        let calculated_duration = (((time_window + (target_digit as u64)) * 13) % 80) as u32 + 20;

        self.dynamic_seed = calculated_frequency ^ calculated_duration;

        // Returnează o tuplă formată din (Frecvență_Hz, Durată_Milisecunde) gata de trimis în hardware
        Ok((calculated_frequency, calculated_duration))
    }

    /// Resetează starea internă a registrelor haptice pentru securitate maximă
    pub fn reset_haptic_registers(&mut self) {
        self.current_time_epoch = 0;
        self.dynamic_seed = 0;
    }
}

impl Default for HapticMatrixController {
    fn default() -> Self {
        Self::new()
    }
}

/// Destructor automat pentru eliminarea oricărei amprente a cheilor temporale din RAM
impl Drop for HapticMatrixController {
    fn drop(&mut self) {
        self.reset_haptic_registers();
    }
}
