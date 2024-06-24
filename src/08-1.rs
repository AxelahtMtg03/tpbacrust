fn delta(tab:&[i32])->Vec<i32>{
 let mut new_tab= Vec::new();
 new_tab.push(tab[0]);
 for i in 1..tab.len(){
    new_tab.push(tab[i]-tab[i-1]);
 }
 new_tab
}
fn main(){
    println!("{:?}", delta(&[1000, 800, 802, 1000, 1003]));
    println!("{:?}", delta(&[42]));
}