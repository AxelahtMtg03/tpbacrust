fn verifie(tab: &[i32]) -> bool {
    if tab.len() == 1 || tab.len() == 0 {
        return true;
    } else {
        for i in 0..tab.len() - 1 {
            if tab[i] == tab[i + 1] {
                return true;
            }
        }
    }
    false
}
fn main() {
    println!("{:?}", verifie(&[0, 5, 8, 8, 9]));
    println!("{:?}", verifie(&[8, 12, 4]));
    println!("{:?}", verifie(&[-1, 4]));
    println!("{:?}", verifie(&[]));
    println!("{:?}", verifie(&[5]));
}
