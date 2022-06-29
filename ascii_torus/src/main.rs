
use std::{thread};
use libc::putchar;

fn main() {
    let mut A : f32 = 0.0;
    let mut B : f32 = 0.0;
    print!("\x1b[2J");

    loop {
        let mut z : [f32; 1760] = [0.0; 1760]; 
        let mut b : [char; 1760] = [32 as char; 1760];

        let mut j : f32 = 0.0;
        while j < 6.28 {
            let mut i : f32 = 0.0;
            while i < 6.28 {
                let c : f32 = i.sin();
                let d : f32 = j.cos();
                let e : f32 = A.sin();
                let f : f32 = j.sin();
                let g : f32 = A.cos();

                let h : f32 = d + 2.0;                              // size          
                let depth : f32 = 1.0 / (c * h * e + f * g + 5.0); // distance
                let l : f32 = i.cos();
                let m : f32 = B.cos();
                let n : f32 = B.sin();
                let t : f32 = c * h * g - f * e;

                let x : i32 = (40.0 + 30.0 * depth * (l * h * m - t * n)) as i32;
                let y : i32 = (12.0 + 15.0 * depth * (l * h * n + t * m)) as i32;
                let o : usize = (x + 80 * y) as usize;
                let luminance : usize = (8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as usize;

                if 22 > y && y > 0 && x > 0 && 80 > x && depth > z[o] {
                    z[o] = depth;
                    b[o] = (".,-~:;=!*#$@".as_bytes()[if luminance > 0 {luminance} else {0}]) as char;
                }

                i += 0.02;
            }
            j += 0.07;
        }

        print!("\x1b[H");
        let mut k : usize = 0;
        while k < 1761 {
            unsafe {
                putchar( if (k % 80) != 0 {b[k] as i32} else {10});
            }
            A += 0.00004;
            B += 0.00002;
            k += 1;
        }
        thread::sleep_ms(100);
    }
}
