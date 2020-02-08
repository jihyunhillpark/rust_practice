fn main() {
    /*iteration can be used by loop,while,for*/
    let a = [10,20,30,40,50];
    let mut index = 0;

    //Implementing in this way is vunerable to index error.
    //index error will call panic!
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    //Alternative way - reduce the possibility of bugs 
    //from omitting some items in array 
    index = 0;
    for element in a.iter() {
        println!("the value is: {}", a[index]);
    }
    /*Use 'for' and the code will be concise and safe*/
    //reverse printing method - rev()
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
