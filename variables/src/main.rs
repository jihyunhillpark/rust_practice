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

}
