# TASK-E01-01 — Provar decode, gapless e precisão de samples

Versão: 0.7.0 · Data: 2026-10-14 · Estado: spike implementado; revisão e integração pendentes.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-01 / REQ-E01-01 |
| Fase / prioridade | M0 / P0 |
| Estado | Em progresso; prova numérica, revisão e integração pendentes |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia

## Entrega concreta

Spike de decoder registry, corpus básico e ADR-002 com versões/features propostas.

Documento canônico: [spec](../../docs/audio/01-engine.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

FLAC/MP3/WAV decodificam contra referência; delay/padding e PCM24/32 preservados quando perfil permite.

## Verificação e evidência

QA-001, QA-002, QA-004 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.

## Execução E01-01

[Plano](../../docs/superpowers/plans/2026-10-18-e01-01-decode.md), [ADR-002](../../docs/architecture/adr/ADR-002-audio-core.md), [evidência](../../docs/evidence/PR/task-e01-01/README.md) e [corpus](../../tests/fixtures/audio/decode/README.md). Adapter offline limitado; WAV24/32 e FLAC24 sample-exact; MP3 CBR/VBR comparado com referência independente e concatenação de pares sem alterar contagem. Sem metadados de trim não há inferência por silêncio. Erros/truncamento e limites/cancelamento têm testes. FLAC32, seek/state machine/playback/output e validação em hardware permanecem pendentes; QA-001/002/004 cobertos somente na parte numérica descrita. Revisão cruzada/CI/integração antes de aceite, sem anunciar capacidade de produto.
