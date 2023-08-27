# cotametais-lme-api-rust

## Descrição

Este é um backend API escrito em Rust para fornecer dados do London Metal Exchange (LME). É um trabalho em progresso. 🦀

## Stack

- Rust (linguagem de programação de alto nível com baixo footprint de memória e performance comparável com a de C/C++)
- Actix Web (framework HTTP para Rust)
- Tokio (Runtime async não-bloqueante que utiliza loop de eventos single-threaded)
- MongoDB (banco de dados opensource, de alta performance, flexível e NOSQL)
- Serde (lib de desserialização/serialização em JSON)
- Nginx (balanceador de carga baseado em eventos)
- Utoipa (gerador de documentação automática no formato OpenAPI)
- Utoipa-rapidoc (UI para documentação automática no formato OpenAPI)

## Como utilizar

### Test

```bash
cargo test
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Docker

```bash
docker-compose up
```
