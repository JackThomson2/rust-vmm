use std::io::Error;

use eyre::eyre;

pub fn get_os_error() -> String {
    if let Some(error) = Error::last_os_error().raw_os_error() {
        format!("OS Error: {error:?}")
    } else {
        format!("Cant find os error")
    }
}

#[macro_export]
macro_rules! check_libc  {
    ($x:expr) => {
        if $x < 0 {
            return Err(eyre!("Error calling libc. Code: {}, OS Error {:?}", $x, get_os_error()));
        }
    };

    ($x:expr, $y:expr) => {
        if $x < 0 {
            return Err(eyre!("Error calling libc, on : {:?}. Status was: {}. OS Error: {:?}", $y, $x, get_os_error()));
        }
    }
}

#[macro_export]
macro_rules! check_libc_no_print {
    ($x:expr) => {
        if $x < 0 {
            return Err(eyre!("Error calling libc."));
        }
    };

    ($x:expr, $y:expr) => {
        if $x < 0 {
            return Err(eyre!("Error calling libc, on : {:?}.", $y));
        }
    }
}


