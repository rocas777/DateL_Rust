# DateL_Rust

Simple class with CRUD functions and formated output, changeable in runtime
Handles both date and hours,minute;  
Whenever a date input is illegal the change is not performed(todo: return a boolean to inform if the change took effect) 

## Functions  

### Initiate
To create an "instance" of the class call the funcion new of DateL(DateL::new()) that returns a DateL "class".  
Next call one of the following functions to fill the struct with the data:
 * **set_only_hours**(&mut self, hour: u8, minute: u8)
 * **set_only_date**(&mut self,mday: u8,month: u8,year: i32)
 * **set_full_date**(&mut self,mday: u8,month: u8,year: i32,hour: u8, minute: u8)

### Output Function
 There are three possible outputs(specific date formats(order) and separators can be changed,see **Configuration Functions** section), considering the date class is initialized with the date of 05 of July of 2020, 17 hours and 12 minutes, the possible outputs are:
 * 05-07-2020 17:12
 * 05-07-2020
 * 17:12
 
 Such outpus are, respectively, the return of the following functions:
 * **full_output**      -> String
 * **date_only_output**   -> String
 * **hours_only_output**  -> String

### Configuration Functions
Output Functions use one of three formats:
 * ddmmyy
 * yymmdd
 * mmddyy

By default the date output is in the format **ddmmyy**, where the separator used is '-'.  
The default output is -> dd-mm-yyyy  
Both the separator and the format can be changed using the following functions:  

* **set_dateformat**(DateFormat dateFormat)
* **set_separator**(char separator)


### Set Functions
* **set_year**(i32 )
* **set_month**(u8 )
* **set_mday**(u8 ) (day of the month)
* **set_hour**(u8 )
* **set_minute**(u8 )

### Get Functions
* u32 **get_year()**
* u8 **get_month()**
* u8 **get_mday()** (return day of the month)
* u16 **get_yday()** (return day of the year, not fully implemented)
* u8 **get_hour()**
* u8 **get_minute()**


### Implementation details
It uses a unix time schema, with the initial date being 01-01-1970 00:00

### Dependencies
The class depends o the following libraries:
* chrono 
add lines bellow to Cargo.toml:
[dependencies]
chrono = "0.4"
