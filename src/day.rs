pub const SUNDAY: u8 = 0b0000_0001;
pub const MONDAY: u8 = 0b0000_0010;
pub const TUESDAY: u8 = 0b0000_0100;
pub const WEDNESDAY: u8 = 0b0000_1000;
pub const THURSDAY: u8 = 0b0001_0000;
pub const FRIDAY: u8 = 0b0010_0000;
pub const SATURDAY: u8 = 0b0100_0000;

pub fn to_day(
    mut sunday: bool,
    mut monday: bool,
    mut tuesday: bool,
    mut wednesday: bool,
    mut thursday: bool,
    mut friday: bool,
    mut saturday: bool,
    weekday: bool,
    weekend: bool,
) -> u8 {
    let mut day = 0;

    if weekday {
        monday = true;
        tuesday = true;
        wednesday = true;
        thursday = true;
        friday = true;
    }

    if weekend {
        sunday = true;
        saturday = true;
    }

    if sunday {
        day |= SUNDAY
    }
    if monday {
        day |= MONDAY
    }
    if tuesday {
        day |= TUESDAY
    }
    if wednesday {
        day |= WEDNESDAY
    }
    if thursday {
        day |= THURSDAY
    }
    if friday {
        day |= FRIDAY
    }
    if saturday {
        day |= SATURDAY
    }

    day
}

pub fn to_string(value: u8) -> String {
    let mut days = vec![];
    if value & SUNDAY == SUNDAY {
        days.push("Sunday");
    }

    if value & MONDAY == MONDAY {
        days.push("Monday");
    }

    if value & TUESDAY == TUESDAY {
        days.push("Tuesday");
    }

    if value & WEDNESDAY == WEDNESDAY {
        days.push("Wednesday");
    }

    if value & THURSDAY == THURSDAY {
        days.push("Thursday");
    }

    if value & FRIDAY == FRIDAY {
        days.push("Friday");
    }

    if value & SATURDAY == SATURDAY {
        days.push("Saturday");
    }

    days.join(" ")
}
