﻿# rust-language
Instalação do Rust visite: https://rustup.rs


* Cargo: é o gerenciador de compilação, gerenciador de pacotes e ferramenta de uso geral do Rust. Você pode utilizar o Cargo para iniciar um novo projeto, criar e executar
seu programa e gerenciar quaisquer bibliotecas externas da quais seu código depende
``` shell
 cargo --version 
```
* Rustc: é o compilador do Rust. Normalmente, deixamos o Cargo invocar o compilador para nós, mas às vezes é útil executá-lo diretamente.
``` shell
rustc --version
```
* Rustdoc: ferramente de documentação do Rust. Se você escrever doc na forma apropriada, em comentários no código-fonte do seu programa, rustdoc pode construir um html bem formatado
a parti deles. Como corre com rustc, geralmente deixamos o Cargo executar rustdoc para nós.
``` shell
rustdoc --version
```
Cargo pode criar um novo pacote Rust para nós, com alguns metadados padrão:
``` rust
cargo new hello
``` 
