# Pull request — Vynamp

## Resultado

Problema e comportamento final:

O mantenedor decidiu, em 2026-11-05, que os dois arquivos de skin da raiz (`SKIN-REQUIREMENTS-CODEX…` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2`) **não são fonte de requisito**. A UI nativa se baseia no MVP web (`vynamp-mvp-reference`). O escopo original era o inverso — absorver as "lacunas" dos SKIN nas specs; com a decisão não há lacuna a herdar.

Este PR registra a decisão e limpa as citações que ainda davam autoridade aos dois documentos. Nenhuma alteração de código, dependência, workflow ou backlog. Contagens seguem **191 requisitos / 191 tarefas / 19 épicos**.

A decisão registrada em [`docs/product/05-decisions.md`](../../../product/05-decisions.md):

- **D-MVP-01** — o MVP web é a fonte única da UI nativa; os SKIN não são fonte de requisito.
- **D-MVP-02** — só herda o comportamento que o código do MVP prova; o que os SKIN afirmavam e o MVP não tem **não** entra por herança.
- **D-MVP-03** — nenhuma TASK, REQ ou marco nasce dos SKIN.
- **D-MVP-04** — onde o repositório e o MVP divergem **em número**, vale o MVP. Caso atual: VU-INT fala em 12 medidores (número da rev.15) e o pin rev.38 tem **16, todos selecionáveis**.

Duas tabelas acompanham: as divergências encerradas por esta decisão, e os seis comportamentos que os SKIN atribuíam ao produto e o MVP não tem.

Nota de propriedade factual: os SKIN eram **consumidores** deste repositório — seus próprios changelogs citavam `DSP-UX`, `VU-INT` e `LYR` como documentos canônicos daqui. Manter as duas fontes produzia duas especificações para o mesmo comportamento.

TASK / REQ / QA:

Nenhuma. Trabalho de spec, sem TASK id — mesmo padrão de `chore-specs-vynamp-restore` e `chore-specs-shell-integration`. Não altera requisito, TASK nem marco (D-MVP-03).

Plano Superpowers e evidência de execução:

[`docs/evidence/PR/chore-specs-skin-gaps/README.md`](README.md) — método de verificação, as oito afirmações conferidas uma a uma contra o código do pin, e as limitações declaradas.

## Verificação

Comandos, resultados, ambientes/fixtures e limitações:

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, `errors: []` — 355 arquivos Markdown, 191 requisitos, 191 tarefas, 19 épicos |
| `cargo xtask check-delivery` | `result: passed`, `errors: []` |
| `cargo fmt --all --check` | sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | `No issues found` |
| `cargo test --workspace --locked` | 52 + 34 testes, 0 falhas |

Método: oito afirmações atribuídas aos SKIN foram conferidas contra o código do MVP em `vynamp-mvp-reference@0d9860b` (rev.38) — **as oito diretamente pelo autor**, lendo o arquivo do pin citado em cada linha. Um workflow de verificação por subagentes chegou a rodar sobre quatro delas, mas foi abortado sem concluir o estágio adversarial, então nenhuma teve segunda leitura independente. O resultado por afirmação — inclusive as três que **não** existem no MVP ("0 VU = −18 dBFS ajustável", "slot de 28 DIP", "intensidade 0 = bypass da seção") e as que **divergem** (presets da Vitrola; atalho no inspector) — está na evidência.

Limitações declaradas:

- A verificação do MVP é **de código, não de comportamento em execução**: nenhum navegador foi aberto. "Existe" significa "o código implementa"; "não existe" significa "não há handler, constante nem CSS que o faça".
- **Nenhuma** das oito afirmações teve segunda leitura independente: o workflow de verificação foi abortado sem concluir o estágio adversarial e o subagente revisor morreu por cota (HTTP 429) antes de emitir relatório.
- Medição de latência, CPU e qualidade de áudio no EQ continua não executada aqui.
- Este PR não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06, por decisão da ADR-014.

O que este PR deliberadamente **não** faz:

- **Não reescreve a matriz VU-INT-06** (144 combinações = 12 skins × 12 VUs). Atualizá-la é trabalho da TASK-E02-07; D-MVP-04 registra a divergência sem tocar na matriz.
- **Não decide** adotar "0 VU ajustável" nem "slot de 28 DIP" — se desejados, é decisão nova.
- **Não altera a evidência histórica** `chore-dsp-eq-docs/summary.md`, que registra o que foi feito naquele PR quando os SKIN ainda existiam.

Revisão técnica (subagente Sonnet/Opus), commit revisado e achados:

- [x] Worktree e owner próprios.
- [x] Superpowers carregado; teste útil e verificação executados.
- [x] Sucesso/erro/cancelamento e acessibilidade aplicáveis cobertos — não se aplica: PR de documentação, sem comportamento de runtime.
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [ ] CI Gate (GitHub Actions) aprovado; hardware e release assinado continuam gates separados — CI deste PR em execução.

**Revisão independente: não realizada.** O subagente revisor (Sonnet 5-5) foi interrompido por limite de cota do plano (HTTP 429, reset em ~74 h) antes de emitir relatório. As oito afirmações atribuídas ao MVP foram **reverificadas diretamente contra o código** de `vynamp-mvp-reference@0d9860b` por este autor, e foi assim que um erro próprio foi encontrado e corrigido: a §6 do painel DSP dizia que **Shift+V** abre o menu de modelos, mas em `mvp/player.js:224` o modificador de comando é `ctrlKey || metaKey` — **Ctrl+Shift+V** abre o menu, **Shift+V** recua o ciclo de medidores e **Ctrl+V** sozinho não faz nada. Não houve, portanto, segundo par de olhos sobre a redação.

### Rev. 2 — correções depois do merge do #35

O #35 foi mergeado com dois erros factuais que a revisão pós-merge encontrou. Ambos estão corrigidos nos commits seguintes a este:

1. **`docs/audio/10-vitrola.md` dizia que Vinyl e Tube não têm processamento no pin.** Têm: `class Vinyl` e `class Tube` em `mvp/advanced-dsp.js`, montados como `AudioWorkletNode` por `mvp/graph-audio.js`, com `real=true` em `mvp/graph-model.js`. A frase do MVP que dizia o contrário (`graph-editor.js`: "Vinyl/Tube não têm adaptador") é texto que ficou para trás. O cabeçalho e a §5 foram reescritos para dizer o que o código faz e separar isso do que continua **não validado** (acústica, CPU/latência, WearState persistido, `Context` completo, plataformas nativas).
2. **`docs/design/05-dsp-panel.md` mantinha "0 VU = −18 dBFS (−14/−20 opcionais)"** numa tabela três parágrafos abaixo do texto que declara essa convenção inexistente no MVP. A linha agora diz "Referência de 0 VU: **a definir**" e cita a escala real, que vem por medidor em `scaleDB`/`scaleAngles` no `meta.json` (os analógicos vão de −20 a +3 dB).

Também corrigidos: uma seção duplicada no `docs/INDEX.md` (duas entradas para a mesma decisão D-MVP) e três contradições internas da evidência — o campo "Revisor" e a linha "quatro afirmações saíram de subagentes com estágio adversarial" diziam o oposto da seção "Limitações", e a versão de `05-decisions.md` estava registrada como 0.7.6 quando o arquivo já estava em 0.8.0 desde a E19-02.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
