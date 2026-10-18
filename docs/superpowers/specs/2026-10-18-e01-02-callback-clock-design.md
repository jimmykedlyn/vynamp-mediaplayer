# TASK-E01-02 — design do spike de callback e output clock

Data: 2026-10-18 · Autor: Claude (owner TASK-E01-02) · Estado: design do spike; não é aceite.

## Intenção

Provar, antes de congelar o ADR-002, que a callback de áudio pode consumir PCM pré-decodificado sem lock, alocação, I/O ou log; que a posição apresentada deriva de frames realmente consumidos menos a latência informada pelo dispositivo; e que underrun gera zeros sem repetir amostras. Fonte: [TASK](../../../tasks/items/TASK-E01-02.md), REQ-E01-02, QA-001/QA-003, [engine](../../audio/01-engine.md), [stack](../../architecture/02-stack.md).

## Objetivos, não objetivos e anti-objetivos

- Objetivos: ring SPSC pré-alocado, renderer da callback, relógio publicado sem lock, estatísticas/instrumentação de callback (alocações e duração), harness determinístico e com threads, backend shared real via CPAL com probe medindo um dispositivo real.
- Não objetivos: decode (TASK-E01-01), generation/cancelamento/state machine (TASK-E01-03), mixer/drain completo (TASK-E01-05), resampler (TASK-E04-02), exclusivo/WASAPI dedicado, fade de recuperação de underrun, UI.
- Anti-objetivos: chamar o spike de player; aprovar hardware/targets não executados; testar apenas um espelho da implementação; Cargo.lock escrito à mão.

## Decisões (escolhas do autor, revisáveis pelo Codex)

1. **Crates** conforme [arquitetura](../../architecture/01-system.md): `crates/vynamp-audio` (sem dependências: ring, clock, renderer, stats, instrumentação RT, harness simulado) e `crates/vynamp-output` (conversão de formato e cálculo de latência puros; backend CPAL atrás da feature `cpal`, desligada por padrão).
2. **Ring SPSC próprio** (contadores monotônicos `AtomicUsize`, Acquire/Release, `Box<[UnsafeCell<f32>]>` alocado só na construção). Evita dependência para ~150 linhas auditáveis; teste de stress com threads e oráculo de rampa.
3. **CPAL 0.18.2 opcional** (Apache-2.0) para saída shared. Sem a feature, `clippy --workspace` no CI Linux não exige ALSA; o código CPAL é verificado localmente com `--features cpal` e no probe Windows. Limitação registrada: CI não compila o backend CPAL até a tarefa de CI multiplataforma (E15-04) instalar dependências de sistema.
4. **Instrumentação de alocação própria**: `CountingAllocator` (wrapper de `System`) conta alocações/desalocações apenas dentro de `rt_section`, marcada por thread-local `const`. Instalado como `#[global_allocator]` somente em testes e no probe.
5. **Relógio**: a callback publica por seqlock de escritor único `{content_frames, silence_frames, device_latency_frames, callbacks, presented_frames}`; `presented = max(anterior, content - latency)`, monotônico, nunca acima do conteúdo consumido e sem contar zeros de underrun. Leitor (UI/controle) pode repetir leitura; a callback nunca espera.
6. **Underrun x fim**: zeros após o produtor sinalizar `finish()` contam como drenagem, não underrun. Underrun incrementa contador e frames zerados; amostras nunca são reemitidas.
7. **Lock/I/O**: provados por construção (tipos atômicos apenas) e por teste de guarda de fonte nas regiões `RT-BEGIN/RT-END`, que rejeita `Mutex`, `RwLock`, `Condvar`, `println!`, `std::fs`, `std::io`, `std::net`, `Vec`/`Box`/`format!`/`String`. É guarda complementar, não substitui revisão.
8. **Probe real** (`vynamp-output`, exemplo `output_probe`, feature `cpal`): abre o dispositivo padrão shared, produz sinal de amplitude configurável (padrão 0.0, inaudível), mede alocações na callback, histograma de duração versus período, underruns, latência e relógio; grava JSON. Um modo com produtor pausado induz underrun real.
9. **CI**: `cargo xtask ci docs` passa a testar `--workspace` em vez de só `xtask`, para que os testes dos novos crates rodem nos quatro hosts de tooling.

## Coordenação com TASK-E01-01 (Codex)

Reservas E01-02: `crates/vynamp-audio/`, `crates/vynamp-output/`, `docs/evidence/PR/task-e01-02/`, este design e seu plano, entrada TASK-E01-02 em `tasks/backlog.json`, linha REQ-E01-02 em `docs/product/03-requirements.md`, `tasks/items/TASK-E01-02.md`, e a troca `--package xtask` → `--workspace` em `xtask/src/ci.rs`. Compartilhados serializados: `Cargo.toml` (uma linha de membro por TASK), `Cargo.lock` (regenerado pelo Cargo, nunca à mão; quem integrar por último resolve por `cargo metadata --locked` após merge), `VALIDATION.json` (regenerado por `ci docs`). ADR-002 fica com a E01-01; a E01-02 entrega sua proposta de saída na evidência para o congelamento. Registro no painel do guia externo.

## O que fica para o construtor

Tamanho de buckets do histograma, formato exato do JSON do probe, nomes internos. Hardware além do Windows local permanece pendente.
