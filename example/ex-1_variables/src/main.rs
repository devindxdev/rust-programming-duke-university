fn main(){
    //all vars are immutable by def
    //to make it mutable, use mut like so:
    let mut message = String::from("Name: Alfredo, Weight: ");
    message.clear();
    let mut weight = 190.0;

    let kilos = weight / 2.2; //type checking, cannot do int/float
    println!("{}{}", message,weight);
    println!("{}{}", message, kilos);

}