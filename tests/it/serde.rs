#[test]
fn test_serde() {
    let hash_str = "T12D900249414E0BD59A46503F3ADA802AE50825242B2590561CF690599112214C051556";
    let tlsh: tlsh2::TlshDefault = hash_str.parse().unwrap();

    let json = serde_json::to_string(&tlsh).unwrap();
    // In JSON, it should be serialized as a string
    assert_eq!(json, format!("\"{}\"", hash_str));

    let tlsh_deser: tlsh2::TlshDefault = serde_json::from_str(&json).unwrap();
    assert_eq!(tlsh.hash(), tlsh_deser.hash());
}
