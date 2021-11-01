# Barebones Server
Esse projeto será implementado unicamente com as informações do livro. Nesse arquivo vou colocar as anotações que eu julgar pertinentes na análise e estudo desse projeto.

## Single-Threaded Web Server
### Listening TCP Conections
Pontos interessantes desse trecho do livro

* A função ```bind``` retorna um ```Result<T, E>``` que em caso de sucesso retorna um ```TcpListener```. 
* Este ```TcpListener``` possui um método chamado ```incoming```, que ao ser chamado retorna um iterável contendo ```streams```. 
* Uma *stream* representa uma conexão aberta (representando uma tentativa de comunicação). 
* Uma das fontes de erros na leitura de uma stream podem ser: exceder o limite de conexões abertas pelo sistema operacional

### Reading the Request
Pontos interessantes desse trecho do livro

* O parâmetro stream tem que ser mutável pois a instância do ```TcpStream``` acompanha os dados que ele retorna, o que causa alterações no estado interno da entidade. 
* O tamanho do buffer foi definido como 1024, o que deve ser suficiente para requisições básicas. Para lidar com requisições de tamanho arbitrário, seria necessário implementar um gerenciamento de buffer. 
* A função ```String::from_utf8_lossy``` faz a transformação ```&[u8]``` -> ```String```. 

### Closer Look at HTTP & Writting a Response
Pontos interessantes desse trecho do livro

* URI é quase a mesma coisa que URL, que seria basicamente o caminho que aparece após o endereço. ('\' ou '\tests')

Estrutura básica de uma resposta: 
```
HTTP-Version Status-Code Reason-Phase CRLF
headers CRLF
message-body
```
## Multithreaded Web Server

### Thread Pool
Basicamente é um grupo threads que ficam aguardando uma tarefa. 

### Worker Struct & Requests via Channels
Nesse trecho vale observar que o objetivo da estrutura 'worker' é basicamente servir de interface para as ```threads::JoinHandle```, de modo que originalmente as threads fiquem aguardando uma tarefa. Dessa forma o código fica mais limpo e as funcionalidades ficam bem definidas dentro de cada estrutura de dados. 

### Implementing the execute method
Um ponto interessante que foi abordado nesse trecho está relacionado com o uso de um mutex dentro de um match statement. Especificamente, este código: 
```rust
let job = receiver.lock().unwrap().recv().unwrap();
```
Nesse ponto, o ```lock``` é dropado após passar por essa linha, de modo que as outras threads podem rodar livremente. Se fosse feito usando um if let, while let ou match block, o ```lock``` só seria liberado após o término do bloco, o que bloquearia a thread em execução e não permitiria as outras threads de trabalharem. 