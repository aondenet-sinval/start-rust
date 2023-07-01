fn main(){
    // Inteiro:
    let number: u32 = 14;//tipo number declarado como inteiro de 32 bits
    // Ponto flutuante:
    let number_64 = 4.0;      // Ponto flutuante sem anotação
    let number_32: f32 = 5.0; //Com anotação
    println!("Inteiro: {}, ponto flutuante sem anotação: {} com anotação: {}.", number, number_64, number_32);
    //Inteiro 14, ponto flutuante sem anotação 4 com anotação 5.
    // Operações
    // Adição subtração multiplicação
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Divisão com inteiro e ponto flutuante
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
}
