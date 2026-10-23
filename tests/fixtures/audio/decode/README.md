# Corpus sintético de decode E01-01

[Inventário](../../manifests/decode.json), [schema Draft2020-12](../../manifests/decode.schema.json) e [gerador](generate.ps1). `cargo test -p vynamp-decode --locked` valida schema, hashes/tamanhos e resultados sem FFmpeg instalado ou rede. Todos os arquivos binários são marcados binary em .gitattributes.

Origem: sinais sintéticos gerados para estes testes, sem áudio/código/assets de terceiros. A permissão de redistribuir/transformar estes dados de teste consta do inventário; não define a licença final do código do projeto. Dependências e ferramenta externa têm suas próprias licenças no [inventário da evidência](../../../../docs/evidence/PR/task-e01-01/dependencies.json).

São dois trechos estéreo de 4417 frames a 44100Hz, não alinhados ao frame MP3: sequência inteira com extremos e LSBs para WAV24/WAV32/FLAC24 e tons distintos por canal com silêncio intencional nas bordas para MP3 CBR192k/VBR q2. Referências s32le lossless são iguais às fontes inteiras geradas antes do encoder; referências MP3 f32le vêm do decoder independente mp3float do FFmpeg. Os resultados dos dois arquivos, decodificados separadamente, concatenam 8834 frames (não é prova de gapless entre faixas). Duas fixtures negativas sem Xing/LAME (CBR curta e VBR longa com fonte22050 frames) têm referências FFmpeg, mas o adapter retorna Unsupported por ausência de trim/contagem confiáveis; seus truncamentos também não podem devolver PCM parcial. Não são corpus musical amplo, AAC, surround ou testes de output.

Reprodução da geração (PowerShell7, referência FFmpeg9.0.2 com libmp3lame; versão/build/SHA no manifesto):

```powershell
./tests/fixtures/audio/decode/generate.ps1 -FFmpeg <caminho-do-ffmpeg-de-referencia>
cargo test -p vynamp-decode --locked -- --nocapture
```

O gerador chama FFmpeg sem shell e falha em divergência lossless/contagem; grava cada comando de encode/decode na própria implementação. Comparação MP3 usa tolerância pré-fixada 2e-5 FS, contagem exata, sem ajuste de alinhamento ou ganho. Gerações com outra versão/build podem produzir bytes diferentes: não atualizar hashes/oráculos só para fazer um alvo passar. O FFmpeg usado fica em scratch, não é dependência nem artefato de distribuição. FLAC32 tentado perdeu LSBs no encoder e foi rejeitado: não incluído como fixture aprovada; PCM32 foi provado em WAV.
