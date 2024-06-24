fn mult(num1:i32,num2:i32)->i32{
    let mut nb = 0;
    for _i in 0..num1.abs(){
        nb+=num2
    }
    nb
}
fn main(){
    println!("{:?}",mult(3, 5));
    println!("{:?}",mult(-4, -8));
    println!("{:?}",mult(-2, 6));
    println!("{:?}",mult(-2, 0));
}