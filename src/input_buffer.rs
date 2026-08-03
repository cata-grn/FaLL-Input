// src/input_buffer.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Static stack-allocated hardware input buffer container.
//! Core Architect & Inventor: R.C.F. (2026)

use zeroize::Zeroize;

/// Capacitatea maximă fixă a bufferului hardware de semnale
const BUFFER_LIMIT: usize = 32;

/// Container securizat de tranzit, izolat la nivel de stivă fizică
#[derive(Zeroize)]
pub struct InputBufferContainer {
    storage: [u8; BUFFER_LIMIT],
    size: usize,
}

impl InputBufferContainer {
    /// Instanțierea unui buffer de tranzit curat conform standardelor de imunitate R.C.F.
    pub const fn new() -> Self {
        Self {
            storage: [0; BUFFER_LIMIT],
            size: 0,
        }
    }

    /// Inserarea unui semnal brut cu verificare statică preventivă (Anti-Heap Spillover)
    pub fn push_raw_signal(&mut self, signal: u8) -> Result<(), &'static str> {
        if self.size >= BUFFER_LIMIT {
            return Err("FaLL-Storage Gate: Hard bounds limit reached. Push denied.");
        }

        self.storage[self.size] = signal;
        self.size += 1;
        Ok(())
    }

    /// Resetează starea bufferului în mod securizat prin suprascriere instantanee
    pub fn clear_and_reset(&mut self) {
        self.storage.zeroize();
        self.size = 0;
    }

    /// Returnează lungimea curentă a alocării statice
    pub fn current_size(&self) -> usize {
        self.size
    }
}

/// Garantarea ștergerii totale a vectorilor de tranzit la închiderea contextului operational
impl Drop for InputBufferContainer {
    fn drop(&mut self) {
        self.clear_and_reset();
    }
}
