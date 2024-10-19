use anyhow::{anyhow, Result};
use num_traits::FromPrimitive;
use tensor_amm::errors::TensorAmmError;
use tensor_marketplace::errors::TensorMarketplaceError;
use tensor_whitelist::errors::TensorWhitelistError;

mod anchor_error;

use anchor_error::AnchorErrorCode;

#[derive(Debug)]
pub struct ErrorParams {
    pub error_code: String,
}

pub fn handle_error(args: ErrorParams) -> Result<()> {
    let error_code = parse_error_code(&args.error_code)?;

    match error_code {
        code if TensorAmmError::from_i32(code as i32).is_some() => print_error(
            "TensorAmmError",
            code,
            &TensorAmmError::from_i32(code as i32).unwrap(),
        ),
        code if TensorWhitelistError::from_i32(code as i32).is_some() => print_error(
            "TensorWhitelistError",
            code,
            &TensorWhitelistError::from_i32(code as i32).unwrap(),
        ),
        code if TensorMarketplaceError::from_i32(code as i32).is_some() => print_error(
            "TensorMarketplaceError",
            code,
            &TensorMarketplaceError::from_i32(code as i32).unwrap(),
        ),
        code if AnchorErrorCode::from_u32(code).is_some() => print_error(
            "Anchor ErrorCode",
            code,
            &AnchorErrorCode::from_u32(code).unwrap(),
        ),
        _ => println!("Unknown error code: {}", error_code),
    }

    Ok(())
}

fn print_error<T: std::fmt::Debug>(error_type: &str, error_code: u32, error: &T) {
    println!("{}:", error_type);
    println!("Error Code: {}", error_code);
    println!("Error Type: {:?}", error);
}

fn parse_error_code(input: &str) -> Result<u32> {
    if let Some(stripped) = input.strip_prefix("0x") {
        // Parse hexadecimal
        u32::from_str_radix(stripped, 16).map_err(|_| anyhow!("Invalid hexadecimal error code"))
    } else {
        // Parse decimal
        input
            .parse::<u32>()
            .map_err(|_| anyhow!("Invalid decimal error code"))
    }
}
