fn rind(nb:i32,tab:&[i32])->(Vec<i32>, Vec<i32>, Vec<i32>){
    let mut sup = Vec::new();
    let mut eg = Vec::new();
    let mut inf = Vec::new();
    for &value in tab {
        if value == nb {
            eg.push(value);
        } else if value < nb {
            inf.push(value);
        } else {
            sup.push(value);
        }
    }
(inf,eg,sup)
    }

fn main(){
    println!("{:?}",rind(3, &[1, 3, 4, 2, 4, 6, 3, 0]));
    println!("{:?}",rind(3, &[1, 4, 2, 4, 6, 0]));
    println!("{:?}",rind(3, &[1, 1, 1, 1]));
    println!("{:?}",rind(3, &[]));
}