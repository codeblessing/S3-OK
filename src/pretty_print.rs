use crate::utils::Schedule;

use colorful::Colorful;

impl Schedule {
    pub fn print(&self) {
        let s = "█";

        for core in self.cores() {
            for task in core.timeline() {
                let length = task.length() as usize / 10 + 1;
                let hue = (task.length() * 15 % 360) as f32/ 360.0;
                print!("{}", s.repeat(length).hsl(hue, 1.0, 0.5));
            }
            print!("\n");
        }
    }
}
