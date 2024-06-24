fn moyenne(tab:&[f32])->f32{
    let mut fin = 0.0;
    for number in tab{
        fin+=number;
    }
    fin/tab.len() as f32
}

fn main(){
    println!("{:?}",moyenne(&[1.0]));
    println!("{:?}",moyenne(&[1.0,2.0]));
    println!("{:?}",moyenne(&[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0]));
}