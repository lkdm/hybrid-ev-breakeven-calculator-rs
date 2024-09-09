use std::fmt;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money(u32);

impl Money {
    /// Parse a string into Money, handling conversion to cents.
    fn parse_string(s: &str) -> u32 {
        let mut parts = s.split('.');

        let integer_part = parts.next().unwrap_or("0");
        let fractional_part = parts.next().unwrap_or("00");

        if parts.next().is_some() {
            // More than one decimal point
            return 0;
        }

        let integer_value: u32 = integer_part.parse().unwrap_or(0);
        let fractional_value: u32 = fractional_part.chars().take(2).collect::<String>().parse().unwrap_or(0);

        integer_value * 100 + fractional_value
    }
}

impl From<String> for Money {
    fn from(s: String) -> Self {
        // Clean up the string and handle invalid cases
        let cleaned_str = s.chars()
            .filter(|&c| c.is_digit(10) || c == '.')
            .collect::<String>();

        let parsed_value = Money::parse_string(&cleaned_str);
        Money(parsed_value)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.0 / 100;
        let cents = self.0 % 100;
        write!(f, "${:}.{:02}", dollars, cents)
    }
}
