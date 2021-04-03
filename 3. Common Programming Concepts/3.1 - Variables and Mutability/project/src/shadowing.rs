fn main(){
    /* SOMBREAMENTO [shadowing]:
    ** Podemos declarar duas variaveis com o mesmo nome.
    ** A última a ser declarada oculta a primeira, e passa a ocupar seu lugar.
    ** Esse processo é chamado de "shadowing" [sombreamento]
    ** O shadowing não significa que as variaveis serão do mesmo tipo. **/
    let y = 10;
    let y = y * 3;
    println!("{}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);
}