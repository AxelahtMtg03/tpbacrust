fn moyenne(tab:&[f64])->f64{
    let mut nb=0.0;
    for &i in tab{
        nb+=i
    }
    nb / (tab.len() as f64) // Ensure the division is done with f64
}
fn main(){
    println!("{:?}",moyenne(&[1.0]));
    println!("{:?}",moyenne(&[1.0,2.0,4.0]));
}