#[derive(Debug, Clone, PartialEq)]
pub struct HitSound {
    normal: bool,
    whistle: bool,
    finish: bool,
    clap: bool,
}
impl std::str::FromStr for HitSound {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i64 = s
            .parse()
            .expect("at num assignment and i64 parsing in HitSound parsing");
        let mut bits = [false; 8];
        if num > 2_i64.pow(4) - 1 {
            return Result::Err(format!("Invalid HitSound: {}", s));
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
            return Result::Err(format!("Logic error in HitSound creation"));
        }
        Ok(Self {
            normal: bits[0],
            whistle: bits[1],
            finish: bits[2],
            clap: bits[3],
        })
    }
}
