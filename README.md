# 🦀 Rust CRUD API - "From Scratch"

Uma API REST para gestão de utilizadores (CRUD) desenvolvida integralmente em **Rust** sem o recurso a quaisquer frameworks web (como Axum, Actix-web ou Rocket). 

O servidor HTTP e o encaminhamento (*routing*) foram construídos a partir do zero utilizando apenas a biblioteca padrão do Rust (`std::net`), manipulando diretamente *sockets* TCP e efetuando a análise sintática (*parsing*) manual do protocolo HTTP. A persistência de dados é realizada numa base de dados **PostgreSQL**, com todo o ambiente orquestrado através de **Docker Compose**.

---

## 🚀 Destaques Técnicos

* **Servidor TCP Nativo:** Gestão direta de ligações através de `std::net::TcpListener` e `TcpStream`.
* **Protocolo HTTP Manual:** Interpretação manual de métodos HTTP (`GET`, `POST`, `PUT`, `DELETE`), cabeçalhos (*headers*) e construção de respostas HTTP com os respetivos códigos de estado (*Status Codes*).
* **Persistência Relacional:** Integração direta com base de dados PostgreSQL utilizando a *crate* nativa `postgres`.
* **Serialização JSON:** Manipulação de dados estruturados com `serde` e `serde_json`.
* **Ambiente Contentorizado:** Suporte completo para Docker e Docker Compose, utilizando *multi-stage builds* para gerar imagens finais extremamente leves e otimizadas.
* **Migração Automática:** Criação automática da tabela `users` na base de dados durante o arranque do servidor.

---

## 🛠️ Tecnologias Utilizadas

* **[Rust](https://www.rust-lang.org/)** (Edição 2024)
* **[PostgreSQL](https://www.postgresql.org/)** (v16)
* **[Serde](https://serde.rs/)** (Serialização e Desserialização JSON)
* **[Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)**

---

## 📦 Como Executar o Projeto

Existem duas formas de executar a aplicação: utilizando o **Docker Compose** (método recomendado e mais rápido) ou executando localmente através do **Cargo**.

### Opção 1: Com Docker Compose (Recomendado)

**Pré-requisitos:** Ter o [Docker](https://docs.docker.com/get-docker/) e o [Docker Compose](https://docs.docker.com/compose/install/) instalados na máquina.

1. Clone o repositório e aceda à pasta do projeto:
   ```bash
   git clone [https://github.com/caio13vinni/crud-api-rust.git](https://github.com/caio13vinni/crud-api-rust.git)
   cd crud-api-rust

   Inicie os contentores (base de dados e API) em segundo plano:
Bash

docker compose up --build -d
