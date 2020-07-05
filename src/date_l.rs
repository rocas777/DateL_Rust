use chrono::{NaiveDate, NaiveDateTime, Timelike, Datelike};

pub(crate) struct DateL {
    minute: u8,
    hour: u8,
    y_day: u16,
    m_day: u8,
    month: u8,
    year: i32
}

impl DateL {
    pub(crate) fn new() -> DateL {
        DateL {
            minute: 0,
            hour: 0,
            y_day: 0,
            m_day: 0,
            month: 0,
            year: 0
        }
    }

    pub(crate) fn set_only_hours(&mut self, hour: u8, minute: u8){
        self.minute = minute;
        self.hour = hour;
    }

    pub(crate) fn set_only_date(&mut self,mday: u8,month: u8,year: i32){
        self.m_day = mday;
        self.month = month;
        self.year = year;
    }

    pub(crate) fn set_full_date(&mut self,mday: u8,month: u8,year: i32,hour: u8, minute: u8){
        self.m_day = mday;
        self.month = month;
        self.year = year;
        self.minute = minute;
        self.hour = hour;
    }

    pub(crate) fn get_minute(&self) -> u8{
        return self.minute;
    }
    pub(crate) fn set_minute(&mut self, minute: u8){
        self.minute = minute;
    }

    pub(crate) fn get_hour(&self) -> u8{
        return self.hour;
    }

    pub(crate) fn set_hour(&mut self, hour: u8){
        self.hour = hour;
    }

    pub(crate) fn get_mday(&self) -> u8{
        return self.m_day;
    }

    pub(crate) fn get_yday(&self) -> u16{
        return self.y_day;
    }
    pub(crate) fn set_mday(&mut self, mday: u8){
        let dif = self.m_day-mday;
        self.y_day -= dif as u16;
        self.m_day = mday;
    }

    pub(crate) fn get_month(&self) -> u8{
        return self.month;
    }
    pub(crate) fn set_month(&mut self, month: u8){
        self.month = month;
    }

    pub(crate) fn get_year(&self) -> i32{
        return self.year + 1970;
    }
    pub(crate) fn set_year(&mut self, year: u32){
        self.year = (year - 1970) as i32;
    }

    pub(crate) fn diference(&self, date: DateL) -> DateL {
        let date1: NaiveDateTime = NaiveDate::from_ymd(self.year+1970, self.month as u32, self.m_day as u32).and_hms(self.hour as u32, self.minute as u32, 0);
        let date2: NaiveDateTime = NaiveDate::from_ymd(date.year+1970, date.month as u32, date.m_day as u32).and_hms(date.hour as u32, date.minute as u32, 0);
        let diff = NaiveDateTime::from_timestamp(date1.timestamp()-date2.timestamp(), 0);
        DateL {
            minute: diff.minute() as u8,
            hour: diff.hour() as u8,
            y_day: diff.day0() as u16,
            m_day: diff.day() as u8,
            month: diff.month() as u8,
            year: diff.year()
        }
    }
}