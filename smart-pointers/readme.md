# Anotações Smart Pointers

Os ponteiros em rust são os seguintes: 

* Referencias
* Box 
* Rc (Reference Counter)
* Arc (Atomic Reference Counter)
* RefCell

Strings e vetores são smart pointers. Em geral, os smart pointers implementam os traits **Deref** e **Drop**.
# Box
Uma box é simplesmente um ponteiro que implementa os traços Deref, DerefMut e Drop. Ele pode apontar para qualquer tipo de dado \<T\>, e armazena a referência na heap.
## Deref Trait
Essa trait é necessária para que seja possível acessar o valor de uma struct usando o operador '*'

## DerefMut
Mesma coisa que o Deref, a diferença eh que ela funciona para ponteiros mutaveis.

## DerefCoercion
O compilador converte os tipos das referências automaticamente em tempo de compilação. Existem algumas regrinhas, considerando que ```T:Deref<Target=U>```: 

* Deref de &T para &U
* DerefMut de &mut T para &mut U
* Deref de &mut T para &U
* !! Nunca de &T para &mut U

# Rc (Reference Counter)
O ponteiro **Rc** permite múltiplas posses sobre uma dada variável (apontamentos múltiplos) e acompanha a quantidade de referências para um dado recurso.

# RefCel
Este ponteiro possui a característica de impor as *borrowing rules* em runtime, e não em tempo de compilação. Isso é necessário em situações que, embora sejam *memory safe*, o compilador não é capaz de avaliar em tempo de compilação. Portanto, ao usar o RefCel, o ponteiro fará essa avaliação em runtime, e entrará em pânico caso as regras sejam violadas.