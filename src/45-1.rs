fn nbocc(target:i32,tab:&[i32])->i32{
    let mut occ=0;
    if tab.len()==0{
        return 0
    }
    for &number in tab {
        if target==number{
            occ+=1;
        }
    }
    occ
}

fn main(){
    println!("{:?}",nbocc(5, &[]));
    println!("{:?}",nbocc(3, &[1,2,3,4,5,6,7,8,9]));
    println!("{:?}",nbocc(5, &[5,5,5,5,5]));
}