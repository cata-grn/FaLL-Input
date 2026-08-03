#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Extended regression suite for deterministic vectors, FHE encoding,
//! haptic polymorphism, TOTP seeding, and stack-only zeroization behavior.
//! Core Architect & Inventor: R.C.F. (2026)

mod common;

use common::MockHardwareFixture;
use fall_input::biometrics::{SensorFramePayload, VectorAnalyzer};
use fall_input::core_engine::MultiTapEngine;
use fall_input::fhe_encoder::FheEncoder;
use fall_input::haptic::totp_generator::HapticTotpGenerator;
use fall_input::haptic_matrix::HapticMatrixController;
use fall_input::memory_manager::VolatileMemoryScrubber;

#[test]
fn test_nine_axis_biometric_vector_accepts_valid_profile_below_threshold() {
    let _fixture = MockHardwareFixture::setup_sandbox();
    let mut analyzer = VectorAnalyzer::new();

    let valid_payload = SensorFramePayload {
        accel_data: [151, 91, 451],
        gyro_data: [11, -4, 31],
        pressure_raw: 105,
        temporal_delta: 55,
    };

    let result = analyzer.verify_integrity_matrix(&valid_payload);
    assert!(result.is_ok(), "FaLL-Security Failure: Valid biometric profile was incorrectly rejected.");
}

#[test]
fn test_nine_axis_biometric_vector_rejects_aberrant_payload_above_threshold() {
    let _fixture = MockHardwareFixture::setup_sandbox();
    let mut analyzer = VectorAnalyzer::new();

    let aberrant_payload = SensorFramePayload {
        accel_data: [900, -800, 1500],
        gyro_data: [-400, 300, 900],
        pressure_raw: 950,
        temporal_delta: 650,
    };

    let result = analyzer.verify_integrity_matrix(&aberrant_payload);
    assert!(result.is_err(), "FaLL-Security Failure: Aberrant biometric payload bypassed the self-destruct gate.");
}

#[test]
fn test_fhe_encoding_on_valid_inputs_is_deterministic_across_15000_vectors() {
    let mut encoder = FheEncoder::new();

    for index in 0..15_000 {
        let digit = (index % 9) as u8 + 1;
        let vector = [digit; 8];

        assert!(encoder.encode_vector_to_matrix(&vector).is_ok());

        let matrix_state = encoder.export_matrix_state();
        let expected = ((digit as u16) * 512) % 4096;
        assert_eq!(matrix_state[0], expected);
    }
}

#[test]
fn test_haptic_polymorphic_pattern_is_deterministic_and_time_sensitive() {
    let mut controller = HapticMatrixController::new();

    let baseline = controller.compute_polymorphic_pattern(7, 1_700_000_000).unwrap();
    let repeated = controller.compute_polymorphic_pattern(7, 1_700_000_000).unwrap();
    let shifted = controller.compute_polymorphic_pattern(3, 1_700_000_060).unwrap();

    assert_eq!(baseline, repeated);
    assert_ne!(baseline, shifted);
}

#[test]
fn test_totp_generator_is_deterministic_within_window_and_resets_on_cleanup() {
    let mut generator = HapticTotpGenerator::new();

    let first = generator.compute_ephemeral_step(1_700_000_000);
    let second = generator.compute_ephemeral_step(1_700_000_000);
    assert_eq!(first, second);

    generator.clean_totp_registers();
    let after_cleanup = generator.compute_ephemeral_step(1_700_000_000);
    assert_ne!(first, after_cleanup);
}

#[test]
fn test_multi_tap_engine_preserves_numeric_vectors_without_plaintext_leak() {
    let fixture = MockHardwareFixture::setup_sandbox();
    let mut engine = MultiTapEngine::new();

    for digit in [1, 2, 3, 4, 5, 6, 7, 8, 9] {
        let touch = fixture.create_mock_touch(100 + digit as u32, 120, 450);
        let resolved_digit = fixture.hal.process_raw_hardware_input(&touch).unwrap();
        engine.register_hardware_pulse(resolved_digit, fixture.simulated_time_ms).unwrap();
    }

    let state = engine.fetch_vector_state();
    assert_eq!(state.len(), 9);
    assert!(state.iter().all(|&value| value & 0x80 != 0));
    assert!(std::str::from_utf8(state).is_err());
}

#[test]
fn test_memory_scrubber_reports_clean_state_after_immediate_scrub() {
    let mut scrubber = VolatileMemoryScrubber::new();

    scrubber.lock_memory_zone();
    assert!(!scrubber.verify_cleanliness_status());

    scrubber.force_immediate_scrub();
    assert!(scrubber.verify_cleanliness_status());
}

#[test]
fn test_stack_only_structures_use_fixed_size_buffers() {
    assert_eq!(std::mem::size_of::<MultiTapEngine>(), 32);
    assert_eq!(std::mem::size_of::<FheEncoder>(), 16);
    assert_eq!(std::mem::size_of::<HapticMatrixController>(), 16);
}
