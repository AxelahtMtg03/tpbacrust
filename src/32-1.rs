fn xor(tab1:&[i32],tab2:&[i32])->Vec<i32>{
    let mut new = Vec::new();
    for i in 0..tab1.len(){
        if (tab1[i]==1 && tab2[i]==1) || (tab1[i]==0 && tab2[i]==0){
            new.push(0)
        } else {
            new.push(1)
        }
    }
    new
}
fn main(){
    println!("{:?}",xor(&[1, 0, 1, 0, 1, 1, 0, 1], &[0, 1, 1, 1, 0, 1, 0, 0]));
    println!("{:?}",xor(&[1, 1, 0, 1], &[0, 0, 1, 1]));
}