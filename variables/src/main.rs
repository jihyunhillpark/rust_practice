fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*Using - mut - mean variable variable 
    let spaces = "   ";
    let spaces = spaces.len();
    Compile error will occur because variable 'spaces' is already type defined*/

    /*Not Using - mut - means unvariable variable*
    In this case, assiging value to using let represent 'shadowing'
    A big difference between a former is that shadowing allowing type change*/
    let spaces = "   ";
    let spaces = spaces.len();

    /*Sometimes type should be explicit. 
     *eg. let guess = "42".parse().expect("Not a number!"); 
     * occurs error.*/
    let guess: u32 = "42".parse().expect("Not a number!");
    
    /*explicit type is optional in tuple*/
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    /*tuple pattern matching*/
    let tup1 = (500, 6.4, 1);

    let (x, y, z) = tup1;
    /*access tuple by index*/
    let five_hundred = tup1.0;

    /*Array - all element should be same type contrary to tuple
     *Data will be saved in stack instead of heap.
     *If size of data is unclear, it'd be better to use vector.*/
    let a = [1,2,3,4,5];
    let first = a[0];
    /*Index error 
     * tehnth = a[10]
     * Here, have no problem with compiling, but error finally happens 
     * during the execution*/
    println!("The value of y is: {}", y);
    
    another_function(5,6);
}
    /*Parameters of functions should define the type*/
fn another_function(x: i32, y: i32) -> i32{
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    /* let x = (let y = 6);
     * Error! because right side is not expression
     * To assign left side, the expression should be wrapped by middle brackets*/
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // be aware that semi colon is not used here.
        //Being semi colon used indicates statement 
        //, but not returing value - expression. 
    };
    x+1 //Watch out here as well. Implicitly return the value by expression.  
    //also adding ; here will occur error since it's got statement 
    //with no returing value 
}
