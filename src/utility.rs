extern crate num;

use num::rational::Ratio;

pub fn cs_to_osu_px(cs: Ratio<i32>) -> Ratio<i32> {
    Ratio::new(544, 10) - Ratio::new(448, 100) * cs
}

pub fn ar_to_ms(ar: Ratio<i32>) -> i32 {
    let scalar = if ar < Ratio::from_integer(5) {
        Ratio::from_integer(120)
    } else {
        Ratio::from_integer(150)
    };
    (Ratio::from_integer(1200) + (scalar * (Ratio::from_integer(5) - ar))).to_integer()
}

pub fn ar_to_hr(ar: Ratio<i32>) -> Ratio<i32> {
    let hr_ar = ar * Ratio::new(14, 10);
    if hr_ar < Ratio::from_integer(10) {
        hr_ar
    } else {
        Ratio::from_integer(10)
    }
}

pub fn ar_to_ez(ar: Ratio<i32>) -> Ratio<i32> {
    ar * Ratio::new(1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cs_to_osu_px() {
        let correct = [
            (Ratio::from_integer(0), Ratio::new(544, 10)),
            (Ratio::from_integer(1), Ratio::new(4992, 100)),
            (Ratio::from_integer(9), Ratio::new(1408, 100)),
            (Ratio::from_integer(10), Ratio::new(96, 10)),
        ];
        for pair in correct {
            assert_eq!(cs_to_osu_px(pair.0), pair.1);
        }
    }
    #[test]
    fn test_ar_to_ms() {
        let correct = [
            (Ratio::from_integer(10), 450),
            (Ratio::new(98, 10), 480),
            (Ratio::from_integer(9), 600),
            (Ratio::new(84, 10), 690),
            (Ratio::from_integer(8), 750),
            (Ratio::from_integer(7), 900),
            (Ratio::from_integer(6), 1050),
            (Ratio::new(56, 10), 1110),
            (Ratio::from_integer(5), 1200),
            (Ratio::new(42, 10), 1296),
            (Ratio::from_integer(4), 1320),
            (Ratio::from_integer(3), 1440),
            (Ratio::new(28, 10), 1464),
            (Ratio::from_integer(2), 1560),
            (Ratio::new(14, 10), 1632),
            (Ratio::from_integer(1), 1680),
            (Ratio::from_integer(0), 1800),
        ];
        for pair in correct {
            assert_eq!(ar_to_ms(pair.0), pair.1);
        }
    }
    #[test]
    fn test_ar_to_hr() {
        let correct = [
            (Ratio::from_integer(10), Ratio::from_integer(10)),
            (Ratio::from_integer(9), Ratio::from_integer(10)),
            (Ratio::from_integer(8), Ratio::from_integer(10)),
            (Ratio::from_integer(7), Ratio::new(98, 10)),
            (Ratio::from_integer(6), Ratio::new(84, 10)),
            (Ratio::from_integer(5), Ratio::from_integer(7)),
            (Ratio::from_integer(4), Ratio::new(56, 10)),
            (Ratio::from_integer(3), Ratio::new(42, 10)),
            (Ratio::from_integer(2), Ratio::new(28, 10)),
            (Ratio::from_integer(1), Ratio::new(14, 10)),
            (Ratio::from_integer(0), Ratio::from_integer(0)),
        ];
        for pair in correct {
            assert_eq!(ar_to_hr(pair.0), pair.1);
        }
    }
}
