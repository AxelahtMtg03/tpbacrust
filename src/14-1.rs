use std::collections::HashMap;
fn maxmin(tab:&[i32])->HashMap<String, i32> {
    let mut newash = HashMap::new();
    let mut max= tab[0];
    let mut min= tab[0];
    for &i in tab {
        if i<min {
            min = i;
        } else if i>max{
            max = i;
        }
    }
    newash.insert("maxi:".to_string(),max);
    newash.insert("min:".to_string(),min);
    newash
}

fn main(){
    println!("{:?}",maxmin(&[0, 1, 4, 2, -2, 9, 3, 1, 7, 1]));
    println!("{:?}",maxmin(&[0, 1, 2, 3]));
    println!("{:?}",maxmin(&[3]));
    println!("{:?}",maxmin(&[1, 3, 2, 1, 3]));
    println!("{:?}",maxmin(&[-1, -1, -1, -1, -1]));
}