fn main() {
    let number = 3;

    /*Condition should be predicate. Tures or false. Any ohter than these 
     * will indicate an error*/
    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }
    /*To avoid too nested cases we can use match*/
    
    /*Use 'if' in 'let' statement
     * Be aware that block{} is it self an expression 
     * and the final value of expression is the last expresiion in the block
     * Don't forget to make the last each expression have the same type*/
    let condition = true;
    let number = if condition{
        5
    } else {
        6
    };

    println!("The value of number is : {}",number);
}
