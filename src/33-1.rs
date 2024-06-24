fn inverse(chaine:&str)->String{
    let mut new = String::new();
    for caractere in chaine.chars().rev(){
        new.push(caractere)
    }
    new
}
fn main(){
    println!("{:?}",inverse("bac"));
    println!("{:?}",inverse("moi"));
    println!("{:?}",inverse("nsi"));
}