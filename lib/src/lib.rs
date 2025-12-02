
#[cfg(feature = "feature_a")]
pub fn lib_fun() -> String {
    "From library: Feature A active".to_string()
}

#[cfg(not(feature = "feature_a"))]
pub fn lib_fun() -> String {
    "From library: Feature A inactive".to_string()
}
