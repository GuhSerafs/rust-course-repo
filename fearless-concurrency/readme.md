# Concorrência 
O manejo das threads em rust, em geral, se baseia nos seguintes elementos: 
* Mutex
* Arc

## Mutex
O mutex permite que um recurso compartilhado seja acessado de forma segura por diferentes threads, usando o método lock(). Ao contrário de outras linguagens, não é necessário fazer o unlock() manualmente. 

## Arc
O ponteiro Arc é um Rc atômico, que permite que diferentes threads apontem para o mesmo objeto de forma segura. Ele implementa alguns traços de sincronização que são importantes para garantir a segurança entre threads.

## Setup e uso dos Arcs para multiplas threads
Conforme mostrado nos códigos dessa pasta, geralmente declara-se um recurso (constante ou variável) como Arc aninhando um mutex dentro do recurso.
```rust
let r = Arc::new(Mutex::new(<recurso>));
```

Na hora de passar o recurso para a thread, é necessário fazer um clone antes. Depois, dentro da thread, é possível acessar o recurso de forma segura usando o lock do mutex.

```rust
let r_clonado = Arc::clone(&r);
handler = thread::spawn(move || {
    let mut r_thread = r_clonado.lock().unwrap();
    
    ...
    <snip>
    ...
});

handler.join().unwrap();
```