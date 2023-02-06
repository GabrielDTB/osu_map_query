#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Type {
    object_type: ObjectType, // 0 circle, 1 slider, 3 spinner
    new_combo: bool,         // 2
    color_skip: u8,          // 4-6 -- Actually a 3 bit uint
                             // 7 Mania hold
}
impl std::str::FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i64 = s
            .parse()
            .expect("at num assignment and i64 parsing in Type parsing");
        let mut bits = [false; 8];
        if num > 2_i64.pow(8) - 1 {
            return Result::Err(format!("Invalid Type"));
        }
        if num > 2_i64.pow(7) - 1 {
            bits[7] = true;
            num -= 2_i64.pow(7);
        }
        if num > 2_i64.pow(6) - 1 {
            bits[6] = true;
            num -= 2_i64.pow(6);
        }
        if num > 2_i64.pow(5) - 1 {
            bits[5] = true;
            num -= 2_i64.pow(5);
        }
        if num > 2_i64.pow(4) - 1 {
            bits[4] = true;
            num -= 2_i64.pow(4);
        }
        if num > 2_i64.pow(3) - 1 {
            bits[3] = true;
            num -= 2_i64.pow(3);
        }
        if num > 2_i64.pow(2) - 1 {
            bits[2] = true;
            num -= 2_i64.pow(2);
        }
        if num > 2_i64.pow(1) - 1 {
            bits[1] = true;
            num -= 2_i64.pow(1);
        }
        if num > 2_i64.pow(0) - 1 {
            bits[0] = true;
            num -= 2_i64.pow(0);
        }
        if num > 0 {
            return Result::Err(format!("Logic error in Type creation"));
        }
        let mut color_skip = 0;
        if bits[4] {
            color_skip += 2_u8.pow(2);
        }
        if bits[5] {
            color_skip += 2_u8.pow(1);
        }
        if bits[6] {
            color_skip += 2_u8.pow(0);
        }
        let object_type = match (bits[0], bits[1], bits[3]) {
            (true, false, false) => ObjectType::Circle,
            (false, true, false) => ObjectType::Slider,
            (false, false, true) => ObjectType::Spinner,
            _ => {
                return Result::Err(format!(
                    "Invalid object type: {:?}",
                    (bits[0], bits[1], bits[3])
                ))
            }
        };
        Ok(Self {
            object_type,
            new_combo: bits[2],
            color_skip,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
}
