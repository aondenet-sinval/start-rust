fn main(){
    // Declarando vetor com vec
    let tres = vec![1,3,5];
    let cinco = vec![7;5];
    println!("Conteudo dos vetores {:?} {:?}", tres, cinco);
    // Conteudo dos vetores [1, 3, 5] [7, 7, 7, 7, 7]
    // Criando vetor modificavel
    let mut frutas = Vec::new();
    frutas.push("Uva");
    frutas.push("Maçã");
    frutas.push("Abacaxi");
    frutas.push("Banana");
    println!("Minhas frutas preferidas são: {:?}", frutas);
    // Minhas frutas preferidas são: ["Uva", "Maçã", "Abacaxi", "Banana"]
    println!("Fruta removida: {:?}", frutas.pop());
    // Fruta removida: Some("Banana")
    println!("Frutas mantidas {:?}", frutas);
    // Frutas mantidas ["Uva", "Maçã", "Abacaxi"]
    // Acessando vetor pelo indice
    println!("Primeira fruta {}", frutas[0]);
    //Primeira fruta Uva
    // Alterando valor do vetor
    let mut mutavel = vec![1, 5];
    mutavel[0] = mutavel[0] + 5;
    println!("O valor do elemento mudou para {:?}", mutavel);
    // O valor do elemento mudou para [6, 5]
}
