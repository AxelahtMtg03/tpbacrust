fn absolu(num:i32)->i32{
    num.abs()
}
fn mult(num1:i32,num2:i32)->i32{
    let mut nbfinal= 0;
    for _i in 0..absolu(num1){
        nbfinal+=num2
    }
    nbfinal
}
fn main(){
    println!("{:?}",mult(3,5));
    println!("{:?}",mult(-4,-8));
    println!("{:?}",mult(-2,6));
    println!("{:?}",mult(-2,0));
}