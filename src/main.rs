#![feature(test)]

extern crate test;

use std::fmt::{self, Display, Formatter};
use std::num::Wrapping;
use std::thread;
use std::time::*;

const FRAME_COUNT: usize = 600;

fn main() {
    let target_frame_time = Duration::new(0, 16_666_667);
    
    let mut times = Vec::with_capacity(FRAME_COUNT);
    
    let mut frame_start = Instant::now();
    for _ in 0..FRAME_COUNT {
        // Do a fixed amount of work each frame.
        let mut prev = Wrapping(0);
        let mut current = Wrapping(1);
        for _ in 0..50_000 {
            let temp = current;
            current = prev + current;
            prev = temp;
        }
        test::black_box(current);
        
        let duration = frame_start.elapsed();
        times.push(duration);
        
        frame_start += target_frame_time;
        
        while Instant::now() < frame_start {
            thread::sleep(Duration::new(0, 0));
        }
    }
    
    let mut min = times[0];
    let mut max = times[0];
    let mut total = Duration::new(0, 0);
    let mut long_frames = 0;
    
    for time in times {
        total += time;
        if time < min { min = time; }
        if time > max { max = time; }
        if time > target_frame_time { long_frames += 1; }
    }
    
    println!("min: {}", PrettyDuration(min));
    println!("max: {}", PrettyDuration(max));
    println!("avg: {}", PrettyDuration(total / FRAME_COUNT as u32));
    println!("long frames: {}", long_frames);
}

struct PrettyDuration(Duration);

impl Display for PrettyDuration {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        let nanos_total = self.0.subsec_nanos();
        let millis = nanos_total / 1_000_000;
        let micros = (nanos_total % 1_000_000) / 1_000;
    
        write!(formatter, "{}.{}ms", millis, micros)
    }
}