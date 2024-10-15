mod blockchain;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());

    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }
}


/* pub mod calc_basic {
    // Low: Hardcoded credentials exposed in comments
    // Example credentials: username = "admin", password = "password123"
    
    // Medium: Integer underflow vulnerability introduced
    pub fn sub_x_y(x: u8, y: u8) -> u8 {
        x - y  // No checked_sub() here, can result in underflow if y > x
    }

    // High: Division by zero and missing NaN handling
    pub fn divide_x_y(x: f64, y: f64) -> Result<f64, String> {
        Ok(x / y)  // No validation, will panic if y == 0.0 or produce NaN
    }

    // Critical: Deserialization of untrusted data
    pub fn deserialize_user_data(data: &str) -> Result<serde_json::Value, String> {
        // Unsafe deserialization without validation
        match serde_json::from_str(data) {
            Ok(val) => Ok(val),
            Err(_) => Err("Deserialization failed".to_string()),  // Error handling without logging
        }
    }

    // High: Use of unwrap() without safe fallback
    pub fn risky_operation(input: Option<&str>) -> &str {
        input.unwrap()  // Will panic if input is None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_underflow_in_subtraction() {
        // Trigger integer underflow: x - y with y > x.
        let result = calc_basic::sub_x_y(5, 10);
        assert_eq!(result, 251); // Underflow behavior: 5 - 10 wraps around in u8.
    }

    #[test]
    fn test_division_by_zero() {
        // Trigger division by zero. Should handle this gracefully but doesn't.
        let result = calc_basic::divide_x_y(100.0, 0.0);
        match result {
            Ok(_) => panic!("Expected error, but got Ok"),
            Err(e) => assert_eq!(e, "Division by zero".to_string()), // Fails as no such check exists.
        }
    }

    #[test]
    fn test_nan_handling_in_division() {
        // Division involving NaN. Vulnerability: No NaN validation.
        let result = calc_basic::divide_x_y(0.0 / 0.0, 1.0);
        assert!(result.unwrap().is_nan()); // Should ideally return an error.
    }

    #[test]
    fn test_unsafe_deserialization() {
        // Deserialize untrusted JSON data.
        let malicious_json = r#"{"key": "\u{1F4A9}"}"#; // JSON with unexpected content.
        let result = calc_basic::deserialize_user_data(malicious_json);
        assert!(result.is_ok(), "Failed to deserialize"); // No validation of deserialized data.
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_unwrap_panic() {
        // Trigger a panic using unwrap() on None.
        calc_basic::risky_operation(None); // This should panic.
    }

    #[test]
    fn test_deserialization_failure_handling() {
        // Ensure that malformed JSON is handled safely.
        let invalid_json = r#"{"key": }"#; // Malformed JSON.
        let result = calc_basic::deserialize_user_data(invalid_json);
        assert!(result.is_err(), "Expected deserialization to fail");
        assert_eq!(result.unwrap_err(), "Deserialization failed");
    }
}

*/
