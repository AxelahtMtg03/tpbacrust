fn max_et_ind(tab: &[i32]) -> (i32, usize) {
    let mut maxi = tab[0];
    let mut ind = 0;
    for i in 0..tab.len() {
        if maxi < tab[i] {
            maxi = tab[i];
            ind = i
        }
    }
    (maxi, ind)
}

fn main() {
    println!("{:?}", max_et_ind(&[1, 5, 6, 9, 1, 2, 3, 7, 9, 8]));
    println!("{:?}", max_et_ind(&[-2]));
    println!("{:?}", max_et_ind(&[-1, -1, 3, 3, 3]));
    println!("{:?}", max_et_ind(&[1, 1, 1, 1]));
}
