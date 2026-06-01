// src/biometrics/flight_time.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! High-precision hardware flight and contact time microsecond biometric counter.
//! Core Architect & Inventor: LRF (2026)

use zeroize::Zeroize;

/// Structura de izolare a registrelor temporale biometrice, alocată pe stivă
#[derive(Zeroize)]
pub struct FlightTimeCounter {
    press_timestamp_ms: u64,
    release_timestamp_ms: u64,
    maximum_delta_threshold: u32,
}

impl FlightTimeCounter {
    /// Instanțierea unui cronometru hardware curat sub designul de conformitate LRF
    pub const fn new() -> Self {
        Self {
            press_timestamp_ms: 0,
            release_timestamp_ms: 0,
            maximum_delta_threshold: 3000, // Limită standard de 3 secunde pentru abandon sesiune
        }
    }

    /// Înregistrează momentul electric exact al impactului degetului cu digitizer-ul
    pub fn mark_hardware_press(&mut self, timestamp_ms: u64) {
        self.press_timestamp_ms = timestamp_ms;
    }

    /// Înregistrează momentul ridicării degetului de pe ecran și calculează timpul de contact
    pub fn mark_hardware_release(&mut self, timestamp_ms: u64) -> Result<u32, &'static str> {
        if timestamp_ms < self.press_timestamp_ms {
            return Err("FaLL-Temporal Gate: Clock-skew or hardware time-tampering detected.");
        }

        self.release_timestamp_ms = timestamp_ms;
        let contact_duration = (timestamp_ms - self.press_timestamp_ms) as u32;

        if contact_duration > self.maximum_delta_threshold {
            return Err("FaLL-Temporal Gate: Execution timeout. Session invalidated.");
        }

        Ok(contact_duration)
    }

    /// Calculează timpul de zbor dintre două evenimente de tastare distincte
    pub fn calculate_flight_bridge(&self, next_press_ms: u64) -> u32 {
        if next_press_ms < self.release_timestamp_ms {
            return 0;
        }
        (next_press_ms - self.release_timestamp_ms) as u32
    }

    /// Resetarea completă a registrelor temporale pentru securitate post-execuție
    pub fn wipe_temporal_metrics(&mut self) {
        self.press_timestamp_ms = 0;
        self.release_timestamp_ms = 0;
    }
}

/// Destructor Safe-Drop pentru ștergerea automată a amprentelor cronologice din RAM volatile
impl Drop for FlightTimeCounter {
    fn drop(&mut self) {
        self.wipe_temporal_metrics();
        self.maximum_delta_threshold = 0;
    }
}
