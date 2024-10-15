#[derive(Default, Debug, Copy, Clone)]
pub enum Unit {
    Seconds,
    Minutes,
    Hours,
    Days,
    #[default]
    Unknown,
}

trait ToUnit {
    fn to_unit(&self) -> Unit;
}

impl ToUnit for &str {
    fn to_unit(&self) -> Unit {
        self.to_lowercase()
            .chars()
            .next()
            .map_or(Unit::Unknown, |c| match c {
                's' => Unit::Seconds,
                'm' => Unit::Minutes,
                'h' => Unit::Hours,
                'd' => Unit::Days,
                _ => Unit::Unknown,
            })
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds => write!(f, "seconds"),
            Self::Minutes => write!(f, "minutes"),
            Self::Hours => write!(f, "hours"),
            Self::Days => write!(f, "days"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Default)]
pub struct Timestamp {
    seconds: u64,
    minutes: u64,
    hours: u64,
    days: u64,
    unit: Unit,
}

impl Timestamp {
    pub const fn seconds(&self) -> u64 {
        self.seconds
    }

    pub const fn minutes(&self) -> u64 {
        self.minutes
    }

    pub const fn hours(&self) -> u64 {
        self.hours
    }

    pub const fn days(&self) -> u64 {
        self.days
    }

    pub const fn unit(&self) -> Unit {
        self.unit
    }
}

pub fn parse_timestamp(timestamp: &str) -> Result<Timestamp, std::num::ParseIntError> {
    let number_str = timestamp
        .chars()
        .map(|c| if c.is_numeric() { c } else { ' ' })
        .collect::<String>()
        .trim()
        .to_owned();

    let number = number_str.parse::<u64>()?;
    let unit = timestamp.replace(&number.to_string(), "").trim().to_unit();

    match unit {
        Unit::Seconds => Ok(Timestamp {
            seconds: number,
            unit,
            ..Default::default()
        }),
        Unit::Minutes => Ok(Timestamp {
            seconds: number * 60,
            minutes: number,
            unit,
            ..Default::default()
        }),
        Unit::Hours => Ok(Timestamp {
            seconds: number * 60 * 60,
            minutes: number * 60,
            hours: number,
            days: 0,
            unit,
        }),
        Unit::Days => Ok(Timestamp {
            seconds: number * 60 * 60 * 24,
            minutes: number * 60,
            hours: number * 60,
            days: number,
            unit,
        }),
        Unit::Unknown => Ok(Timestamp {
            unit,
            ..Default::default()
        }),
    }
}
