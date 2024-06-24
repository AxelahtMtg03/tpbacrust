fn recherche(target:i32,tab:&[i32])->Option<usize>{
    let mut ind = tab.len()+1;
    for _i in 0..tab.len(){
        if target==tab[_i]{
            ind=_i;
        }
    }
    if ind==tab.len()+1{
        return None
    }
    Some(ind)
} 
fn main(){
 println!("{:?}", recherche(1, &[2, 3, 4]));
 println!("{:?}", recherche(1, &[10, 12, 1, 56]));
 println!("{:?}",recherche(1, &[1, 0, 42, 7]));
 println!("{:?}",recherche(1, &[1, 50, 1]));
 println!("{:?}",recherche(1, &[8, 1, 10, 1, 7, 1, 8]));
}