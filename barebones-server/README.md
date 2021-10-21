# Barebones Server
Esse projeto será implementado unicamente com as informações do livro. Nesse arquivo vou colocar as anotações que eu julgar pertinentes na análise e estudo desse projeto.

## Listening TCP Conections
Pontos interessantes desse trecho do livro

* A função ```bind``` retorna um ```Result<T, E>``` que em caso de sucesso retorna um ```TcpListener```. 
* Este ```TcpListener``` possui um método chamado ```incoming```, que ao ser chamado retorna um iterável contendo ```streams```. 
* Uma *stream* representa uma conexão aberta (representando uma tentativa de comunicação). 
* Uma das fontes de erros na leitura de uma stream podem ser: exceder o limite de conexões abertas pelo sistema operacional

## Reading the Request
Pontos interessantes desse trecho do livro

* O parâmetro stream tem que ser mutável pois a instância do ```TcpStream``` acompanha os dados que ele retorna, o que causa alterações no estado interno da entidade. 
* O tamanho do buffer foi definido como 1024, o que deve ser suficiente para requisições básicas. Para lidar com requisições de tamanho arbitrário, seria necessário implementar um gerenciamento de buffer. 
* A função ```String::from_utf8_lossy``` faz a transformação ```&[u8]``` -> ```String```. 

## Closer Look at HTTP & Writting a Response
Pontos interessantes desse trecho do livro

* URI é quase a mesma coisa que URL, que seria basicamente o caminho que aparece após o endereço. ('\' ou '\tests')

Estrutura básica de uma resposta: 
```
HTTP-Version Status-Code Reason-Phase CRLF
headers CRLF
message-body
```
