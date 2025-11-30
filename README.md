# readable_time #

A lightweight and non crate dependent rust library for date and time.
This library is for simple use cases and doesnot provide advanced time functionality.
I created it cause 'chrono' was too heavy for my use case.

## Quick Start ## 

**Add from <u>crates.io</u>**:

```bash
cargo add readable_time    
```

**OR**

```bash
cargo add --git https://github.com/BayonetArch/readable_time    
```

### Basic usage ###

 ```rust
 let rt:ReadableTime  = get_readable_time()?;
 // You  can use rt.month,rt.year,rt.day,rt.hour_24,rt.hour_12,......
 // Or specific method to get formatted_time like 'rt.get_timef()' 'rt.get_ptimef',....

 println!("{}",;rt.get_timef());             // OUTPUT:  2025-01-01 03:04:05
 println!("{}",;rt.get_ptimef()?);           // OUTPUT:  Mon Jan 15 2024 03:45 PM
 println!("{}",;rt.get_extended_ptimef()?);  // OUTPUT:  Sun Nov 30 07:14:00 +0545 2025
 ```

 If you want to get the time period for 'hour_24' you can use :
 ```rust
 ReadableTime::get_time_period(hour_24)?; // "AM" or "PM"
 ````
### NOTE ###
This library works on unix systems and is not  guaranted to work on windows(who gives an fuck anyway right?)

