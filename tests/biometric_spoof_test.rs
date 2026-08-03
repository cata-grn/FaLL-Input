// tests/biometric_spoof_test.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Biometric spoofing and credential-theft mitigation verification test suite.
//! Core Architect & Inventor: R.C.F. (2026)

mod common;

use fall_input::biometrics::{VectorAnalyzer, SensorFramePayload};
use common::MockHardwareFixture;

#[test]
fn test_biometric_spoof_rejection_on_aberration() {
    // 1. Inițializarea infrastructurii de testare izolate în Sandbox
    let _fixture = MockHardwareFixture::setup_sandbox();
    let mut analyzer = VectorAnalyzer::new();

    // 2. Simulare atacator: a furat cifrele, dar amprenta lui cinematică pe cele 9 axe este complet greșită
    let mut malicious_payload = SensorFramePayload {
        accel_data: [900, -800, 1500],   // Anomalie masivă de accelerație (mână nesigură sau robot)
        gyro_data: [-400, 300, 900],     // Răsucire unghiulară total deviată față de tiparul R.C.F.
        pressure_raw: 950,               // Apăsare fizică violentă, neconformă cu calibrarea
        temporal_delta: 650,             // Timp de zbor erratic, indicând ezitare sau introducere manuală străină
    };

    // 3. Executarea analizei de fuziune senzorială pe stivă
    let mut evaluation_registry = fall_input::biometrics::BiometricSensorRegistry::new();
    evaluation_registry.acquire_registry_lock();

    // 4. Încercarea de validare a cadrului compromis
    let validation_result = analyzer.verify_integrity_matrix(&malicious_payload);

    // 5. Validarea Matematică a Apărării: Sistemul TREBUIE să returneze eroare (Respingere)
    assert!(
        validation_result.is_err(),
        "FaLL-Security Failure: Malicious biometric spoofing payload bypassed verification layers."
    );

    // Curățarea forțată a reziduurilor din bufferul simulat de atac
    let clean_res = evaluation_registry.commit_sensor_frame(&mut malicious_payload);
    assert!(clean_res.is_ok());
}

#[test]
fn self_destruct_on_aberration() {
    let _fixture = MockHardwareFixture::setup_sandbox();
    let mut analyzer = VectorAnalyzer::new();

    let aberrant_payload = SensorFramePayload {
        accel_data: [900, -800, 1500],
        gyro_data: [-400, 300, 900],
        pressure_raw: 950,
        temporal_delta: 650,
    };

    let first_result = analyzer.verify_integrity_matrix(&aberrant_payload);
    assert!(
        first_result.is_err(),
        "FaLL-Security Failure: The analyzer failed to trigger its self-destruct gate on aberrant input."
    );

    let second_result = analyzer.verify_integrity_matrix(&aberrant_payload);
    assert!(
        second_result.is_err(),
        "FaLL-Security Failure: The analyzer did not remain locked after the initial self-destruct event."
    );
}
