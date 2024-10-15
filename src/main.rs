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
*/
