fn moyenne(tab:&[i32])->Option<f32>{
    if tab.len()==0{
        return None
    }
    let mut moy = 0;
    for _i in 0..tab.len(){
        moy+=tab[_i]
    
    }
    Some(moy as f32/tab.len() as f32)
}
fn main(){
    println!("{:?}",moyenne(&[5,3,8]));
    println!("{:?}",moyenne(&[1,2,3,4,5,6,7,8,9,10]));
    println!("{:?}",moyenne(&[]));
}