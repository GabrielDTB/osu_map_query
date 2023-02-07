// Screen size: 640x480 osu px
// Play area: 510x385 osu px
// Center of playfield: 256x192 osu px

extern crate num;

use num::rational::Ratio;

macro_rules! ratio {
    ($numer:expr) => {
        num::rational::Ratio::from_integer($numer)
    };
    ($numer:expr, $denom:expr) => {
        num::rational::Ratio::new($numer, $denom)
    };
}

pub fn cs_to_osu_px(cs: Ratio<i32>) -> Ratio<i32> {
    ratio![544, 10] - ratio![448, 100] * cs
}

pub fn ar_to_ms(ar: Ratio<i32>) -> i32 {
    let scalar = if ar < ratio![5] {
        ratio![120]
    } else {
        ratio![150]
    };
    (ratio![1200] + (scalar * (ratio![5] - ar))).to_integer()
}

pub fn ar_to_hr(ar: Ratio<i32>) -> Ratio<i32> {
    let hr_ar = ar * ratio![14, 10];
    if hr_ar < ratio![10] {
        hr_ar
    } else {
        ratio![10]
    }
}

pub fn ar_to_ez(ar: Ratio<i32>) -> Ratio<i32> {
    ar * ratio![1, 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cs_to_osu_px() {
        let correct = [
            (ratio![0], ratio![544, 10]),
            (ratio![1], ratio![4992, 100]),
            (ratio![9], ratio![1408, 100]),
            (ratio![10], ratio![96, 10]),
        ];
        let function = cs_to_osu_px;
        for pair in correct {
            assert_eq!(function(pair.0), pair.1);
        }
    }
    #[test]
    fn test_ar_to_ms() {
        let correct = [
            (ratio![10], 450),
            (ratio![98, 10], 480),
            (ratio![9], 600),
            (ratio![84, 10], 690),
            (ratio![8], 750),
            (ratio![7], 900),
            (ratio![6], 1050),
            (ratio![56, 10], 1110),
            (ratio![5], 1200),
            (ratio![42, 10], 1296),
            (ratio![4], 1320),
            (ratio![3], 1440),
            (ratio![28, 10], 1464),
            (ratio![2], 1560),
            (ratio![14, 10], 1632),
            (ratio![1], 1680),
            (ratio![0], 1800),
        ];
        let function = ar_to_ms;
        for pair in correct {
            assert_eq!(function(pair.0), pair.1);
        }
    }
    #[test]
    fn test_ar_to_hr() {
        let correct = [
            (ratio![10], ratio![10]),
            (ratio![9], ratio![10]),
            (ratio![8], ratio![10]),
            (ratio![7], ratio![98, 10]),
            (ratio![6], ratio![84, 10]),
            (ratio![5], ratio![7]),
            (ratio![4], ratio![56, 10]),
            (ratio![3], ratio![42, 10]),
            (ratio![2], ratio![28, 10]),
            (ratio![1], ratio![14, 10]),
            (ratio![0], ratio![0]),
        ];
        let function = ar_to_hr;
        for pair in correct {
            assert_eq!(function(pair.0), pair.1);
        }
    }
    #[test]
    fn test_ar_to_ez() {
        let correct = [
            (ratio![10], ratio![5]),
            (ratio![9], ratio![9, 2]),
            (ratio![8], ratio![8, 2]),
            (ratio![7], ratio![7, 2]),
            (ratio![6], ratio![6, 2]),
            (ratio![5], ratio![5, 2]),
            (ratio![4], ratio![4, 2]),
            (ratio![3], ratio![3, 2]),
            (ratio![2], ratio![2, 2]),
            (ratio![1], ratio![1, 2]),
            (ratio![0], ratio![0]),
        ];
        let function = ar_to_ez;
        for pair in correct {
            assert_eq!(function(pair.0), pair.1);
        }
    }
}
