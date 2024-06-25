fn recherche(tab:&[i32],target:i32)->Option<usize>{
    let mut ind = tab.len()+1;
    for _i in 0..tab.len(){
        if tab[_i]==target{
            ind=_i
        }
    }
    if ind == tab.len()+1{
        return None
    }
    Some(ind)
}
fn main(){
    println!("{:?}",recherche(&[2, 3, 4, 5, 6],5));
    println!("{:?}",recherche(&[2,3,4,6,7,8],5));
}