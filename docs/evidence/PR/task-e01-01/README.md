# E01-01 — evidência de decode offline

Autor Codex, 2026-10-18. Branch `feature/task-e01-01-codex-decode`, base `8024bdc2363d9d9b489a3b0d553fb59d2b127703`; worktree `D:\Projects\vynamp-media-player\repo\.worktrees\codex\task-e01-01-decode`, claim própria e índice separado. Este snapshot não contém seu próprio SHA futuro; SHA/PR/CI posterior e revisão Claude ficam no guia externo e PR. Nada aqui atribui checks ao Claude.

## Escopo e proveniência

Spike offline em Rust 1.99.0, Symphonia =0.6.1 sem default features, somente flac/mp3/pcm/wav. [Plano](../../../superpowers/plans/2026-10-18-e01-01-decode.md), [ADR-002](../../../architecture/adr/ADR-002-audio-core.md), [corpus](../../../../tests/fixtures/audio/decode/README.md), [manifesto](../../../../tests/fixtures/manifests/decode.json) e [schema](../../../../tests/fixtures/manifests/decode.schema.json). Doze entradas (dez positivas e duas negativas), fontes inteiras e referências independentes; SHA256/tamanho de cada binário conferidos pelos testes. FFmpeg não integra o runtime; binário de referência baixado para scratch da própria worktree, hash do ZIP `60f467265b1e312373dbcd92200c2618a74850f98d3d078e94296bb3fa2047ba` conferido contra o publicado pelo fornecedor Gyan, linkado pelo site oficial FFmpeg. Build/config/hash do executável no manifesto. Origem sintética/permissão de dados registrada, sem material Winamp ou de terceiros.

Discovery nesta sessão: skill using-superpowers 6.4.2 disponível no catálogo e lida; skills de planejamento, isolamento, execução, TDD, debugging, verificação e revisão consultadas. `cargo xtask doctor --agent both` exit0 confirmou managers nativos conforme lock (Codex6.4.2/Claude7.0.0); isso não confirma discovery da sessão Claude. Worktree existente reutilizada, superproject vazio, claim codex/TASK-E01-01/develop e índice `.git/worktrees/task-e01-01-decode/index` conferidos. Baseline ci docs exit0: 39 unit +39 tooling; CI pós-merge39152762151 consultado com wrapper autenticado, completed/success no SHA base. Primeira tentativa doctor no sandbox falhou LNK1104 no TMP externo; repetição autorizada passou, sem mudança de permissões/pins. Consulta gh sem wrapper retornou404; wrapper do projeto confirmou acesso, sem novo login.

Coordenação lida no guia: E01-02 reserva vynamp-audio/vynamp-output e xtask/src/ci.rs. Nenhum desses arquivos editado. E01-01 reserva vynamp-decode/corpus/ADR002; membros/lock nas próprias branches com merge e regeneração pela segunda integração. CI E01-01 acrescenta teste explícito do decoder aos quatro runners tooling, sem depender de app ou editar ci.rs reservado. Guia atualizado sob FileShare.None, preservando registros E01-02. Sua entrega PR14 não foi revisada nesta tarefa.

## Resultados numéricos do autor

| Prova | Resultado observado | Limite |
|---|---|---|
| WAV24, WAV32, FLAC24, partes a/b | 4417 frames por arquivo, 2 canais/44100Hz; todos samples inteiros idênticos ao FFmpeg e à fonte anterior ao encoder | FLAC32 não provado |
| MP3 CBR192k a/b | 4417 frames; delay1105/padding238; erros máximos 5.5134296e-7 / 5.066395e-7 FS | Decoder float, não lossless |
| MP3 VBR q2 a/b | 4417 frames; delay1105/padding238; erros máximos 6.2584877e-7 / 8.046627e-7 FS | Tolerância pré-fixada2e-5 FS, sem alinhar/normalizar referência |
| MP3 sem Xing/LAME | Recusa Unsupported para CBR curto e VBR longo; truncamentos não retornam PCM | Referências FFmpeg preservadas; sem inferência por silêncio/contagem estimada |
| Pares WAV24/WAV32/FLAC24/MP3 CBR/VBR | 8834 frames na concatenação, contra referências concatenadas | Buffers offline; não playback, callback ou gapless em dispositivo |
| Precisão PCM32 | MIN/MAX/-1/+1 e demais LSBs preservados sem float | Não aprova captura transparente de DAC |
| Erro/cancelamento/limites | Geração obsoleta/restart, limites bytes/frames/rate/canais, inválido/truncado WAV/FLAC/MP3 cobertos | Não é fuzz, isolamento de parser ou prova de cancelamento durante I/O |

RED real: 2 testes de decode/pares falharam por Unsupported com adapter ainda não implementado; hash/proveniência já passavam. Depois decode/pares passaram. Registry vazio falhou (0 vs3), quatro testes adversariais já passaram por comportamento implementado no primeiro slice; esses quatro não são alegados RED próprio. Logs originais preservados em `decode-red.txt` e `registry-red.txt`. Revisão interna Codex somente leitura encontrou EOF parcial e estimativa de duração em MP3 sem Xing; regressão `mp3_without_trustworthy_trim_is_rejected_instead_of_partial_success` falhou por retorno Ok antes de restringir o adapter (`no-xing-red.txt`). Decisão: perfil sem metadados explícitos Unsupported, documentado no ADR; suporte amplo fica pendente. Schema é caracterização adicional, não regressão com RED observado. Clippy inicial rejeitou chunks_exact constante; migração para as_chunks conforme Rust1.99, sem alterar oráculo. Captura ci docs intermediária falhou por fmt após mudanças de revisão; substituída por captura final após fmt, sem ocultar a tentativa.

## Checks finais

Capturas finais próprias, todas exit0, após correção da revisão interna:

- [Ambiente](environment.txt): Windows26300 x86_64, PowerShell7.4.20, Rust1.99.0/MSVC, branch e HEAD da base no momento da captura.
- [Doctor](doctor.txt): `cargo xtask doctor --agent both`; managers confirmados, discovery de sessão apenas Codex.
- [Decode](decode-tests.txt): `cargo test -p vynamp-decode --locked -- --nocapture`; 6 adversariais+3 corpus, hashes/schema/12 fixtures, números por formato.
- [Gates](ci-docs.txt): `cargo xtask ci docs`; docs/config/fmt/clippy --workspace all-targets, 39 unit+39 tooling; VALIDATION308 Markdown/176 TASK/REQ, errors vazio.
- [Workspace](workspace-tests.txt): `cargo test --workspace --locked`; 87 testes passaram (9 decoder+39 unitários xtask+39 tooling), zero falhas; doc-tests sem exemplos.
- [Detect](detect.txt): `cargo xtask ci detect`; core=false/android=false.
- [Diff](diff-check.txt): `git diff --check`; exit0. Após staging, diff --cached --check também é conferido no handoff.

[Dependências](dependencies.json) geradas de metadata --locked; `.gitattributes` confirmou text/diff unset nos binários. `git diff --exit-code -- config/project.json rust-toolchain.toml xtask/src/ci.rs` exit0: flags/pin/arquivo reservado intactos. CI remoto e revisão do SHA final ainda pendentes neste snapshot. [Notices](NOTICES.md) e inventário normal/runtime não equivalem à autorização de release. Resultados posteriores ao commit ficam no guia externo/PR; não interpretar evidência capturada na base como teste de SHA futuro.

## Limitações e gates

QA-002 cobre contagens/concatenação/trim de buffers desta amostra; QA-004 cobre precisão inteira offline. QA-001 somente cancelamento/restart de worker; state machine, seek e comandos playback não implementados. Sem testes de DAC/output, transparência end-to-end, device clock, soak, performance, Linux/MacIntel/MacSilicon/Android local; CI nativo hospedado será registro distinto e não hardware. AAC/rates/perfis fora do corpus permanecem sem prova. M3 demanda todos os alvos e corpus amplo. FLAC32 tentado com FFmpeg reduziu precisão a24bits e falhou comparação com fonte; tentativa não foi incluída como sucesso e adapter recusa esse perfil. Não é limitação geral declarada da biblioteca sem prova.

Limites do adapter restringem arquivo já carregado, frames devolvidos e parâmetros antes de criar decoder; alocações internas do demuxer/decoder não são isoladas nem interrompidas no meio de um packet. Cancelamento cooperativo é checado antes/depois de probe/packets e antes do retorno; consumidor deve revalidar geração ao publicar. Mudança de configuração é rejeitada pelo adapter, mas corpus não contém fixture real de mudança durante stream; NaN/Inf têm guard sem fixture MP3 que produza esse caso. EOF sem duração confiável não prova integridade de todo arquivo adversarial. Sem promises de segurança de parser ou player.

Flags application_ready/release_enabled/branding_exports_ready continuam false, sem UI/app/installer fictícios. TASK/backlog in_progress até revisão cruzada/CI/integração. Fonte final do projeto ainda sem licença escolhida. Não foi executado merge, release, bypass, force push nem edição da worktree Claude.
