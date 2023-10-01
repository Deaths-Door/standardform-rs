use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::{pair, preceded},
    IResult, 
    bytes::complete::tag, number::complete::double,
};

use crate::StandardForm;

/// Parses a string in standard form (scientific notation) and returns a `StandardForm` struct.
///
/// The standard form number can be written as , "mantissaeexponent", 
/// "mantissaEexponent", or "mantissa*10^exponent". It supports both positive and negative exponents.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// Returns a `Result` containing the parsed `StandardForm` struct if successful, or a parsing error.
#[must_use = "Whats the point of using this then"]
pub fn parse_standard_form(input: &str) -> IResult<&str, StandardForm> {
    map(
        pair(double,parse_exponent),
        |(mantissa , exponent)| StandardForm::new(mantissa,exponent.unwrap_or(0)) 
    )(input)
}

fn parse_exponent(input : &str) -> IResult<&str,Option<i8>> {
    opt(preceded(
        alt((tag("e"), tag("E"), tag("*10^"))),
        map(double, |exp: f64| exp as i8)
    ))(input)
}

#[cfg(test)]
mod parse_exponent_tests {
    use super::*;

    #[test]
    fn test_parse_exponent_valid_positive_e() {
        let input = "e3";
        let expected_result = Ok(("", Some(3)));
        assert_eq!(parse_exponent(input), expected_result);
    }

    #[test]
    fn test_parse_exponent_valid_negative_e() {
        let input = "e-4";
        let expected_result = Ok(("", Some(-4)));
        assert_eq!(parse_exponent(input), expected_result);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_parse_exponent_valid_positive_E() {
        let input = "E2";
        let expected_result = Ok(("", Some(2)));
        assert_eq!(parse_exponent(input), expected_result);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_parse_exponent_valid_negative_E() {
        let input = "E-5";
        let expected_result = Ok(("", Some(-5)));
        assert_eq!(parse_exponent(input), expected_result);
    }

    #[test]
    fn test_parse_exponent_valid_star() {
        let input = "*10^6";
        let expected_result = Ok(("", Some(6)));
        assert_eq!(parse_exponent(input), expected_result);
    }

    #[test]
    fn test_parse_exponent_valid_no_exponent() {
        let input = "123.456";
        let expected_result = Ok(("123.456", None));
        assert_eq!(parse_exponent(input), expected_result);
    }

    #[test]
    fn test_parse_exponent_invalid() {
        let input = "abc";
        
        match parse_exponent(input) {
            Ok((input,exponent)) => {
                assert_eq!(input,"abc");
                assert!(exponent.is_none())
            },
            Err(_) => assert!(true),
        }
    }
}

#[cfg(test)]
mod parse_standardform_tests {
    use super::*;
    
    #[test]
    fn test_parse_standard_form_positive_exponent() {
        let input = "1.23e3"; // Example input: 1230.0
        match parse_standard_form(input) {
            Ok((_, standard_form)) => {
                assert_eq!(standard_form.mantissa(), &1.23);
                assert_eq!(standard_form.exponent(), &3);
            },
            Err(e) => panic!("Parsing error: {:?}", e),
        }
    }
    
    #[test]
    fn test_parse_standard_form_negative_exponent() {
        let input = "2.5*10^-2"; // Example input: 0.025
        match parse_standard_form(input) {
            Ok((_, standard_form)) => {
                assert_eq!(standard_form.mantissa(), &2.5);
                assert_eq!(standard_form.exponent(), &-2);
            },
            Err(e) => panic!("Parsing error: {:?}", e),
        }
    }
    
    #[test]
    fn test_parse_standard_form_no_exponent() {
        let input = "42"; // No exponent, should be treated as 42.0 * 10^0
        match parse_standard_form(input) {
            Ok((_, standard_form)) => {
                assert_eq!(standard_form.mantissa(), &4.2);
                assert_eq!(standard_form.exponent(), &1);
            },
            Err(e) => panic!("Parsing error: {:?}", e),
        }
    }
    
    #[test]
    fn test_parse_standard_form_star_exponent() {
        let input = "5*10^4"; // Example input: 50000.0
        match parse_standard_form(input) {
            Ok((_, standard_form)) => {
                assert_eq!(standard_form.mantissa(), &5.0);
                assert_eq!(standard_form.exponent(), &4);
            },
            Err(e) => panic!("Parsing error: {:?}", e),
        }
    }
    
    #[test]
    fn test_parse_standard_form_invalid_input() {
        let input = "invalid"; // Invalid input, should result in an error
        match parse_standard_form(input) {
            Ok(_) => panic!("Expected parsing error but got a result."),
            Err(_) => {} // Expected result
        }
    }
}
