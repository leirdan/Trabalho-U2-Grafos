# Trabalho Unidade 2 Grafos

O resumo em vídeo está disponível [aqui](https://drive.google.com/file/d/14Frkv91yFWFBreM_ojlaLVPwyXyVqA9f/view?usp=sharing)

## Estrutura do repositório em Rust 🦀

```bash
Trabalho-U2-Grafos/
├── README.md
├── Cargo.lock
├── Cargo.toml
├── examples    # Programas de exemplo para testar implementações
│   ├── demo_bellmanford.rs # exemplo do funcionamento do algoritmo de Bellman-Ford
│   ├── demo_dijkstra.rs    # mesma coisa para o Dijkstra...
│   ├── demo_kruskal.rs
│   └── demo_prim.rs
├── latex # Código fonte do documento latex
│   ├── chapters/
│   ├── CS_report.sty   # Definições e import de biblioteca
│   ├── figures         # Imagens usadas no documento
│   ├── main.tex        # Entry point do código fonte
│   ├── Dockerfile      # Receita de uma imagem docker para compilar o documento
│   ├── Makefile        # Para compilar o documento
│   └── references.bib  # Referências usadas no texto
└── src # Código fonte da implementação dos algoritmos
    ├── adjacency_list.rs        # Implementação de um grafo ponderado como lista de adjacência
    ├── eulerian_cycle.rs        # Implementação de algoritmos relacionados a caminhos eulerianos
    ├── graph.rs                 # Traços relacionadas a grafos (Grafo, Grafo não direcionado, Grafo ponderado)
    ├── lib.rs                   # Módulos exportados pela biblioteca
    ├── minimum_spanning_tree.rs # Algoritmos relacionados a criação de árvores geradoras mínimas
    ├── shortest_path.rs         # Algoritmos de menor caminho
    └── traversal.rs             # Algoritmos de travessia no grafo (DFS, BFS, etc)
    ...
```

## Desenvolvimento

### Pré-requisitos

- [Cargo 1.90.0 (com rustc 1.93.0 nightly)](https://rust-lang.org/learn/get-started/)
- [Texlive (full)](https://tug.org/texlive/) e Texlive-lang-portuguese: pode ser encontrado nos gerenciadores de pacote comuns.
- [Docker](https://www.docker.com/): Alternativa para compilar o $\LaTeX$, caso não queira instalar o `texlive`
- [Graphviz](https://www.graphviz.org/download/): Para converter os arquivos `.dot` em imagens `.png`

### Compilação e testes

> [!NOTE]
> Antes de testar o projeto, troque a versão do compilador para a versão nightly com:
>
> ```bash
> rustup override set nightly
> ```

```bash
# Compila o projeto
cargo b

# Executa binários na pasta examples/
cargo r --example [example]

# Executa testes unitários
cargo t

# Executa benchmarks
cargo bench

# Verifica o código usando o clippy
cargo clippy

# Formata o código
cargo fmt

# Compila documentação
cargo doc
```

#### $\LaTeX$

Na pasta `latex/`:

```bash
# Exibe receitas disponíveis
make help

# Compila pdf no diretório output/
make

# Limpa arquivos auxiliares
make clean

# Limpa todos os arquivos (incluindo pdf)
make distclean

# Limpa e compila novamente
make rebuild
```

#### $\LaTeX$ com Docker

```bash
# Cria a imagem docker
docker build -t latex-compiler latex/

# Compila a imagem e executa o container criando o pdf.
# --rm automaticamente deleta o container e o volume
docker run --rm latex-compiler > main.pdf
```

Existe uma imagem compilada em `vleema/latex-compiler` (não garantimos que esteja atualizada). Podes substituir `docker build...` por

```bash
docker pull vleema/latex-compiler:latest
```
