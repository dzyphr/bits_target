use num_bigint::BigInt;
use std::env;
use std::str::FromStr;
pub fn int_to_hex(int: BigInt) -> String
{
    let mut DEBUG = false;
    let mut hex_string = String::new();
    let mut ans = int;
    let mut _16: BigInt = BigInt::from(16);
    while &ans !=&BigInt::from(0)
    {
        let mut hex_digit = String::new();
        if DEBUG == true {println!("Quotient:  {}", ans);}
        let quo = ans.clone();
        ans = &ans / &_16;
        let rem  = quo % &_16;
        if DEBUG == true {println!("Remainder: {}", rem);}
        if rem == BigInt::from(10)
        {
            hex_digit = "a".to_string();
        }
        else if rem == BigInt::from(11)
        {
            hex_digit = "b".to_string();
        }
        else if rem == BigInt::from(12)
        {
            hex_digit = "c".to_string();
        }
        else if rem == BigInt::from(13)
        {
            hex_digit = "d".to_string();
        }
        else if rem == BigInt::from(14)
        {
            hex_digit = "e".to_string();
        }
        else if rem == BigInt::from(15)
        {
            hex_digit = "f".to_string();
        }
        else 
        {
            hex_digit = rem.to_string();
        }
        hex_string =  hex_digit + &hex_string;
        if DEBUG == true {println!("Current Hex String:   {}", hex_string);}
    }
    //println!("Final Hex:         {}", hex_string);
    return hex_string;
}
