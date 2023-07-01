struct Aluno { nome: String, serie: u8, aprovacao: bool}
fn main(){
    let aluno_1 = Aluno { nome: String::from("Carlos Silva"), serie: 2, aprovacao: true};
    println!("Aluno 1 {} serie {} status apovação {}, Materias {} {} {} nota: {}",
        aluno_1.nome, aluno_1.serie, aluno_1.aprovacao);
}
