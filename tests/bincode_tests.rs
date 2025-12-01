use bincode::config::standard;
use smol_str::{SmolStr, ToSmolStr};

#[test]
fn bincode_serialize_stack() {
    let smolstr_on_stack = "aßΔCaßδc".to_smolstr();
    let config = standard();
    let encoded = bincode::encode_to_vec(&smolstr_on_stack, config).unwrap();
    let decoded: SmolStr = bincode::decode_from_slice(&encoded, config).unwrap().0;
    assert_eq!(smolstr_on_stack, decoded);
}

#[test]
fn bincode_serialize_heap() {
    let smolstr_on_heap =
        "aßΔCaßδcaßΔCaßδcaßΔCaßδcaßΔCaßδcaßΔCaßδcaßΔCaßδcaßΔCaßδcaßΔCaßδcaßΔCaßδcaßΔCaßδc"
            .to_smolstr();
    let config = standard();
    let encoded = bincode::encode_to_vec(&smolstr_on_heap, config).unwrap();
    let decoded: SmolStr = bincode::decode_from_slice(&encoded, config).unwrap().0;
    assert_eq!(smolstr_on_heap, decoded);
}

#[test]
fn bincode_non_utf8_failure() {
    let invalid_utf8_bytes: Vec<u8> = vec![0xF0, 0x9F, 0x8F]; // Incomplete UTF-8 sequence
    let invalid_smol_str =
        SmolStr::from(unsafe { String::from_utf8_unchecked(invalid_utf8_bytes.clone()) });

    // For encoding, bincode will serialize the raw bytes, so it should succeed.
    // However, for SmolStr, the actual validation happens during the String::decode phase.
    let config = standard(); // Use standard config as requested
    let encoded_invalid_utf8_smol_str = bincode::encode_to_vec(&invalid_smol_str, config).unwrap();

    // Decoding these bytes into a SmolStr (which internally calls String::decode) should fail
    // due to UTF-8 validation.
    let decode_result: Result<SmolStr, _> =
        bincode::decode_from_slice(&encoded_invalid_utf8_smol_str, config).map(|(s, _)| s);
    assert!(decode_result.is_err());
}
