# TASK-E01-02 — callback e output clock

Autor: Claude (owner), 2026-10-18. Branch `feature/task-e01-02-claude-callback`, worktree `D:\Projects\vynamp-media-player\repo\.worktrees\claude\task-e01-02-callback`, base `8024bdc2363d9d9b489a3b0d553fb59d2b127703` (merge PR12, develop). Este snapshot não contém o próprio SHA final; head/PR/revisão ficam no guia externo e no PR. Spike de M0: não é player, não altera flags de prontidão e não aceita a TASK sem revisão Codex, CI e integração.

## Descoberta, isolamento e baseline

- Skills Superpowers 7.0.0 carregadas nesta sessão: using-superpowers, brainstorming, writing-plans, test-driven-development (e verification-before-completion antes do handoff). Design: [spec](../../../superpowers/specs/2026-10-18-e01-02-callback-clock-design.md); plano: [plano](../../../superpowers/plans/2026-10-18-e01-02-callback-clock.md). Execução inline pelo owner; nenhum subagente.
- Isolamento conferido antes de editar: HEAD=origin/develop=`8024bdc…` após `git fetch`, árvore limpa, git-dir `.git/worktrees/task-e01-02-callback` distinto do common dir, claim `.git/auralis/claims/TASK-E01-02/owner.json` = claude/TASK-E01-02/branch/path esperados. Worktree E01-01 (Codex) apenas inspecionada em leitura (limpa, sem commits). Reservas de arquivos registradas no painel do guia externo.
- Dependência TASK-E11-01: PR11 integrado em develop (registro do guia); base contém o merge.
- CI pós-merge da base: run 39152762151, `completed/success`, head `8024bdc…` (consulta `gh run view`).
- Baseline próprio antes de qualquer arquivo de feature, Windows, `CARGO_TARGET_DIR` da worktree: `cargo xtask doctor --agent both` exit 0 (managers confirmam Superpowers para codex e claude); `cargo xtask ci docs` exit 0 (spec/delivery sem erros, fmt, clippy, 39 unitários + 39 integração do xtask).

## Entrega

| Requisito / aceite | Implementação | Prova |
|---|---|---|
| Ring SPSC pré-alocado | `crates/vynamp-audio/src/ring.rs`: contadores `u64` monotônicos, Acquire/Release, alocação só em `channel` | `tests/ring.rs`: ordem, não sobrescreve não lido, wraparound, 1 Mi amostras entre threads com oráculo de rampa |
| Nenhum lock/alloc/I/O/log na callback | Regiões `RT-BEGIN/RT-END` só com atômicos e slices; `rt::CountingAllocator` conta heap dentro de `rt_section` | `tests/rt_alloc.rs` (10 000 renders com underruns, 0 eventos; controle positivo detecta alocação), `vynamp-output/tests/rt_alloc.rs` (i16/i32, 0 eventos), `tests/rt_source_guard.rs` (tokens proibidos em 6 arquivos RT); probe real: 0 eventos |
| Clock deriva consumo e device latency | `clock.rs`: seqlock de escritor único; apresentada = conteúdo escrito antes da callback − latência do primeiro frame do buffer; monotônica; silêncio não avança | `tests/clock.rs`, `tests/callback.rs`; probe real: latência WASAPI (`IAudioClock` + `GetStreamLatency` via CPAL) 12 304–12 821 frames, taxa do relógio 1,00017× nominal sem stall, 0 regressões |
| Underrun gera zeros sem replay | `callback.rs`: só frames completos, zera o restante, conta underrun; após `finish` conta drenagem | `tests/callback.rs`, `tests/harness.rs` (thread de dispositivo com período real + produtor com stall de 200 ms); probe real com stall de 300 ms |
| Harness com instrumentation | `harness.rs`, `stats.rs` (histograma log2, p99.9 como limite superior do bucket), `output_probe` | Seções abaixo |
| Saída shared (QA-003: cadeia real) | `vynamp-output::shared` (feature `cpal`), formato f32/i16/i32, conversão em blocos para buffers maiores que o scratch | Probe registra host/dispositivo/taxa/canais/formato/buffer negociados |

TDD: cada slice começou por testes que falharam pela ausência do módulo (erros `unresolved import`), depois verdes. A correção de semântica da latência (commit `fix(audio)`) foi guiada por testes reescritos que falharam antes da mudança (2 em `clock`, 1 em `callback`). Mutações temporárias, revertidas, confirmaram que os testes detectam: seqlock sem verificação de sequência (`torn snapshot`), alocação inserida no render (`render path allocated`), remoção do zero-fill (testes de callback e harness: `missing frame is silence`, `loss or replay detected`). O guard estático não detectava `Box::<u8>::new`; o token passou a ser `Box::`.

## Medições em hardware real (Windows, WASAPI shared)

Ambiente: [environment.txt](environment.txt) — Windows 11 build 26300 x64, Intel i5-8350U, Rust 1.99.0 MSVC, dispositivo padrão `Headset (USB 384KZ HIFI Audio)`, 384 000 Hz, 2 canais, f32, buffer anunciado 3 840 frames (callbacks observadas de 3 840 a 8 448 frames). Comando (release): `cargo run -p vynamp-output --features cpal --example output_probe --release --locked -- --seconds 30 [--stall-at 10 --stall-ms 300] --out <json>`. Amplitude 0.0: frames de conteúdo reais com silêncio digital (inaudível).

| Métrica | [Sem stall](probe-steady.json) | [Stall 300 ms em t=10 s](probe-stall.json) |
|---|---|---|
| Produzido / consumido como conteúdo | 11 520 000 / 11 520 000 | 11 520 000 / 11 520 000 |
| Underruns / frames zerados | 0 / 0 | 25 / 94 976 (≈247 ms) |
| Eventos de heap na callback | 0 | 0 |
| Callbacks do dispositivo | 3 022 | 3 052 |
| Duração máx. / p99.9 (limite do bucket) | 168,5 µs / ≤131,1 µs | 102,2 µs / ≤131,1 µs |
| p99.9 / menor período (10 ms) | 0,0131 (gate < 0,25) | 0,0131 |
| Latência do dispositivo (frames) | 12 393–12 800 | 12 304–12 802 |
| Taxa do relógio / nominal | 1,00017 | 0,99163 (silêncio de underrun excluído: ≈1 − 0,247/29) |
| Regressões do relógio / erros do stream | 0 / 0 | 0 / 0 |

SHA-256: `probe-steady.json` A3621CC778AEFB278F4D013576CDEF808724B1A6B5F4097FBC7EBF022E38931C; `probe-stall.json` 4844D2FD49FD65EC35188CD154EE44C294D519E55DBDDD3207953C124D3B4F01. stderr: [steady](probe-steady.stderr.txt), [stall](probe-stall.stderr.txt).

## Checks do autor

Comandos, todos na worktree, Windows, exit 0 salvo indicação:

- `cargo test --workspace --exclude xtask --locked`: 34 testes dos novos crates, 0 falhas (harness repetido 3× sem flutuação; `rt_alloc` 40× paralelo e 40× serial antes da correção abaixo, sem reproduzir a corrida localmente).
- `cargo clippy --workspace --all-targets --locked -- -D warnings` e `cargo clippy -p vynamp-output --features cpal --all-targets --locked -- -D warnings`.
- `cargo fmt --all -- --check`, `git diff --check`.
- `cargo xtask ci docs` final: [ci-docs.txt](ci-docs.txt), exit 0 — validação sem erros, fmt/clippy, `cargo test --workspace` (34 testes dos novos crates + 39 unitários + 39 integração do xtask); regenera `VALIDATION.json`. A tentativa anterior falhou (exit 1) somente por `Missing index` dos três documentos novos; corrigido indexando-os em `docs/INDEX.md`.
- Probes de hardware acima.

## Proposta para o congelamento do ADR-002 (ADR reservado à E01-01)

Saída shared via CPAL 0.18.2 (Apache-2.0) como adapter atrás de feature; callback com ring SPSC próprio, sem dependência; relógio publicado pela callback e latência do primeiro frame vinda do backend; allocator de contagem como gate RT em testes/probes. O contrato do backend é `render(&mut [f32], latency_frames)`; WASAPI exclusivo/ASIO continuam adapters futuros.

## Limitações e pendências

- Hardware medido: somente este Windows x64 com um DAC USB em shared mode. Linux (ALSA/PipeWire), macOS Intel/Apple Silicon e Android ficam pendentes; CI não compila o adapter CPAL (feature desligada, sem headers ALSA). Execução de 8 h e carga de CPU do gate não foram feitas.
- A instrumentação de alocação cobre o código Vynamp dentro do closure; heap de CPAL/OS ao redor não é medido. Lock/I/O são provados por construção e guard estático complementar, não por interceptação do OS.
- `stats` do renderer contam blocos de conversão; o probe usa a duração da callback inteira (`device_callbacks`).
- Relógio é amostrado no instante da callback; interpolação entre callbacks, generation/cancelamento (E01-03), fade de recuperação de underrun, mixer/drain completo (E01-05) e resampler (E04-02) estão fora do escopo.
- `Cargo.lock` ganhou 51 pacotes resolvidos para todas as plataformas do CPAL; licenças/inventário transitivo pertencem à TASK-E11-03. Flags `application_ready`/`release_enabled` permanecem false.

## Incidente de CI e correção

CI [39156053036](https://github.com/jimmykedlyn/vynamp-mediaplayer/actions/runs/39156053036) do head `c0a5cbb` falhou só em `tooling (windows-2022)`: `render_allocates_nothing` contou 2 eventos. Causa raiz: o teste media o contador global de processo, e `counter_detects_allocation_inside_section`, rodando em paralelo no mesmo binário, fez exatamente um alloc+dealloc dentro da própria seção. Era corrida de medição entre testes, não alocação do render (os 40+40 reruns locais não reproduziram). Correção: o contador por thread `rt::thread_rt_alloc_events()` passou a ser usado nos testes e no harness, onde a callback roda na própria thread de medição. O contador global continua no probe, onde só a thread CPAL entra em seção. Regressão `thread_counter_ignores_other_threads_sections` (RED por API ausente e depois verde); uma mutação com alocação no render continua detectada (`render path allocated` e harness com 202 eventos).
