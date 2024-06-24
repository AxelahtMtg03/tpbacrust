fn moy(tab1:&[f32],tab2:&[i32])->(f32,i32){
    let mut min = tab1[0];
    let mut ind=0;
    for temp in 0..tab1.len(){
        if tab1[temp]<min{
            min=tab1[temp];
            ind=temp;
        }
    }
    (min,tab2[ind])
}
fn main(){
    println!("{:?}",moy(&[14.9, 13.3, 13.1, 12.5, 13.0, 13.6, 13.7], &[2013, 2014, 2015, 2016, 2017, 2018, 2019]))
}