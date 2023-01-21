use rust_utils_macro::New;

pub struct HexValue {
    value: u8,
}

impl HexValue {
    pub fn new(value: u8) -> HexValue {
        if value > 0xF { panic!("Invalid value for an octave : {value}"); }
        HexValue { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

enum HexDigit {
    First,
    Second,
}

#[derive(New, Default)]
pub struct VelocityField {
    pub value: Option<u8>,
}

impl VelocityField {
    fn set_digit_hex(&mut self, digit: HexDigit, value: HexValue) {
        let (mask, value) = match digit {
            HexDigit::First => (0x0F, value.value() << 4),
            HexDigit::Second => (0xF0, value.value()),
        };

        let mut current_value = self.value.unwrap_or(0);
        current_value &= mask;
        current_value |= value;

        self.value = Some(current_value);
    }

    pub fn set_first_digit_hex(&mut self, value: HexValue) {
        self.set_digit_hex(HexDigit::First, value);
    }

    pub fn set_second_digit_hex(&mut self, value: HexValue) {
        self.set_digit_hex(HexDigit::Second, value);
    }

    pub fn clear(&mut self) {
        self.value = None;
    }
}


