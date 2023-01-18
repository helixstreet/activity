use std::time::{SystemTime};

pub fn multiply_by_2(num: i32) -> i32 {
    num * 2
}

pub fn print42() -> i32{
    42
}

pub fn print_current_time() -> SystemTime {
     SystemTime::now()
}

#[cfg(test)]
fn print42 () {
    //write test for multiply_by_2

    //write test for print42
assert_eq!(print42(), 42);

    //write test for print_current_time


}
//