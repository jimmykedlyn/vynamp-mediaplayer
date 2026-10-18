# Workspace Rust: contrato de implementação

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

O Cargo workspace raiz, toolchain 1.99.0 e Cargo.lock já existem para xtask. TASK-E11-01 deve adicionar crates/apps reais e resolver lockfile após provas M0. Não duplicar nem remover xtask. CI do player exige apps/desktop/Cargo.toml com package vynamp-desktop e src/main.rs reais; Android exige wrappers Unix/Windows e app/build.gradle.kts. Somente o workspace da automação não ativa esses jobs. Núcleo atende cinco targets; bridge Android adapta e não duplica queue/DSP.

## Membros reais (TASK-E01-02, 2026-10-18)

- `vynamp-audio`: spike do caminho de callback — ring SPSC pré-alocado, renderer, output clock por seqlock, estatísticas lock-free, instrumentação de alocação (`rt::CountingAllocator`) e harness simulado. Sem dependências externas.
- `vynamp-output`: conversão f32→i16/i32 e latência por timestamps; adapter shared CPAL 0.18.2 e exemplo `output_probe` somente com `--features cpal` (desligada por padrão, sem headers ALSA no CI). Não é player nem backend exclusivo/WASAPI dedicado. [Evidência](../docs/evidence/PR/task-e01-02/README.md).
