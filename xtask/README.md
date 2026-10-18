# Automação Rust com cargo xtask

Requer Rust/rustup 1.99.0 e ferramentas externas por operação. Cargo.lock é resolvido realmente; Python não é requisito. Android mantém integração nativa Kotlin/Compose/Gradle.

| Operação | Comando |
|---|---|
| Validar especificação | `cargo xtask check-docs` |
| Validar configs | `cargo xtask check-delivery` |
| Gates locais | `cargo xtask ci docs` |
| Regressões de tooling e crates do workspace | `cargo test --workspace --locked` (incluído em `cargo xtask ci docs`; xtask isolado: `cargo test --package xtask --locked`) |
| Verificar plugins nativos | `cargo xtask doctor --agent both` |
| Preparar autor/revisor bootstrap | `cargo xtask repo prepare-agents` |
| Worktree | `cargo xtask repo worktree --agent codex --task TASK-E11-01 --slug foundation` |
| Importar GitHub privado | `cargo xtask github-import --destination ../vynamp-git --push` |
| Proteções | `cargo xtask repo protect --repo jimmykedlyn/vynamp-mediaplayer --reviews 0` |
| Auditoria remota (somente leitura) | `cargo xtask repo audit --repo jimmykedlyn/vynamp-mediaplayer --out <arquivo.json>`; falha enquanto houver pendência Gitflow |
| Detectar fontes reais | `cargo xtask ci detect` |
| Hardware real | `cargo xtask quality-gate` |
| Preflight de release | `cargo xtask release preflight --version 0.1.0` |
| Instaladores assinados | `cargo xtask package --platform windows-x64 --version 0.1.0` |
| Pacote documental | `cargo xtask bundle-spec --output /caminho/fora-do-projeto/vynamp.zip` |

`cargo xtask --help` expõe subcomandos e argumentos. Usar `--root` somente para escolher projeto explícito, não para contornar isolamento.

Validadores verificam Markdown/index/links, REQ/TASK/QA, DAG/fases, plataformas M3, sincronia JSON/MD, dez schemas Draft2020-12 e exemplos, OpenAPI refs/operações/params, SVG/XML e contraste. VALIDATION.json registra o resultado. Não substituem compilação WIT, validação integral OpenAPI ou QA do player.

Doctor rejeita plugin ausente, desabilitado, versão diferente e discovery desconhecido. Instalação Codex usa o manager interativo oficial `/plugins`; Claude usa seu marketplace oficial. Ajustar parser/lock por PR e evidência real se os managers mudarem.

Instaladores dependem de app, toolchains Android, runtime audit, licenças, certificados e evidências ainda ausentes. Release permanece bloqueada. Comandos externos recebem argumentos separados; operações sensíveis suprimem logs de assinatura e usam credenciais efêmeras. Publicação automatizada cria somente draft.

O scanner de distribuição exclui caches e nomes/extensões conhecidos de credenciais; segredos nunca devem ser escritos em código ou documentos. Importação valida symlinks e destinos antes de remover arquivos legados. .gitattributes mantém fontes em LF também no Windows para digest consistente.

Revisão 0.6 valida também o catálogo de formatos, IDs/targets/fases, WebM/FLAC, IAMF, Bluetooth e limites UTF-8/envelope dos exemplos de ativação. Esses checks não são servidor IPC nem decoder do player.

Windows: docs clippy/test usam target/xtask-ci ou alternativa resolvida fora do executable atual. Caminhos de arquivos em argumentos Git/gh usam representação ordinária Windows. Bundle inclui raiz vynamp-media-player/pacote e wrappers externos; [bootstrap](../docs/engineering/09-windows-restart.md) mantém tudo numa raiz, sem alterar perfil/trust global.
