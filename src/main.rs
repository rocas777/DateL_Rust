use crate::date_l::DateFormat;

mod date_l;
fn main() {
    let mut date2 = date_l::DateL::new();
    date2.set_full_date(5,7,2020,15,09);
    date2.set_separator('#');
    date2.set_date_format(DateFormat::ddmmyy);
    println!("{}",date2.date_only_output());
}
