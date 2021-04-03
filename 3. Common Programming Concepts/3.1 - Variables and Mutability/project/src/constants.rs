fn main() {
    /* CONSTANTES:
    ** Constantes são imutaveis, sempre. Não se pode torná-las mutaveis como as variaveis.
    ** Não podem receber o valor de uma chamada de função ou algo calculado em tempo de execução.
    ** Devem ser escritas em letras maiusculas.
    ** O seu tipo deve ser sempre definido de antemão */
    //O tração [_] indica que esse valor não vai ser usado
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);
}
