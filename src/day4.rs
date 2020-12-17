use crate::Problem;
use std::collections::HashMap;

pub struct Solution(Vec<HashMap<PassportKey, &'static str>>);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum PassportKey {
    /// Birth Year
    BYR,
    /// Issue Year
    IYR,
    /// Expiration Year
    EYR,
    /// Height
    HGT,
    /// Hair Color
    HCL,
    /// Eye Color
    ECL,
    /// Passport ID
    PID,
    /// Country ID
    CID,
}

impl PassportKey {
    fn required(&self) -> bool {
        match self {
            PassportKey::BYR => true,
            PassportKey::IYR => true,
            PassportKey::EYR => true,
            PassportKey::HGT => true,
            PassportKey::HCL => true,
            PassportKey::ECL => true,
            PassportKey::PID => true,
            PassportKey::CID => false,
        }
    }

    fn parse(s: &str) -> Result<PassportKey, String> {
        use PassportKey::*;
        match &s.to_uppercase()[..] {
            "BYR" => Ok(BYR),
            "IYR" => Ok(IYR),
            "EYR" => Ok(EYR),
            "HGT" => Ok(HGT),
            "HCL" => Ok(HCL),
            "ECL" => Ok(ECL),
            "PID" => Ok(PID),
            "CID" => Ok(CID),
            _ => Err(format!("unexpected key {}", s))
        }
    }

    fn all() -> &'static [PassportKey] {
        use PassportKey::*;
        &[BYR, IYR, EYR, HGT, HCL, ECL, PID, CID][..]
    }
}

impl Solution {
    pub fn init() -> Box<dyn Problem> {
        let mut list = Vec::new();
        let mut passport = HashMap::new();

        for line in include_str!("day4.txt").lines() {
            if line.is_empty() && !passport.is_empty() {
                list.push(passport);
                passport = HashMap::new();
                continue;
            }

            for pair in line.split_whitespace() {
                let mut split = pair.split(':');
                let key = PassportKey::parse(split.next().unwrap()).unwrap();
                let value = split.next().unwrap();
                passport.insert(key, value);
            }
        }

        if !passport.is_empty() {
            list.push(passport);
        }

        Box::new(Solution(list))
    }
}

fn validate_year(value: &'static str, min: u16, max: u16) -> Result<(), Box<dyn std::error::Error>> {
    if value.len() != 4 {
        return Err(format!("Not four digit year {:?}", value).into());
    }

    let value = value.parse::<u16>()?;

    if (min..=max).contains(&value) {
        Ok(())
    } else {
        Err(format!("Year out of range {:?}", value).into())
    }
}

fn validate(key: PassportKey, value: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    use PassportKey::*;
    match key {
        BYR => validate_year(value, 1920, 2002),
        IYR => validate_year(value, 2010, 2020),
        EYR => validate_year(value, 2020, 2030),
        HGT => {
            if value.ends_with("cm") {
                let value = value.trim_end_matches("cm").parse::<u16>()?;
                if (150..=193).contains(&value) {
                    Ok(())
                } else {
                    Err(format!("Height out of range {:?}", value).into())
                }
            } else if value.ends_with("in") {
                let value = value.trim_end_matches("in").parse::<u16>()?;
                if (59..=76).contains(&value) {
                    Ok(())
                } else {
                    Err(format!("Height out of range {:?}", value).into())
                }
            } else {
                Err(format!("unexpected Height suffix {:?}", value).into())
            }
        }
        HCL => {
            if value.starts_with('#') && value.len() == 7 {
                u64::from_str_radix(value.trim_start_matches('#'), 16)
                    .map(|_| ())
                    .map_err(|e| e.into())
            } else {
                Err(format!("Invalid hair color {:?}", value).into())
            }
        }
        ECL => match value {
            "amb" => Ok(()),
            "blu" => Ok(()),
            "brn" => Ok(()),
            "gry" => Ok(()),
            "grn" => Ok(()),
            "hzl" => Ok(()),
            "oth" => Ok(()),
            _ => Err(format!("Invalid eye color {:?}", value).into())
        },
        PID => {
            if value.len() == 9 && value.chars().all(|c| c.is_numeric()) {
                Ok(())
            } else {
                Err(format!("Invalid Passport ID {:?}", value).into())
            }
        }
        _ => Err(format!("unknown key {:?}", key).into())
    }
}

impl Problem for Solution {
    fn part1(&self) -> i64 {
        self.0.iter()
            .filter(|passport| PassportKey::all()
                .iter()
                .filter(|k| k.required())
                .all(|k| passport.contains_key(k))
            )
            .count()
            as i64
    }

    fn part2(&self) -> i64 {
        self.0.iter()
            .filter(|passport| PassportKey::all()
                .iter()
                .filter(|k| k.required())
                .all(|k| passport.get(k)
                    .map(|value| validate(*k, value).is_ok())
                    .unwrap_or(false)
                )
            )
            .count()
            as i64
    }
}

#[test]
fn test_part1() {
    let solution = Solution::init();
    assert_eq!(solution.part1(), 233)
}

#[test]
fn test_part2() {
    let solution = Solution::init();
    assert_eq!(solution.part2(), 111)
}
