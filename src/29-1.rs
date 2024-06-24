fn moyenne(tab:&[(f32,f32)])->f32{
    let mut nb=0.0;
    let mut coef=0.0;
    for i in tab{
        nb+=i.0*i.1;
        coef+=i.1;
    }
    nb/coef
}
fn main(){
    println!("{:?}",moyenne(&[(15.0,2.0),(9.0,1.0),(12.0,3.0)]));
}