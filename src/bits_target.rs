use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use std::str::FromStr;
use std::env;
mod big_hex;
mod big_dec_pow;
mod big_int_to_hex;
pub fn bits_target(bits: String) -> String
{
    let mut debug = true;
    if debug == true{println!("bits: {}", bits.to_string());}
    let index = get_index(bits.to_string());
    let chopped_bits = &chop_bits(bits.to_string());
    if debug == true{println!("right side bits: {}", chopped_bits.to_string());}
    if debug == true{println!("index: {}, int value: {}", index.to_string(), big_hex::hex_to_int(index.to_string()))}
    let a = big_hex::hex_to_int(chopped_bits.to_string());
    if debug == true{println!("int val of right side bits: {}", a);}
    let b = BigInt::from(2);
    let c = BigInt::from(8);
    let d = big_hex::hex_to_int(index);
    let e = BigInt::from(3);
    let step1 = &d - &e;
    if debug == true{println!("f1 = index - 3 = {}", step1);}
    let step2 = &c * &step1;
    if debug == true{println!("f2 = 8 * f1 = {}", step2);}
    let b_ = BigDecimal::from_str(&b.to_string()).unwrap();
    let step2_ = BigDecimal::from_str(&step2.to_string()).unwrap();
    let step3_ = big_dec_pow::big_dec_pow(b_, step2_);
    if debug == true{println!("f3 = 2 ^ f2 = {}", step3_);}
    let step3 = BigInt::from_str(&step3_.to_string()).unwrap();
    let step4 = &a * &step3; 
    if debug == true{println!("f4 = {} * f3 = {}", a,  step4);}
    let mut ans = BigInt::from_str(&step4.to_string()).unwrap();
    let dec_ans = ans.clone();
    let mut hex_ans = big_int_to_hex::int_to_hex(ans);
    if debug == true{println!("{} * 2 ^ (8 * (index - 3))",a);} 
    //a * b ^ (c * (d - e));
    println!("decimal ans:   {}\nhex ans:  {}",dec_ans,  &mut  hex_ans);
    let mut target = hex_ans;
    while target.chars().count() != 64
    {
        target.insert(0, '0');    
    } 
    //println!("Target:    {}", target);
    return target;
}
fn get_index(bits: String) -> String
{
    let mut index = String::new();
    let char1 = bits.chars().nth(0).unwrap();
    //println!("char 0: {}", index);
    let char2 = bits.chars().nth(1).unwrap();
    index = char1.to_string() + &char2.to_string();
    return index.to_string();
}
fn chop_bits(mut bits: String) -> String
{
    let mut chars = bits.chars();
    chars.next();
    chars.next();
    return chars.as_str().to_string();
}

