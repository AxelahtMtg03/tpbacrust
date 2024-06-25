fn doublon(tab:&[i32])->bool{
    if tab.len()==1 || tab.len()==0{
        return false
    }
    for i in 0..tab.len()-1{
        if tab[i]==tab[i+1]{
            return true
        }
    }
    false
}
fn main(){
    println!("{:?}",doublon(&[]));
    println!("{:?}",doublon(&[1]));
    println!("{:?}",doublon(&[1, 2, 4, 6, 6]));
    println!("{:?}",doublon(&[2, 5, 7, 7, 7, 9]));
    println!("{:?}",doublon(&[0, 2, 3]));
}