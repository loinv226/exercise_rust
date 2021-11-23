pub mod linear;
pub mod fibonacci;

use std::time::{Instant, Duration};
use std::cmp::Eq;
use std::fmt::{Formatter, Debug, Result};

pub struct Algorithm<R> {
    name: String,
    valid: bool,
    time: Duration,
    excute: Box<dyn Fn() -> R>,
    expect: R
}

impl<R> Debug for Algorithm<R>
where R: Eq {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Member")
         .field("name", &self.name)
         .field("valid", &self.valid)
         .field("time", &self.time)
         .finish()
    }
}

impl<R> Algorithm<R>
where R: Eq {
    pub fn init<F>(name: String, excute: F, expect: R) -> Algorithm<R> where F: Fn() -> R + 'static {
        Algorithm {
            name,
            valid: false,
            time: Duration::from_nanos(0),
            excute: Box::new(excute),
            expect
        }
    }

    pub fn run(&mut self) {
        let now = Instant::now();
        let result = (self.excute)();
        let dur = now.elapsed();
        println!("Mem: {:?} - dur: {:.2?}", self.name, dur);
        self.time = dur;
        if self.expect == result {
            self.valid = true;
        }
    }
}