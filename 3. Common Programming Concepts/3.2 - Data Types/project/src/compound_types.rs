fn main(){
    /* Os "tipos compostos" podem agrupar vários valores em um tipo.
     * Rust tem dois tipos de compostos primitivos: tuplas e matrizes */

    /* TUPLAS
     * São um forma de agrupar tipos diversos em um único tipo composto.
     * Possuem tamanho fixo e uma vez declaradas não podem expandir. 
     * A tupla é considerada um valor único, é diferente de um vetor por exemplo
     * Parar acessar individualmente algum valor é necessario desestruturá-la ou buscá-lo por índice*/
    let tup: (i32, &str, i16, bool) = (500, "macaco", 1, true);
    println!("{:?}", tup);
    //Desestruturação
    let (x, y, z, w) = tup;
    println!("{}, {}, {}, {}", x, y, z, w);
    //Buscando no índice
    println!("{}", tup.1);


    /* ARRAYS
     * Em Rust, possuem tamanho fixo e só aceitam valores do mesmo tipo.
     * São úteis caso você deseje alocar espaço na mémoria stack ao invés do heap
     * Ou quando você deseja garantir um tamanh fixo de elementos
     * No entanto o tipo "vetor" é mais flexivel.
     * Em geral, caso você esteja em dúvidas entre vetor e array, escolha o vetor */
     // Ao escrever os meses do ano, faz sentido usar uma array, já que não haverá mudança no valores
     let _months = ["January", "February", "March", "April", "May", "June", "July",
     "August", "September", "October", "November", "December"];
     //[i32; 5] -> indica que os tipos será i32 e que o array contém 5 elementos
     let _a: [i32; 5] = [1, 2, 3, 4, 5];
     //[3; 5] -> equivale a dizer que os cinco elementos terão o valor 3 = [3, 3, 3, 3, 3]
     let _b = [3; 5];
}