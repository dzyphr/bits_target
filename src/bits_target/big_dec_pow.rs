use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use std::str::FromStr;
pub fn big_dec_pow(base: BigDecimal, exp: BigDecimal) -> BigDecimal
{
    let mut answer = base.clone();
    let mut i = BigDecimal::from_u32(1).unwrap();
    while i<exp
    {
        i = i + BigDecimal::from_u32(1).unwrap();
        answer = answer * base.clone();
    }
    return answer;
}
