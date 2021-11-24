# Introdução
Essa lib se baseia no cap 3 do livro Programming Rust. Nesse markdown estarão as anotações importantes sobre o capítulo. 

* O compilador exige que valores representando o tamanho de arrays ou vetores (e o tamanho de outras estruturas de dados) sejam tipados com ```usize```
* Tipos numéricos e caracteres são tratados diferentemente, mas existe uma sintaxe para **byte literals** que servem pra representar caracteres como um valor ```u8```. Exemplo: ```b'Ola Mundo'``` (Gera os caracteres ASCII de _Ola Mundo_).
* Situações onde ocorre overflow de um tipo numérico **só geram um panic em builds do tipo debug**. Em **release** funciona normalmente. Nessas situações, o ideal é definir se o overflow é desejado/indesejado e utilizar os métodos apropriados (wrapping/checked)
* Também existem as operações do tipo ```saturating``` (onde o valor é "saturado" ao valor máximo ou mínimo caso ocorra overflow) e as operações ```overflowing``` que retorna uma tupla ```(result, overflowed): (usize, bool)``` 
* Rust não suporta conversões implícitas entre tipos numéricos. Isso ocorre devido ao fato de que conversões implícitas são uma conhecida fonte de bugs em C/C++.
* O tamanho dos arrays em rust são constantes e determinados em tempo de compilação. Como o comprimento do array faz parte de sua tipagem, é impossível criar arrays de tamanho variável. Deve-se usar vetores para este fim.
* No uso de vetores, sempre que se souber em geral qual a capacidade máxima do vetor, é interessante usar o método ```Vec::with_capacity(n)``` para alocar a quantidade específica de memória uma única vez.
* Slices são regiões de um array ou de um vetor, e como podem ter um tamanho variável, geralmente elas não podem ser armazenadas em variáveis ou passada como argumentos de funções sem que seja por referência. 
* A maioria das funções que podem ser aplicadas tanto a arrays como em vetores são definidas em slices. A notação típica para slices é &[T] ou &str.