fn recherche(nb:i32,tab:&[i32])->Option<i32>{
 for i in 0..tab.len(){
    if tab[i]==nb{
        return Some(i as i32);
    }
 }
 None
}
fn main(){
    println!("{:?}",recherche(1, &[2, 3, 4]));
    println!("{:?}",recherche(1, &[10, 12, 1, 56]));
    println!("{:?}",recherche(50, &[1, 50, 1]));
    println!("{:?}",recherche(15, &[8, 9, 10, 15]));
}