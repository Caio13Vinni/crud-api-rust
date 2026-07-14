# CRUD API Rust

Uma API RESTful robusta e performática construída em Rust para operações CRUD (Create, Read, Update, Delete). Este projeto demonstra as melhores práticas de desenvolvimento em Rust com suporte a containerização via Docker.

## 🚀 Características

- **API RESTful** - Endpoints bem estruturados para gerenciamento de dados
- **Performance** - Compilado em Rust para máxima velocidade e eficiência
- **Type Safety** - Segurança de tipos em tempo de compilação
- **Docker Ready** - Configuração completa para containerização
- **Docker Compose** - Orquestração simplificada de serviços

## 📋 Pré-requisitos

### Opção 1: Desenvolvimento Local
- [Rust](https://www.rust-lang.org/tools/install) 1.70+ instalado
- Cargo (incluso com Rust)

### Opção 2: Docker
- [Docker](https://docs.docker.com/get-docker/) 20.10+
- [Docker Compose](https://docs.docker.com/compose/install/) 1.29+

## 🛠️ Instalação

### Local

1. **Clone o repositório**
```bash
git clone https://github.com/Caio13Vinni/crud-api-rust.git
cd crud-api-rust
```

2. **Instale as dependências e compile**
```bash
cargo build --release
```

3. **Execute a aplicação**
```bash
cargo run --release
```

A API estará disponível em `http://localhost:8080` (ou conforme configurado).

### Com Docker

1. **Clone o repositório**
```bash
git clone https://github.com/Caio13Vinni/crud-api-rust.git
cd crud-api-rust
```

2. **Inicie com Docker Compose**
```bash
docker-compose up -d
```

3. **Verifique se está rodando**
```bash
docker-compose ps
```

Para parar a aplicação:
```bash
docker-compose down
```

## 📖 Uso

### Exemplos de requisições

#### Criar um registro (POST)
```bash
curl -X POST http://localhost:8080/api/items \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Novo Item",
    "description": "Descrição do item"
  }'
```

#### Listar registros (GET)
```bash
curl http://localhost:8080/api/items
```

#### Obter um registro específico (GET)
```bash
curl http://localhost:8080/api/items/{id}
```

#### Atualizar um registro (PUT)
```bash
curl -X PUT http://localhost:8080/api/items/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Item Atualizado",
    "description": "Nova descrição"
  }'
```

#### Deletar um registro (DELETE)
```bash
curl -X DELETE http://localhost:8080/api/items/{id}
```

## 📁 Estrutura do Projeto

```
crud-api-rust/
├── src/                    # Código-fonte principal
│   ├── main.rs            # Ponto de entrada da aplicação
│   ├── handlers/          # Manipuladores de requisições
│   ├── models/            # Estruturas de dados
│   └── database/          # Configuração do banco de dados
├── Cargo.toml             # Dependências do projeto
├── Cargo.lock             # Lock file das dependências
├── Dockerfile             # Configuração Docker para produção
├── docker-compose.yml     # Orquestração de serviços
├── .gitignore             # Arquivos ignorados pelo Git
└── README.md              # Este arquivo
```

## 🔧 Configuração

### Variáveis de Ambiente

Configure as seguintes variáveis conforme necessário:

```bash
# Porta da API
PORT=8080

# Banco de dados
DATABASE_URL=postgres://user:password@localhost:5432/crud_db

# Ambiente
RUST_LOG=info
```

## 🧪 Testes

Execute os testes com:

```bash
cargo test
```

Para testes com output detalhado:

```bash
cargo test -- --nocapture
```

## 🏗️ Build para Produção

### Build otimizado
```bash
cargo build --release
```

O binário compilado estará em `target/release/crud-api-rust`.

### Build Docker
```bash
docker build -t crud-api-rust:latest .
docker run -p 8080:8080 crud-api-rust:latest
```

## 📊 Performance

Este projeto foi otimizado para:
- **Velocidade**: Compilação e execução rápidas
- **Memória**: Uso eficiente de recursos
- **Segurança**: Verificações de tipo em tempo de compilação

## 🤝 Contribuindo

Contribuições são bem-vindas! Para contribuir:

1. Faça um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📝 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo LICENSE para detalhes.

## 👨‍💻 Autor

**Caio Vinni** - [@Caio13Vinni](https://github.com/Caio13Vinni)

## 📞 Suporte

Tem dúvidas ou encontrou um bug? [Abra uma issue](https://github.com/Caio13Vinni/crud-api-rust/issues) no repositório.

## 🔗 Links Úteis

- [Documentação Rust](https://doc.rust-lang.org/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Docker Documentation](https://docs.docker.com/)

---

**Feito com ❤️ em Rust
