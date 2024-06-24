fn  nbmot(chaine:&str)->i32{
    let mut counter=1;
    for caractere in chaine.chars(){
        if caractere==' '{
            counter+=1;
        }
    }
    if chaine.ends_with("?") || chaine.ends_with("!"){
        counter-=1
    }
    counter
}
fn main(){
    println!("{:?}",nbmot("Cet exercice est simple."));
    println!("{:?}",nbmot("Le point d exclamation est séparé !"));
    println!("{:?}",nbmot("Combien de mots y a t il dans cette phrase ?"));
    println!("{:?}",nbmot("Fin."));
}