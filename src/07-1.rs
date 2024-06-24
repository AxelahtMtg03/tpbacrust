fn gbentier(tab: &[bool]) -> i32 {
    let mut nb_final = 0;
    if tab.len() == 0 {
        return 0;
    }
    for (i, &value) in tab.iter().rev().enumerate() {
        if value {
            nb_final += 2_i32.pow(i as u32);  // Conversion correcte pour l'exponentiation
        }
    }
    nb_final
}

fn main() {
    println!("{:?}", gbentier(&[]));
    println!("{:?}", gbentier(&[true]));
    println!(
        "{:?}",
        gbentier(&[true, false, true, false, false, true, true])
    );
    println!(
        "{:?}",
        gbentier(&[true, false, false, false, false, false, true, false])
    );
}
