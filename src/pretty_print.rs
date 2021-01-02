use crate::utils::Schedule;
use colorful::{Colorful, RGB};

impl Schedule {
    pub fn print(&self) {
        let s = "█";
        let colors = vec![RGB::new(154, 205, 50), RGB::new(255, 215, 0)];

        for core in self.cores() {
            for (idx, task) in core.timeline().iter().enumerate() {
                let length = (task.length() as f64 / core.working_time() as f64 * 180.0) as usize;
                let (r, g, b) = colors.get(idx % 2).unwrap().unpack();
                print!("{}", s.repeat(length).rgb(r, g, b));
            }
            print!("\n");
        }
    }
}
