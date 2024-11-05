/*
How can you use variable shadowing in Rust to initially set a `height` variable to 190, 
then subtract 20 from it using shadowing to modify its value?

Next, store the result of a comparison based on this modified `height` in a variable called `result`.

For the comparison conditions:
- If `height` is less than 170, classify it as "Short".
- If `height` is between 170 and 180, classify it as "Average".
- If `height` is greater than 180, classify it as "Tall".

Implement this classification using an `if` statement.
*/


fn main(){

    let mut height = 190;

    println!("Initial value of height = {}", height);

    height = height - 20;

    println!("Subtracted value of height = {}", height);

    let result = if height > 180 {
        "Tall"
    }else if height > 170{
        "Average"
    }else{
        "Short"
    };

    println!("The value of result = {}", result);

    //Shadowing to a different type (NOT RECOMMENDED UNLESS REQUIRED)
    let health = if height < 180 (true) else (false)
}