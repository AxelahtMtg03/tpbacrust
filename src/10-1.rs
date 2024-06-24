fn moyenne(tab:&[(f64,f64)])->Option<f64>{
    let mut nb = 0.0;
    let mut coef=0.0;
    for i in 0..tab.len(){
        nb += tab[i].0*tab[i].1;
        coef += tab[i].1;
    }
    if coef==0.0{
        None
    } else {
        Some(nb/coef)
    }
}
fn main() {
    println!("{:?}", moyenne(&[(8.0, 2.0), (12.0, 0.0), (13.5, 1.0), (5.0, 0.5)]));
    println!("{:?}", moyenne(&[(3.0, 0.0), (5.0, 0.0)]));
}
