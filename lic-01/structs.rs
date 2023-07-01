//Struct básica:
struct Aluno { nome: String, serie: u8, aprovacao: bool}
// Struct com tuplas:
struct Materias(char, char, char, f32);
fn main(){
    // Instanciando as structs
    let aluno_1 = Aluno { nome: String::from("Carlos Silva"), serie: 2, aprovacao: true};
    let aluno_1_m = Materias('A', 'B', 'B', 3.65);
    println!("Aluno 1: {}. \nSérie: {}. \nStatus aprovação: {}. \nMaterias: {} {} {}. Nota: {}",
        aluno_1.nome, aluno_1.serie, aluno_1.aprovacao,
        aluno_1_m.0, aluno_1_m.1, aluno_1_m.2, aluno_1_m.3);
/*
Aluno 1: Carlos Silva.
Série: 2.
Status aprovação: true.
Materias: A B B. Nota: 3.65
*/
}
