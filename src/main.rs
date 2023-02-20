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
    let mut meets_target = false;
    let bits = "17073039";
    let target =  bits_target::bits_target(bits.to_string());
    dbg!(&target);
/*    println!("target:         {}", target);
    let hash = "0000000000000000000d8b4025c6356088d75a7f3e6818411bab2b748947dcda";
    let hashint = big_hex::hex_to_int(hash.to_string());
    let targetint = big_hex::hex_to_int(target.to_string());
    let hashdec = BigDecimal::from_str(&hashint.to_string()).unwrap();
    let targetdec = BigDecimal::from_str(&targetint.to_string()).unwrap();
    let difficulty = targetdec / hashdec;
    if hashint < targetint
    {
        meets_target = true;
    }
    println!("Difficulty:     {}\nTarget Met:     {}", difficulty, meets_target);*/
}
