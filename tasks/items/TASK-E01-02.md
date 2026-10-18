# TASK-E01-02 — Provar callback e output clock

Versão: 0.7.0 · Atualização: 2026-10-18 · Estado: em execução; spike implementado e medido em Windows/WASAPI, revisão Codex/CI/integração pendentes, sem aceite antecipado.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-02 / REQ-E01-02 |
| Fase / prioridade | M0 / P0 |
| Estado | Em execução — [evidência](../../docs/evidence/PR/task-e01-02/README.md); demais hosts/hardware pendentes |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia

## Entrega concreta

Spike shared output, ring SPSC e harness de callback com instrumentation.

Documento canônico: [spec](../../docs/audio/01-engine.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Nenhum lock/alloc/I/O na callback; clock deriva consumo e device latency; underrun gera zeros sem replay.

## Verificação e evidência

QA-001, QA-003 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
