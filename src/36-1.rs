fn occ(caractere:char,chaine:&str)->i32{
    let mut ind = 0;
    for lettre in chaine.chars(){
        if lettre==caractere{
            ind+=1;
        }
    }
    ind
}
fn main(){
    println!("{:?}",occ('e', "sciences"));
    println!("{:?}",occ('i',"mississippi"));
    println!("{:?}",occ('a',"mississippi"));
}