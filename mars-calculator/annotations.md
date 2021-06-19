# Resumo dos tipos de dados

## Bytes
| Sintaxe | Significado | Tamanho (Bytes) |
|:-:|:-:|:-:|
| u8 | unsigned byte | 1 |
| i8 | signed byte | 1 |
| bool | booleana | 1 |

## Inteiros
| Sintaxe | Significado | Tamanho (Bytes) |
|:-:|:-:|:-:|
| u16 | unsigned int | 2 |
| i16 | signed int | 2 |
| u32 | unsigned long int | 4 |
| i32 | signed long int | 4 |
| u64 | unsigned long² int | 8 |
| i64 | signed long² int | 8 |
| u128 | signed long³ int | 16 |
| i128 | signed long³ int | 16 |

## Words
As words são inteiros que dependem da arquitetura do processador (32 bits ou 64 bits).

| Sintaxe | Significado | Tamanho (Bytes) |
|:-:|:-:|:-:|
| usize | unsigned number | word |
| isize | signed number | word |

## Pontos flutuantes
| Sintaxe | Significado | Tamanho (Bytes) |
|:-:|:-:|:-:|
| f32 | float | 4 |
| f64 | double float | 8 |

## Char
Em rust, o char é dimensionado considerando que cada char recebe um caractere utf-8 (unicode).
| Sintaxe | Significado | Tamanho (Bytes) |
|:-:|:-:|:-:|
| char | unicode | 4 |
