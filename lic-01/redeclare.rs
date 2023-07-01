fn main(){
    // Declare first variable binding with name "shadow_num"
    let shadow_num = 10;

    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 9;

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 3;

    println!("The number is {}.", shadow_num);//The number is 57.
}
