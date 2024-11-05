/*
The program should:
Declare a variable proceed that is set to true and print "Proceeding" if it is true, otherwise print "Not proceeding."
Declare a variable height with a value of 190, then check:
If height is greater than 180, print "Tall."
If height is between 171 and 180, print "Average."
Otherwise, print "Short."
*/

fn main(){

    let proceed = true;
    let height = 190;

    if proceed {
        println!("{}","Proceeding");
    }else{
        println!("{}", "Not proceeding");
    }

    if height > 180 {
        println!("{}","Tall");
    }else if height > 170 {
        println!("Average")
    }else{
        println!("Short")
    }

}