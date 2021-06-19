# Anotações sobre as aulas deste exemplo
___
## Funções
Na declaração das funções, o tipo de todas as variáveis, assim como o tipo do retorno, devem ser declarados. O padrão para nomes de funções em rust é o "snake_case". 

Assinatura geral de qualquer função é: 
```rust 
fn nome_da_função(var1 : tipo, var2 : tipo, ...) -> return_type {
    <predicados>;
    return {}
}
```

Interessante: O compilador retorna automaticamente a última linha, desde que não incluir um ponto e vírgula.
Ou seja: "return 50.0;" <--> "50.0"

```rust 
fn nome_da_função(var1 : tipo, var2 : tipo, ...) -> return_type {
    <predicados>;
    50.0
}
```

## Macros
Macros são como funções, mas que podem receber um número variável de parâmetros, inclusive com tipos diferentes. Macros são bastante poderosas, no entanto, declarar macros é uma tarefa mais complicada. As macros são indicadas por um ! no final de seus nomes. println! é uma macro. É possível usar o comando 'cargo expand' para obter o código fonte com as macros abertas. 

## Mutabilidade
A palavra reservada 'let' serve para declarar variáveis no geral. Todas as variáveis em rust são imutáveis por padrão. Para declarar uma variável mutável, deve-se acrescentar a palavra reservada 'mut' na declaração da variável. 

## Ownership (Posse)
A posse está relacionada ao pertencimento de uma variável ao seu escopo. Existem três regras da mutabilidade em Rust (em inglês): 
*1. Each value in Rust is owned by a variable.*
*2. When the owner goes out of scope, the value will be deallocated.*
*3. There can only be ONE owner at a time.*
Qualquer tentativa de burlar essas regras vai gerar um erro no compilador. Devido ao comportamento dos *smart pointers*, quando uma variável alocada na *heap* sai de seu escopo, essa variável é desalocada. Neste caso, se essa variável for passada dentro de uma função, ela não poderá ser usada posteriormente. Caso seja necessário que uma função modifique uma variável e depois possa ser usada, é necessário que ela seja emprestada (borrow). Outro problema que pode ocorrer são duas variáveis apontando para o mesmo endereço na *heap*. Nesse caso, o compilador também gera um erro, pois do contrário poderia ocorrer uma corrupção na memória na segunda vez que o programa tentasse desalocar o endereço. 

## References and Borrowing (Referências e Empréstimos)
As referências funcionam da mesma forma como em C/C++. Em geral, quando precisa-se que uma função mude o conteúdo de uma função, você passa o "endereço" (ou a referência) da variável para a função: 
```rust
fn dummy_fn(variable : &mut char){
    //<predicado>;
}
```
Passar o endereço chama-se *borrow* em Rust. Uma variável só pode ser emprestada uma única vez como mutável, e infinitas vezes como imutável. Além disso, se ela for empresada como mutável, ela não pode ser emprestada de nenhuma outra forma. Esta restrição é uma das grandes vantagens de Rust, pois o compilador previne '*race conditions*' em tempo de compilação, de modo que se houver uma *data race* no código, ele não permite que o código seja compilado. 