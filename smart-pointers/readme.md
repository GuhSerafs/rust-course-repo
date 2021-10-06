# Anotaçoes Smart Pointers

Os ponteiros em rust sao os seguintes: 

* Referencias
* Box 
* Rc (Reference Counter)
* Arc (Atomic Reference Counter)
* RefCell

Strings e vetores sao smart pointers. Em geral, os smart pointers implementam os traits Deref e Drop

# Deref Trait
Essa trait eh necessaria para que seja possivel acessar o valor de uma struct usando o operador *

# DerefMut
Mesma coisa que o Deref, a diferença eh que ela funciona para ponteiros mutaveis.

# DerefCoercion
O compilador converte os tipos das referencias automaticamente em tempo de compilacao. Existem algumas regrinhas, considerando que T:Deref<Target=U>: 

* Deref de &T para &U
* DerefMut de &mut T para &mut U
* Deref de &mut T para &U
* !! Nunca de &T para &mut U