fn recherche(tab:&[i32])->Option<usize> {
    let mut min = tab[0];
    let mut ind = 0;
    for i in 1..tab.len(){
        if tab[i]<min{
            min=tab[i];
            ind=i;
        }
    }
    Some(ind)
}

fn main(){
    println!("{:?}",recherche(&[5]));
    println!("{:?}",recherche(&[2, 4, 1]));
    println!("{:?}",recherche(&[5, 3, 2, 2, 4]));
    println!("{:?}",recherche(&[-1, -2, -3, -3]));
}