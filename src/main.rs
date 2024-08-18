// Metodo da Congruencia Linear
fn mcl(a: u64, c: u64, m: u64, x0: u64) -> u64{
    return ((a * x0) + c) % m;
}
   
fn main(){
    let (a, c, m, mut x0) : (u64, u64, u64, u64) = (101, 1, 1000, 1);
    let mut n_count : i32 = 1000;
    while n_count > 0 {        
        x0 = mcl(a, c, m, x0);        
        println!("{}", (x0 as f64 / 1000.0));
        n_count -= 1;
    }
}