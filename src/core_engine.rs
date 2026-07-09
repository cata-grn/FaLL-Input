// src/core_engine.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Multi-tap algorithmic non-string sequencial translator.
//! Core Architect & Inventor: LRF (2026)

use zeroize::Zeroize;

/// Limita absolută de caractere numerice stocate efemer pentru a preveni atacurile de memorie
const MAX_BUFFER_CAPACITY: usize = 16;

/// Structura securizată a buffer-ului de intrare, alocată integral pe STIVĂ (Stack)
#[derive(Zeroize)]
pub struct MultiTapEngine {
    // Stocăm doar impulsuri numerice de 8 biți (u8), eliminând complet tipurile String/Char
    raw_pulses: [u8; MAX_BUFFER_CAPACITY],
    current_idx: usize,
    last_pulse_timestamp: u64,
}

impl MultiTapEngine {
    /// Inițializarea unei matrice oarbe de input cu curățare nativă post-execuție
    pub const fn new() -> Self {
        Self {
            raw_pulses: [0; MAX_BUFFER_CAPACITY],
            current_idx: 0,
            last_pulse_timestamp: 0,
        }
    }

    /// Înregistrarea chirurgicală a unui impuls numeric de la interfață hardware (Zero Buffer Overflow)
    pub fn register_hardware_pulse(&mut self, digit: u8, timestamp: u64) -> Result<(), &'static str> {
        if self.current_idx >= MAX_BUFFER_CAPACITY {
            return Err("FaLL-Engine Boundary Error: Buffer overflow blocked successfully.");
        }

        // Constrângere logică strictă: acceptăm exclusiv zonele tactile numerice 1-9
        if !(1..=9).contains(&digit) {
            return Err("FaLL-Engine Validation Error: Malformed input digit rejected.");
        }

        self.raw_pulses[self.current_idx] = digit;
        self.current_idx += 1;
        self.last_pulse_timestamp = timestamp;
        Ok(())
    }

    /// Returnează starea curentă a vectorului numeric fără a expune datele pe internet
    pub fn fetch_vector_state(&self) -> &[u8] {
        &self.raw_pulses[..self.current_idx]
    }
}

impl Default for MultiTapEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Destructor automat - Suprascrie RAM-ul cu zerouri în momentul în care variabila părăsește contextul
impl Drop for MultiTapEngine {
    fn drop(&mut self) {
        self.raw_pulses.zeroize();
        self.current_idx = 0;
        self.last_pulse_timestamp = 0;
    }
}
