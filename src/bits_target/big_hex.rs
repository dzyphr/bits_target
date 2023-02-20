use num_bigint::BigInt;
use std::env;
pub fn hex_to_int(hex: String) -> BigInt
{
    //Instantiate variables
    let mut print_or_not = false; //Boolean to determine debug printing true=on/false=off
    let mut char_pre_val: i64 = 0; //Character value in integer 64
    let mut total: BigInt = BigInt::from(0); //BigInt total of hex value
    let hex_size = hex.chars(); //Character reference from the hexadecimal
    let hex_size_size = hex_size.count(); //A size type reference to the length of the hexadecimal
    let hex_size_int: i64 = hex_size_size as i64; //The size of the hex string converted into integer 64
    let mut hex_char_locus: i64 = -1; //An index to determine location of the current character being processed
    for i in hex.chars() //Index through every character of the hex string and store the character in i
    {
        hex_char_locus = hex_char_locus + 1; //This function increments the variable determining the location of the current char 
        let current_hex_power = hex_size_int - hex_char_locus - 1; //Determine the current power to which the hex will be computed
        //Cases to convert expected  alphabetical characters into corresponding hexadecimal value
        if (i == 'A') || (i == 'a')
        {
            char_pre_val = 10;
        }
        else if (i == 'B') || (i == 'b')
        {
            char_pre_val = 11;
        }
        else if (i == 'C') || (i == 'c')
        {
            char_pre_val = 12;
        }
        else if (i == 'D') || (i == 'd')
        {
            char_pre_val = 13;
        }
        else if (i == 'E') || (i == 'e')
        {
            char_pre_val = 14;
        }
        else if (i == 'F') || (i == 'f')
        {
            char_pre_val = 15;
        }
        else //Check if an integer or a non supported character is given
        {
            if (i.is_digit(10)) && (i.is_digit(16)) && (i.is_alphanumeric()) //check for integer
            {
                let i_to_int: i64 = i as i64; //Convert the ascii value into i64 through raw conversion
                char_pre_val = i_to_int - 48; //Subtract 48 because this is the known ascii->int offset for number characters
            }
            else if (i.is_alphanumeric()) && (i.is_alphabetic()) //Check for unexpected alphabetical character
            {
                println!("Non hexadecimal compatible character was given!\nCharacter:   {}", i);
                break;
            }
        }
        if print_or_not == true {println!("current char pre val:     {}", char_pre_val);}
        let big_current_hex_pow: BigInt = BigInt::from(current_hex_power) as BigInt; //current hex power as big integer
        if print_or_not == true{println!("regular hex power: {}\n big hex power: {}", current_hex_power, big_current_hex_pow);}
        let mut pow_total: BigInt = BigInt::from(1); //Instantiate total power collected variable
        if current_hex_power != 0 //if the current hex power is 0, pow_total stays valued at its default 1.
        //This loop raises pow total to the power of 16 the amount of times that the current hex power dictates
        //It uses BigInt to do the calculation so that overflows should not be thrown
        {
            for _ in 0..current_hex_power
            {
                pow_total = pow_total * BigInt::from(16);
            }

        }
        if print_or_not == true{println!("current hex power val:    {}", pow_total);}
        let big_char_pre_val: BigInt = BigInt::from(char_pre_val) as BigInt; //Char pre val as big integer
        let char_hex_val: BigInt =  big_char_pre_val *  pow_total; //Calculate the character pre value multiplied by the power total
        if print_or_not == true{println!("current hex->int value:   {}", char_hex_val);}
        total = total + char_hex_val; //Collect the total stored value throughout each iteration of the calculations
    }
    if print_or_not == true{println!("Total hex value collected:    {}", total);}
    return total;
}
