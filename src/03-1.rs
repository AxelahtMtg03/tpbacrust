fn maxi_tableau(tab: &[i32]) -> i32 {
    let mut maxi = tab[0];
    for &value in tab {
        if maxi < value {
            maxi = value;
        }
    }
    maxi
}

fn main() {
    // let tab = [98, 12, 104, 23, 131, 9];
    let tab = [-27, 24, -3, 15];
    let maxivalue = maxi_tableau(&tab);
    println!("la valeur max est {:?}", maxivalue)
}
