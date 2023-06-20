use rand::Rng;
use std::fs::File;
use std::io::Write;
static N: usize = 1000;       // number of attempts for determining probability
static F: usize = 10;         // flip max multiplier of k
pub fn main() {
    let mut w = File::create("./result.txt").unwrap();
    let mut ks = Vec::<(usize, usize)>::new();
    ks.push((11, 1));       // k bits, step size for flips
    ks.push((101, 10));
    ks.push((1001, 100));
    ks.push((10001, 1000));
    ks.push((100001, 10000));
    writeln!(&mut w, "k\tflips\tprobability");
    for (k, step) in ks.into_iter() {
        for s in ((k/2)..(F*(k-1))).step_by(step) {
            let mut rng = rand::thread_rng();
            let mut retain: usize = 0;
            for _n in 1..=N {
                let mut v = Vec::<bool>::new();
                for _i in 0..k {
                    v.push(true);           // all true to begin with
                }           
                for _i in 1..=s {
                    let r = rng.gen_range(0..k);
                    v[r] = !v[r];           // flip a random bit
                }
                let c = v.into_iter().filter(|&b| b).count();
                if c > k / 2 {
                    retain = retain + 1;    // more than half retained
                }
            }
            writeln!(&mut w, "{}\t{}\t{}", k, s, retain as f32 / N as f32);  
        }
    }
}
