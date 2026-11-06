# Pull request — Vynamp

## Resultado

Problema e comportamento final:

O [#35](https://github.com/jimmykedlyn/vynamp-mediaplayer/pull/35) foi mergeado com **dois erros factuais** e três contradições internas na evidência. Este PR corrige os cinco pontos. Nenhuma alteração de código, dependência, workflow ou backlog; contagens seguem **191 requisitos / 191 tarefas / 19 épicos**.

Erros factuais corrigidos:

1. **[`docs/audio/10-vitrola.md`](../../../audio/10-vitrola.md) dizia que Vinyl e Tube não têm processamento no pin do MVP.** Têm: `class Vinyl` e `class Tube` em `mvp/advanced-dsp.js`, montados como `AudioWorkletNode` por `mvp/graph-audio.js`, com `real=true` em `mvp/graph-model.js` (`VynampAdvancedDSP.TYPES = ['limiter','loudness-meter','tube','vinyl','clip']`). A frase do MVP que sustentava o contrário — "Vinyl/Tube não têm adaptador" em `mvp/graph-editor.js` — é **texto que ficou para trás** no próprio MVP. Cabeçalho e §5 foram reescritos para dizer o que o código faz e separar isso do que continua **não validado**: validação acústica (VIT-T-01..10), CPU/latência, `WearState` persistido, `Context` completo e plataformas nativas — agora numa tabela própria de pendências.
2. **[`docs/design/05-dsp-panel.md`](../../../design/05-dsp-panel.md) mantinha "0 VU = −18 dBFS (−14/−20 opcionais)"** numa tabela três parágrafos abaixo do texto que declara essa convenção **inexistente** no MVP. A linha passou a dizer "Referência de 0 VU: **a definir**" e a citar a escala real, que vem por medidor em `scaleDB`/`scaleAngles` no `meta.json` (os analógicos vão de −20 a +3 dB).

Corrigido também:

- **Seção duplicada em `docs/INDEX.md`** — a rev. 1 criou duas entradas para a mesma decisão D-MVP; sobrou uma.
- **Três contradições internas da evidência** — o campo "Revisor" e a linha "quatro afirmações saíram de subagentes com estágio adversarial" diziam o oposto da seção "Limitações", e a versão de `05-decisions.md` estava registrada como 0.7.6 quando o arquivo já estava em 0.8.0 desde a E19-02.

TASK / REQ / QA:

Nenhuma. Trabalho de spec, sem TASK id — mesmo padrão de `chore-specs-vynamp-restore` e `chore-specs-shell-integration`. Não altera requisito, TASK nem marco (D-MVP-03).

Plano Superpowers e evidência de execução:

[`docs/evidence/PR/chore-specs-skin-gaps/README.md`](README.md) — método de verificação, as oito afirmações conferidas uma a uma contra o código do pin, e as limitações declaradas.

## Verificação

Comandos, resultados, ambientes/fixtures e limitações:

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, `errors: []` — 355 arquivos Markdown, 191 requisitos, 191 tarefas, 19 épicos |
| `cargo xtask check-delivery` | `result: passed`, `errors: []`, 3 arquivos de workflow |
| `cargo fmt --all --check` | sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | `No issues found` |
| `cargo test --workspace --locked` | 52 + 34 testes, 0 falhas |

Os cinco comandos foram executados sobre o estado corrigido deste branch, com o resultado acima.

Limitações declaradas:

- A verificação do MVP é **de código, não de comportamento em execução**: nenhum navegador foi aberto. "Existe" significa "o código implementa"; "não existe" significa "não há handler, constante nem CSS que o faça".
- **Nenhuma** das oito afirmações do PR original teve segunda leitura independente: o workflow de verificação foi abortado sem concluir o estágio adversarial e o subagente revisor morreu por cota (HTTP 429). Os dois erros factuais corrigidos aqui foram encontrados por releitura pós-merge, não por revisão independente.
- Medição de latência, CPU e qualidade de áudio no EQ continua não executada aqui.
- Este PR não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06, por decisão da ADR-014.

O que este PR deliberadamente **não** faz:

- **Não reescreve a matriz VU-INT-06** (144 combinações = 12 skins × 12 VUs). Atualizá-la é trabalho da TASK-E02-07; D-MVP-04 registra a divergência sem tocar na matriz.
- **Não decide** adotar "0 VU ajustável" nem "slot de 28 DIP" — se desejados, é decisão nova.
- **Não corrige `docs/product/05-decisions.md:70`** nem `10-vitrola.md:89`, que ainda afirmam não existir TASK vitrola no backlog. **TASK-E19-07 existe** desde `58e964c`; essas duas linhas estão no escopo da TASK-E19-03, que está em execução em outra worktree.

Revisão técnica (subagente Sonnet/Opus), commit revisado e achados:

- [x] Worktree e owner próprios.
- [x] Superpowers carregado; teste útil e verificação executados.
- [x] Sucesso/erro/cancelamento e acessibilidade aplicáveis cobertos — não se aplica: PR de documentação, sem comportamento de runtime.
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [ ] CI Gate (GitHub Actions) aprovado; hardware e release assinado continuam gates separados. O SHA e o run verificados não são citados aqui de propósito: este arquivo faz parte do commit, então qualquer SHA escrito nele já nasce obsoleto. O run do head é o que o GitHub mostra na aba Checks deste PR, e é ele que vale.

**Revisão independente: não realizada.** O subagente revisor (Sonnet 5-5) foi interrompido por limite de cota do plano (HTTP 429) antes de emitir relatório. Os dois erros factuais deste PR foram achados por releitura do autor contra o código do pin — mas por releitura própria, não por segundo par de olhos.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
