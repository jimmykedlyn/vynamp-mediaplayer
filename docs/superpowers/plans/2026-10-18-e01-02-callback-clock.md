# TASK-E01-02 Callback e Output Clock Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Spike com ring SPSC, renderer de callback, relógio de saída e instrumentação provando REQ-E01-02, mais probe CPAL shared em dispositivo real.

**Architecture:** `vynamp-audio` (sem dependências) contém o caminho RT e harness; `vynamp-output` contém conversão/latência puras e backend CPAL opcional (`feature = "cpal"`) com exemplo `output_probe`.

**Tech Stack:** Rust 1.99.0, std atomics, CPAL 0.18.2 (opcional).

**Spec:** [design](../specs/2026-10-18-e01-02-callback-clock-design.md)

**Execução:** inline pelo owner (Claude), conforme guia; revisão técnica cruzada pelo Codex no SHA final.

## Global Constraints

- Callback: sem `Mutex`/`RwLock`/`Condvar`, alocação, filesystem/rede/stdio, log; só atômicos e buffers pré-alocados.
- Relógio em frames inteiros (`u64`), não f32/ms.
- Underrun zera o restante do bloco, nunca repete amostras, publica contador.
- Gate de duração: p99.9 da callback < 25% do período negociado (medido, não presumido).
- `cargo xtask ci docs` verde; `Cargo.lock` só gerado pelo Cargo; flags de prontidão continuam false.

## Review Focus

1. Buffer de saída com tamanho não múltiplo de canais → renderer não pode desalinhar canais (teste `partial_frame_tail_is_zeroed`).
2. Latência maior que conteúdo consumido (início) → relógio 0, sem underflow (teste `latency_larger_than_content_clamps_to_zero`).
3. Latência que diminui entre callbacks → posição apresentada não regride (teste `presented_is_monotonic_when_latency_drops`).
4. Produtor mais rápido que capacidade → `push` parcial, nunca sobrescreve não lido (teste `push_never_overwrites_unread`).
5. Buffer do dispositivo maior que scratch de conversão → zeros + contador, sem alocar (teste `oversized_device_buffer_zero_fills`).

---

### Task 1: Ring SPSC (`crates/vynamp-audio/src/ring.rs`)

**Interfaces — Produces:** `ring::channel(capacity_samples: usize) -> (Producer, Consumer)`; `Producer::push(&mut self, &[f32]) -> usize`, `Producer::finish(&self)`, `Producer::free(&self) -> usize`; `Consumer::pop_into(&mut self, &mut [f32]) -> usize`, `Consumer::is_finished(&self) -> bool`, `Consumer::available(&self) -> usize`.

- [ ] Testes: `push_pop_roundtrip_preserves_order`, `push_never_overwrites_unread`, `wraparound_keeps_ramp_contiguous`, `threaded_ramp_has_no_loss_or_duplicate` (1M amostras, oráculo de rampa), `finish_is_visible_after_data`. Rodar → FAIL (módulo ausente).
- [ ] Implementar com contadores monotônicos `head/tail: AtomicUsize` (Release no publish, Acquire no read), `Box<[UnsafeCell<f32>]>`; `unsafe impl Send` documentando a invariante SPSC.
- [ ] `cargo test -p vynamp-audio` → PASS; commit `feat(audio): pre-allocated SPSC ring (TASK-E01-02)`. Inclui Cargo.toml membro.

### Task 2: Clock e stats (`clock.rs`, `stats.rs`)

**Produces:** `clock::clock() -> (ClockWriter, ClockReader)`; `ClockWriter::publish(&mut self, content_delta: u64, silence_delta: u64, latency_frames: u64)`; `ClockReader::snapshot(&self) -> ClockSnapshot { content_frames, silence_frames, latency_frames, presented_frames, callbacks }`. `stats::CallbackStats::new(period_ns: u64) -> Arc<Self>`, `record(duration_ns)`, `record_underrun(frames)`, `report() -> StatsReport { callbacks, underruns, underrun_frames, drained_frames, max_ns, p999_ns, period_ns }`.

- [ ] Testes: `presented_subtracts_latency`, `silence_does_not_advance_presented`, `latency_larger_than_content_clamps_to_zero`, `presented_is_monotonic_when_latency_drops`, `concurrent_reader_sees_consistent_snapshot` (invariante `presented <= content` sob escrita contínua), `p999_bucket_bounds_true_value`.
- [ ] Implementar seqlock de escritor único (`seq: AtomicU64`, campos `AtomicU64`, fences); histograma log2 com 64 buckets `AtomicU64`, p99.9 reporta o limite superior do bucket.
- [ ] PASS; commit.

### Task 3: Renderer + instrumentação RT (`callback.rs`, `rt.rs`)

**Produces:** `Renderer::new(consumer: Consumer, clock: ClockWriter, stats: Arc<CallbackStats>, channels: u16) -> Renderer`; `Renderer::render(&mut self, out: &mut [f32], latency_frames: u64) -> RenderOutcome { content_frames, silence_frames, underrun: bool }`. `rt::CountingAllocator`, `rt::rt_section<R>(f: impl FnOnce() -> R) -> R`, `rt::rt_alloc_events() -> u64`.

- [ ] Testes: `underrun_zero_fills_without_replay` (rampa → zeros → próxima amostra é a seguinte da rampa), `drain_after_finish_is_not_underrun`, `partial_frame_tail_is_zeroed`, `render_updates_clock_with_device_latency`; `tests/rt_alloc.rs` com `#[global_allocator]`: `render_allocates_nothing` (10k callbacks, `rt_alloc_events()==0`) e controle positivo `counter_detects_allocation_inside_section`; `tests/rt_source_guard.rs` rejeita tokens proibidos entre `RT-BEGIN`/`RT-END` e exige que cada arquivo RT tenha marcadores.
- [ ] Implementar; render mede duração com `Instant` e registra em stats.
- [ ] PASS; commit.

### Task 4: Harness simulado (`harness.rs`)

**Produces:** `harness::SimConfig { sample_rate, channels, period_frames, latency_frames, callbacks, producer_stall: Option<(u64,u64)> }`, `harness::run(cfg) -> SimReport { stats, clock, max_ramp_gap, duplicates, rt_allocs }`.

- [ ] Testes: `steady_run_has_no_underrun`, `stalled_producer_underruns_without_replay` (duplicates==0, conteúdo consumido == produzido ao final).
- [ ] Implementar device thread com período real (`sleep`) e produtor thread com rampa.
- [ ] PASS; commit.

### Task 5: `vynamp-output` + probe CPAL

**Produces:** `convert::write_i16/write_i32/write_f32(src: &[f32], dst: &mut [T])` (clamp, sem alocar); `latency::frames_between(callback: Duration, playback: Duration, rate: u32) -> u64`; feature `cpal`: `shared::SharedOutput::open(cfg) -> Result<SharedOutput>` e `examples/output_probe.rs` (args `--seconds`, `--amplitude`, `--stall-ms`, `--out <json>`).

- [ ] Testes: `i16_conversion_clamps_and_rounds`, `i32_conversion_full_scale`, `latency_frames_from_timestamps`, `negative_latency_is_zero`, `oversized_device_buffer_zero_fills`.
- [ ] Implementar; `cargo clippy -p vynamp-output --features cpal --all-targets -- -D warnings` local.
- [ ] Rodar probe 30 s normal e com stall; salvar JSON em `docs/evidence/PR/task-e01-02/`; commit.

### Task 6: CI, docs, evidência, handoff

- [ ] `xtask/src/ci.rs`: `test --workspace`; `crates/README.md`, TASK/backlog/REQ status `in_progress`, evidência README com comandos/resultados/ambiente/limites.
- [ ] `cargo xtask ci docs` (regenera VALIDATION.json), `git diff --check`; commit; push; PR draft para develop; atualizar guia (bloco REVISAR enviar ao Codex + painel).
