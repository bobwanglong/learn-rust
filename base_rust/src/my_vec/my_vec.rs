pub fn demo(){
    let mut v= vec![1,2,3];
    let first = &v[0];
    println!("{}",first);
    v.push(4);
    for i in &v{
     println!("{}",i);
}
}