fn recherche(tab: &[i32], target: i32) {
    let mut counter = -1;
    let mut i = -1;
    for &value in tab {
        if value == target {
            i += 1;
            counter = i;
        } else {
            i += 1;
        }
    }
    match counter {
        -1 => println!("None"),
        _ => println!("L'indice est {:?}", counter),
    }
}
fn main() {
    //recherche(&[2, 4], 2);
    //recherche(&[5, 3], 1);
    recherche(&[2, 3, 5, 2, 4], 2);
}
