use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use std::str::FromStr;
use std::env;
mod big_hex;
mod big_int_to_hex;
mod bits_target;
fn main() 
{
    let bits = "1729d72d";
    let target =  bits_target(bits);
}
