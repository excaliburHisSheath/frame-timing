#![feature(test)]

extern crate test;

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
        for _ in 0..10_000 {
            let temp = current;
            current = prev + current;
            prev = temp;
        }
        test::black_box(current);
        
        let duration = frame_start.elapsed();
        times.push(duration);
        
        // let nanos_total = duration.subsec_nanos();
        // let millis = nanos_total / 1_000_000;
        // let micros = (nanos_total % 1_000_000) / 1_000;
        
        frame_start += target_frame_time;
        
        while Instant::now() < frame_start {
            thread::sleep(Duration::new(0, 0));
        }
    }
    
    let mut min = times[0];
    let mut max = times[0];
    let total = Duration::new(0, 0);
    
    for time in times {
        total += time;
        if time < min { min = time; }
        if time > max { max = time; }
    }
    
    println!("min: {}", min);
    println!("max: {}", max);
    println!("avg: {}", total / FRAME_COUNT);
}
