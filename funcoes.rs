fn soma(a: u32, b: u32){
    let x = a + b;
    println!("A soma é: {}", x)
}
fn  main(){
    println!("Trabalhando com funções:");
    ola();
    soma(2, 9);
    let valor = 3;
    let zero = 0;
    println!("Raiz quadrada: {}", square(valor));
    println!("Divisão por zero: {}", dividir(zero));
}
fn ola(){
    println!("Olá mundo de funções do rust...");
}
// Exemplo de função que retorna um valor:
// Função que retorna a raiz quadrada e um número passado com argumento
fn square(sqr: u32) -> u32 {
    sqr * sqr
}
fn dividir(sqr: u32) -> u32 {
    // Prevenindo erros de divisão por zero
    if sqr == 0 {
        return 0;
    }
    sqr / 2
}
/*
Trabalhando com funções:
Olá mundo de funções do rust...
A soma é: 11
*/
