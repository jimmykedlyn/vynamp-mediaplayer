# ADR-002 — núcleo Rust e backends por capacidade

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: proposto; congelar após TASK-E01-01/TASK-E01-02.

Decisão: domain/engine/control em Rust, decode e output por adapters; RT callback restrita; caminho transparente inteiro separado de DSP float. Symphonia/CPAL candidatos, WASAPI avançado adapter dedicado se necessário. Alternativa: wrapper de playback de alto nível; custo de ocultar clock/gapless/negotiation. Consequência: mais trabalho de engine, mas critérios verificáveis; bibliotecas nativas admitidas com isolation/license.

## Proposta de decode após E01-01 (2026-10-18)

Spike offline [vynamp-decode](../../../crates/vynamp-decode/Cargo.toml): Symphonia **=0.6.1**, `default-features=false`, features `flac`, `mp3`, `pcm`, `wav`. Cargo.lock resolvido pelo Cargo, sem dependência FFmpeg de runtime e sem SIMD/features extras habilitadas incidentalmente. Nesta versão, gapless é opção do **decoder**: `AudioDecoderOptions::default().gapless(true).verify(true)`, não `FormatOptions::enable_gapless` da API 0.5. Demuxer fornece delay/padding; decoder aplica uma vez, adapter não faz outro trim nem remove silêncio.

Inteiros são copiados para i32 MSB-aligned sem passagem por float (PCM24 com oito LSBs zero, PCM32 preserva todos os bits). MP3 permanece f32 e não ganha alegação de origem hi-res. Registry experimental limita o adapter a WAV signed LE24/32, FLAC nativo PCM24 e MPEG Layer III; não é RuntimeCapabilities/associações do produto. Não herdar suporte de codecs apenas compilados pela biblioteca.

[Corpus e evidência](../../evidence/PR/task-e01-01/README.md): pares de 4417 frames estéreo 44.1kHz em WAV24/WAV32/FLAC24 e MP3 CBR/VBR, referências independentes FFmpeg. Nesta prova, MP3 exige delay/padding e contagem declarados pelo demuxer Xing/LAME. Sem essas informações o adapter retorna Unsupported: EOF de um corpo MP3 incompleto pode virar fim normal na biblioteca e contagem estimada de VBR não é oráculo exato. Fixtures sem Xing, curta CBR e VBR com mais de17 frames MPEG, provam a recusa, sem remover silêncio ou devolver PCM parcial. Suporte futuro sem metadata exige delimitação/integridade e duração confiáveis independentes do estimador. PCM32 em FLAC não é aprovado: o encoder FFmpeg 9.0.2 usado reduziu a precisão a 24 bits; a verificação contra fonte rejeitou essa tentativa. O adapter recusa FLAC32 até corpus independente apropriado (M2), em vez de anunciar suporte sem prova.

Alternativas: FFmpeg de runtime amplia cobertura mas acrescenta distribuição/configuração/licenciamento; wrappers de playback escondem trim e precisão. Para este spike, escolher Symphonia mínima torna esses contratos observáveis com referência diferente. MPL-2.0 e dependências constam do [inventário](../../evidence/PR/task-e01-01/dependencies.json); não é decisão de licença final do projeto nem liberação de distribuição.

Proposta ainda depende de revisão Claude do SHA e integração. Output/CPAL, callback e clocks da E01-02 mantêm sua própria evidência; este registro não congela ou aprova seu backend. Decode em workers pequenos não aprova streaming, seek/player, PCM24/32 em DAC, soak ou os cinco targets. Flags de produto/release continuam false.
