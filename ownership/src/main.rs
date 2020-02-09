
fn main() {
    let mut s = String::from("jihyunee"); //make string literal(immutable) to String(mutable)

    s.push_str(", Park"); //appending String

    println!("Hello {}", s);

    {
        let s = String::from("hello"); // 's' is available from here
        // 's' is still alive
    }  //out of scope! so 's'is not available
    /* String has returned the memory to OS
     * when variable is out of scope, RUST will call special function
     * whose name is 'drop'
     * RUST automatically calls 'drop' when it closed by '}'
     * However, developer can also call 'drop' manually */

    /* Copy */
    // The size of integer is already determined
    // and the value of the x which is 5 will be copied
    // and the copied value will be saved in 'y'
    // such that both 5 values are pushed in stack sequencely.
    let x = 5;
    let y = x; //x is still available. x is not moved to y 

    
    /* Move : similar to shallow copy
     * Interacation between variable and data 1 */
    //String different
    //pointer, lenth, capacity are push to stack 
    //but actual contents are saved in heap (traced by pointer in stack)
    let s1 = String::from("hello");
    let s2 = s1; //Move!! s1 is not available from now. It's moved to s2
    // s2 = s1
    // Here, the contents of s1 in stack(p,l,c)is copied to s2, 
    // but, the actual contents(value) of s1 saved in heap are not copied. 
    // println!("{}, world!", s1); will occur an error
    // because s1 is not available anymore in Rust
    

    /* Clone : similar to deep copy
     * Interacation between variable and data 2 */
    // Heap as well as stack is copied.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    /* Ownership and function */
    //passing the arguement in function is similar to
    //assing value to variable
    //such that passing the arguement in function
    //indcates 'Move' or 'copy'
    let s3 = String::from("hello");  // s3가 스코프 안으로 들어왔습니다.

    takes_ownership(s3);             // s3의 값이 함수 안으로 이동했습니다...
                                    // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.
    let s4 = gives_ownership();         // gives_ownership은 반환값을 s4에게
                                        // 이동시킵니다.

    let s5 = String::from("hello");     // s5가 스코프 안에 들어왔습니다.

    let s6 = takes_and_gives_back(s5);  // s6는 takes_and_gives_back 안으로
                                        // 이동되었고, 이 함수가 반환값을 s3으로도
                                        // 이동시켰습니다.
    
    let s7 = String::from("hello");

    let (s8, len) = calculate_length(s7); //s7의 소유권을 돌려받는 한가지 방법

    println!("The length of '{}' is {}.", s8, len);
    //s7의 소유권이 함수 인자로서 전달되었기에 s8로 Move함 s7은 더이상 존재노노
    //println!("The length of '{}' is {}.", s7, len); //에러 발생
}
// 여기서 x는 스코프 밖으로 나가고, s3도 그 후 나갑니다. 하지만 s3는 이미 이동되었으므로,별다른 일이 발생하지 않습니다.
// 여기서 s6는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s5는 스코프 밖으로벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s4은 스코프 밖으로 벗어나서 drop이 호출됩니다.
fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.


fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
                                             // 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

    some_string                              // some_string이 반환되고, 호출한 쪽의
                                             // 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
                                                      // 안으로 들어왔습니다.

    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length) //튜플 반환을 통해서 소유권을 보내
}
