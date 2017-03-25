#![feature(test)]

extern crate test;

fn read_slice(buf: &mut [u8]) -> &[u8] {
    return &buf[..]
}

fn read_usize(buf: &mut [u8]) -> usize {
    return buf.len()
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    fn mkarr() -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for i in 0..1024 {
            let q: u32 = i % 256;
            let b: u8 = q as u8;
            buf.push(b);
        }
        buf
    }

    const RUNS: i32 = 1000000;
    #[bench]
    fn bench_usize(b: &mut Bencher) {
        let mut buf = mkarr();
        b.iter(|| {
            for _ in 0..RUNS {
                test::black_box(read_usize(&mut buf));
            }
        })
    }

    #[bench]
    fn bench_slice(b: &mut Bencher) {
        let mut buf = mkarr();
        b.iter(|| {
            for _ in 0..RUNS {
                test::black_box(read_slice(&mut buf));
            }
        })
    }
}
