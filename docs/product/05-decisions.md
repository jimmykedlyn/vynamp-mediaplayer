# Decisões, hipóteses e pontos em aberto

Versão: 0.8.0 · Data: 2026-11-05 · Estado: especificação proposta, implementação não iniciada.

## Decisões fixadas pelo pedido

Produto independente em Rust; inspiração visual clássica com cores especificadas; temas e plugins próprios; streaming cliente agora e plataforma futura; ampla funcionalidade de áudio; incluir hi-res e recursos inspirados em AIMP e Poweramp. Não exigir compatibilidade Winamp.

## Hipóteses adotadas para execução

| ID | Decisão provisória | Razão / revisão |
|---|---|---|
| H-01 | Substituída: Windows/Linux/macOS Intel+Silicon/Android obrigatórios | Instrução do usuário; ADR-006; matriz/gates M0–M3 |
| H-02 | Resolvida: Vynamp, identidade C — Pulso | Escolha explícita do usuário; ADR-008; exports nativos antes M3 |
| H-03 | Sem conta para uso local | Reduzir dependência da plataforma |
| H-04 | Desktop UI Rust; Android shell Kotlin/Compose proposto, core Rust comum | Provar a11y/bridge/lifecycle em M0; não duplicar engine |
| H-05 | Desktop 1.0 gratuito sem anúncios no player | Política comercial ainda em aberto; não embutir billing no núcleo |
| H-06 | Temas/assets e implementação novos | Evitar acoplamento ao projeto anterior |
| H-07 | PCM até 384 kHz como alvo 1.0 | 768 kHz e DSD nativo em expansão, dependentes de hardware |
| H-08 | Web M4 com live e VOD | Não construir backend antes de validar cliente |

## Decisões pendentes com dono e prazo

UI final e backend exclusivo: responsável engenharia, M0, provas de acessibilidade/latência. Licença do **código**: resolvida em 2026-11-05 (D-LIC-01, `MIT OR Apache-2.0`); a **marca** (registro e domínio) segue pendente, responsável produto, antes publicar M3. Encoder AAC/MP3, ASIO e FFmpeg: engenharia/revisão de distribuição, antes empacotar. Hosting/região/custos/CDN: plataforma, entrada M4. Catálogo próprio, royalties e territórios: produto/plataforma, antes ingest público. DRM e assinatura: nova RFC, não dependência de M1–M3. Datas dependem de equipe/hardware; estimativas por tarefa são faixas, não calendário prometido.

## Gestão de escopo

Adicionar requisito exige ID, fase, critérios mensuráveis e tarefa. Remover exige decisão explícita, atualização da matriz e changelog. “Todas as funcionalidades” significa cobertura da matriz acordada e do catálogo de recursos deste pacote, não afirmação de equivalência binária ou de cada opção histórica dos três produtos.

## Decisões DSP e equalizadores (2026-10-25)

Vêm da revisão documental do DSP (pedido "Documentação DSP — Equalizadores Obrigatórios" v1.3). Estado: **proposto**, até a aprovação do mantenedor no PR. Nenhuma altera código. Detalhes em [DSP](../audio/03-dsp.md), [equalizadores](../audio/08-equalizers.md), [catálogo](../audio/09-dsp-catalog.md) e [vitrola](../audio/10-vitrola.md).

| ID | Decisão | Razão / revisão |
|---|---|---|
| D-DSP-01 | Marcos do EQ: **PEQ nativo de 32 filtros continua M3** (TASK-E04-04); **gráfico nativo de 64 bandas e qualquer PEQ de 64 filtros continuam M5** (TASK-E12-10) até uma decisão documentada mudar isso; o **MVP HTML/SVG/JS cobre agora** a UI interativa de Gráfico 10/20/31/64 e Paramétrico até 32 | O pedido diz "64 bandas M5 permanece M5", mas em 03-dsp v0.7.0 o item M5 era o **paramétrico** de 64; os dois itens de 64 ficam em M5 e a UI do protótipo não muda o marco nativo |
| D-DSP-02 | Clássico e Avançado 10 são o mesmo estado (EQ-10); o clássico mostra ±12 dB e marca "fora da escala" quando o avançado passou disso, sem cortar o valor | Coerência entre painéis sem perda silenciosa |
| D-DSP-03 | Grades: EQ-20 em meia oitava `1000·2^((k−11)/2)` (22 Hz–16 kHz); EQ-31 em 1/3 de oitava ISO 266/IEC 61260-1 base 10; EQ-64 logarítmica `20·1000^(k/63)` (≈1/6,3 oitava, **não** 1/3); peaking de Q constante `Q=√(2^N)/(2^N−1)` | Faltava grade definida para 20 e 64; aprovar em E04-04/E12-10 |
| D-DSP-04 | Rótulo canônico do protótipo: `[DEMO — sem processamento de áudio neste modo]` (en: `[DEMO — no audio processing in this mode]`) | Rótulo do pedido v1.3, adotado pelo MVP. Os SKIN tinham variantes; foram descartados em 2026-11-05 (D-MVP-01) e não são mais referência |
| D-DSP-05 | **Aberto**: o vitrola §8.1 pede "depois do resampler", mas o contrato 07 tem um único resampler final. A decisão vitrola fica preservada e o 07 não muda; opção A (taxa do stream) × B (resampler por stream, emenda do 07) | Precisa de decisão do mantenedor; afeta também mixer com Fs diferentes |
| D-DSP-06 | **Aberto**: crossfade com o vitrola ligado (candidato: desativar o crossfade com aviso) | A referência pressupõe disco contínuo/gapless |
| D-DSP-07 | O vitrola é hospedado por referência; os parâmetros "candidatos" do pedido que a referência não define ficam **a validar**; nos conflitos vale a referência; falta uma TASK de integração (M3, após E04-03) | Preservar o acordo anterior; criar a TASK num PR de backlog |
| D-DSP-08 | **Decidido (2026-11-03)**: lib própria de resampling, nome a definir (repositório a criar pelo mantenedor); sem resampler de terceiros ([ADR-011](../architecture/adr/ADR-011-own-audio-stack.md)) | Spike e ADR de qualidade na TASK-E04-02 |
| D-DSP-09 | Um único modo de EQ processa por vez; os outros ficam guardados. AutoEQ/headphone correction é outro nó, depois do EQ | Pipeline "gráfico OU paramétrico"; correção sem ocupar o EQ do usuário |
| D-DSP-10 | Ordem proposta no segmento por stream: EQ → correção → vitrola → dinâmica/imagem/criativos; reordenação livre não é prometida | Pedido v1.3 §3.5 compatível com vitrola §8.1 |
| D-DSP-11 | As **oito famílias** do painel (Ganho/EQ, Dinâmica, Acústica, Estéreo, Tempo/formato, Criativos, Vitrola, Saída) são navegação: não são switches de processamento, não limitam o catálogo a oito efeitos, não alteram a ordem da cadeia nem os marcos nativos. Os 39 módulos e 198 campos do protótipo descrevem o protótipo e não congelam a arquitetura nativa | Pedido v1.4 §5.1 (DSP-UX-03, §5.1.5); mapeamento módulo→família em [09](../audio/09-dsp-catalog.md#famílias-v14-e-estado-no-protótipo-rev-15-relatado) é proposto |
| D-DSP-12 | **Aberto**: o grafo modular editável é **contrato do protótipo**. No nativo, a reordenação livre continua não prometida (D-DSP-10) e roteamento livre × ordem fixa fica como decisão do mantenedor, sem mudar marcos | Pedido v1.4 §12.2/§12.8 ("reordenação livre do produto nativo não foi implementada"); ver [03-dsp](../audio/03-dsp.md#grafo-modular-protótipo--nativo) |
| D-DSP-13 | Estado único de parâmetros entre acessos (cabeçalho, atalhos EQ/Dinâmica/Estéreo/Vitrola, categorias, tema). O atalho Vitrola leva à área existente (Vinil e Válvula independentes), sem fusão dos simuladores | DSP-UX-02/17, §5.1.5 item 2; [10](../audio/10-vitrola.md) |
| D-DSP-14 | Precisão e validação de parâmetros: preset válido com valor fora do passo do gesto é preservado, sem quantizar; valor rejeitado não é aceito visualmente e o último estado válido é mantido; parâmetros dependentes (crossovers) validam em conjunto. O passo de interação não é restrição de validade | DSP-UX-14..16; regressões em [matriz de testes §2.1](../quality/05-dsp-test-matrix.md#21-aceite-de-acesso-e-controles-dsp-ux-ac) |
| D-DSP-15 | Escolha do controle pelo metadado do parâmetro (knob, slider, chave, seletor, valor clicável com editor exato). Sliders por banda dos EQ gráficos permanecem, com todas as bandas editáveis. O protótipo (controles `range` nativos e SVG) não impõe tecnologia à interface nativa; vale o contrato observável e acessível | DSP-UX-06..10, §5.1.2 |
| D-DSP-16 | Todo status é separado em **especificado · implementado no protótipo (relatado) · validado no protótipo (relatado) · nativo**. O protótipo é do Codex e fica fora do repositório; seus arquivos de evidência são tratados como relato da referência externa rev. 14/15, não verificado aqui. Nó configurável ou controle que se move não prova processamento | Pedido v1.4 §12.8; mesma regra do rótulo `[DEMO]` (D-DSP-04) |
| D-DSP-17 | Limites de capacidade do protótipo registrados como contrato de documentação: o **Limiter amostral não é True Peak** (nem brickwall, nem dBTP) e não substitui o True Peak Limiter exigido; a **Saturação suave não é o Tube Amplifier**; Vinyl e Tube só têm configuração, sem processamento; True Peak, LUFS, gate/expander e demais módulos sem adaptador são indisponíveis para áudio. Fases, prioridades e TASKs anteriores não mudam | Pedido v1.4 §12.3; [09](../audio/09-dsp-catalog.md), [10](../audio/10-vitrola.md) |
| D-DSP-18 | **VU-INT**: preservar byte a byte os 12 SVGs aprovados; adaptar só o gabinete externo, junta e rótulos externos na cópia exibida, por tokens da skin; três apresentações (Original, Metal adaptativo, Integrado ao painel, esta última como padrão, preferência em Aparência). A adaptação é de apresentação em tempo de execução, não embutida nos pacotes `.vyntheme`; o espaço dos VUs estéreo originais não é reduzido pelo DSP | Pedido v1.4 §12.1, DSP-UX-20; [painel DSP §6.1](../design/05-dsp-panel.md#61-integração-visual-dos-vu-meters-vu-int) |

### Divergências encontradas

| Onde | O que diverge | Tratamento |
|---|---|---|
| Pedido v1.3 §"Requisito obrigatório" × `docs/audio/03-dsp.md` v0.7.0 §"EQ paramétrico" | "64 bandas M5" referia-se ao paramétrico, não ao gráfico | D-DSP-01 |
| `docs/audio/03-dsp.md` v0.7.0 §"EQ gráfico" | editor avançado só com 10/20/31, sem modo gráfico 64 | corrigido em 0.7.1 e [08](../audio/08-equalizers.md) |
| `tasks/items/TASK-E12-10.md` §Entrega | "EQ64" não diz se é gráfico ou paramétrico | interpretado como os dois (D-DSP-01); texto da TASK não alterado |
| `schemas/audio-preset.schema.json` v1 | sem `mode`, `enabled`, `slope`, nem modos gráficos | v2 pendente em TASK-E04-04 (este PR é só Markdown) |
| `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §13.1 × §13.3.7 | rótulos `[DEMO — no actual audio processing]` e "Prévia interativa — o áudio não está sendo processado por este modo" diferem do rótulo do pedido v1.3 | **Encerrada** por D-DSP-04: o rótulo canônico é `[DEMO — sem processamento de áudio neste modo]`, e é o que o MVP usa. O histórico dos SKIN não é mais mantido como referência |
| `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §5.11 | presets vitrola Subtle/Warm/Rustic [proposal] × Clean/Warm/Vintage do pedido | **Encerrada**: o MVP tem Sutil/Quente/Rústico, presets reais e sem marcação de proposta. Vale o MVP; ver a tabela "Comportamentos que os SKIN atribuíam" |
| `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §12.8 | lista módulos (Dynamic EQ, Bass Enhancer, Exciter, multibanda, declipper, crossfeed, M/S, HRTF…) ausentes do repositório | catalogados como **propostos** em [09](../audio/09-dsp-catalog.md); nenhum deles é requisito por ter aparecido no SKIN (D-MVP-03) |
| `SKIN-REQUIREMENTS-CODEX.md` v1.0 §1.4 item 5, §5.9 (editor avançado e Android) e §6 | 64 bandas gráficas só como "reserved slot"; editor com 10/20/31; sem seletor de 5 modos | **Encerrada**: o MVP tem os cinco modos e 64 `input[type=range]` reais (`mvp/advanced-eq.js`). Vale o MVP, não o documento |
| Pedido v1.3 §3.5.A "evitar ruído em pausa/silêncio" × vitrola §2 | a referência põe crackle nos silêncios entre faixas e o run-out em loop | vale a referência; pausa = a validar |
| Pedido v1.3 §3.5.B (Bias, Oversampling/Quality, seed) × vitrola §4/§8.7 | bias dinâmico interno, qualidade automática, semente automática | vale a referência; controles do usuário a validar |
| vitrola §8.1 × `docs/audio/07-dsp-signal-contract.md` §Caminho | "depois do resampler" × resampler único final | D-DSP-05 |
| Contexto informado × `ROADMAP-rodadas.md` (raiz) | não há linha vitrola; a ordem "após E04-03" está em `STATE.md` e no vitrola §6 | TASK de integração pendente (D-DSP-07) |
| `docs/audio/04-outputs-hires.md` §Backend matrix × design `vynamp-output` 2026-10-25 | "WASAPI shared via biblioteca de terceiros" e "ASIO opcional" × biblioteca própria substitui a de terceiros; ASIO fora do núcleo | resolvido em 2026-11-03 por [ADR-011](../architecture/adr/ADR-011-own-audio-stack.md); a integração segue o plano 05 da vynamp-output |
| [Painel DSP](../design/05-dsp-panel.md) v1.3, Navegação: 7 categorias (incluindo Medidores) × DSP-UX-03: 8 famílias | Medidores vira área própria, fora das famílias | D-DSP-11 |
| Catálogo [09](../audio/09-dsp-catalog.md) (v1.3: seis seções, cinco de módulos mais extensibilidade) × pedido v1.4 DSP-UX-03 (oito famílias) | agrupamentos diferentes para o mesmo conjunto de módulos | D-DSP-11: capítulos mantidos, família de navegação adicionada por módulo (proposta); sem renumerar seções |
| Grafo modular livre do protótipo (rev. 15) × D-DSP-10 / [07](../audio/07-dsp-signal-contract.md) (ordem fixa por stream) | o protótipo permite roteamento livre; o nativo não promete reordenação | D-DSP-12 (aberto); grafo é contrato do protótipo, ordem fixa permanece no nativo |
| Pedido v1.4 §12.3 (EQ e PEQ com áudio real no grafo) × [08](../audio/08-equalizers.md) v0.7.1 ("nenhum equalizador implementado, nem no MVP") | o 08 dizia que nem o MVP processava; a rev. 15 relata áudio real nas instâncias do grafo, mas só o EQ clássico de 10 bandas (e o grafo) têm adaptação, os demais modos do editor anterior seguem `[DEMO]` | 08 §14 atualizado como evidência relatada; não retroagir para editores desconectados; nativo continua nada, EQ-64 em M5 |
| Pedido v1.4 §12.3 "Limiter amostral" × exigência de True Peak Limiter (07 e catálogo §2) | um limitador no protótipo poderia ser lido como o True Peak exigido | D-DSP-17: não é True Peak nem garantia contra clipping; o True Peak Limiter continua exigido e sem adaptador |
| Pedido v1.4 §12.3 "Saturação suave" × Tube Amplifier (vitrola) | nome parecido, processamento diferente | D-DSP-17: Saturação (tanh, 2×) não é o Tube; Vinyl e Tube só com configuração |
| Pedido v1.4 §5.1 "referência contém 39 módulos / 198 campos" × catálogo [09](../audio/09-dsp-catalog.md) | contagens do protótipo não coincidem com o catálogo nativo (que também tem módulos propostos sem adaptador) | D-DSP-11: números não congelam o catálogo; evidência relatada |
| Pedido v1.4 (protótipo Codex rev.14/15, histórico; fonte atual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 rev.38; evidências `interactive/*`, `*-validation.json`, `CONTROLES-DE-AUDIO.md`, `DSP-MODULAR-E-VU.md`) × repositório | os arquivos citados não existem neste repositório | D-DSP-16: citados como relato da referência externa; nenhum caminho linkado; registrar a referência no repositório do produto fica pendente |
| `docs/audio/07-dsp-signal-contract.md` §Caminho | mixer soma streams antes do resampler final; fontes com Fs diferentes no crossfade não têm conversão definida | dependente de investigação (junto com D-DSP-05) |

## Decisões de licença e governança (2026-11-05)

| ID | Decisão | Razão / revisão |
|---|---|---|
| D-LIC-01 | O código deste repositório é **`MIT OR Apache-2.0`**, com `LICENSE`, `LICENSE-MIT` e `LICENSE-APACHE` na raiz, igual às bibliotecas `vynamp-decode` e `vynamp-output` | Decisão do mantenedor em 2026-11-05, registrada aqui porque `AGENTS.md` proíbe criar licença final sem decisão registrada e [02-privacy-licensing](../security/02-privacy-licensing.md) proibia criar `LICENSE` "como se tivesse sido decisão do usuário". Sem licença, abrir os repositórios (TASK-E19-06) publicaria código que ninguém pode usar legalmente. Ver [LEGAL.md](../../LEGAL.md); TASK-E19-02 |
| D-LIC-02 | A licença cobre **só o código deste repositório**. Não relicencia código que as bibliotecas de áudio reservam para `crates-copyleft/` (convenção ainda vazia nas duas), não concede direitos sobre codecs de terceiros e **não** equivale a release: `release preflight` continua exigindo `THIRD_PARTY_NOTICES`, evidência de hardware real por plataforma e registro de revisores, nenhum dos três existente; `publish-draft` exige ainda assinatura minisign | Evitar que a existência de `LICENSE` seja lida como "produto pronto para distribuir"; TASK-E19-02 |
| D-GOV-01 | Canal de vulnerabilidade = **GitHub Security Advisories** do próprio repositório (`/security/advisories/new`), privado por padrão, sem endereço de e-mail novo | Decisão do mantenedor em 2026-11-05. Fecha a pendência "URL/email de segurança aprovado" de [SECURITY.md](../../SECURITY.md) sem inventar contato operacional; TASK-E19-02 |

## Decisões de letras (2026-10-28)

Vêm do adendo "letras integradas ao VU meter" v1.0 (2026-10-25). Estado: **proposto**, até a aprovação do mantenedor no PR. Nenhuma altera código. Detalhes em [Letras: componente resumido sob o VU e fontes](../library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes), [layouts](../design/02-layouts.md#letras-sob-o-vu-2026-10-28), [acessibilidade](../design/03-accessibility-i18n.md#letras-acessibilidade-e-localização), [requisitos](03-requirements.md#rastreabilidade-de-letras-lyr) e [TASK-E03-09](../../tasks/items/TASK-E03-09.md).

| ID | Decisão | Razão / revisão |
|---|---|---|
| D-LYR-01 | Os IDs `LYR-01..15` e `LYR-AC-01..12` do adendo são adotados como IDs canônicos, rastreados por REQ-E03-09 / TASK-E03-09, até um mapeamento oficial | O adendo permite o mapeamento sem perder a correspondência; sem novo REQ para não alterar contagens da matriz |
| D-LYR-02 | O componente resumido sob o VU **acrescenta** a apresentação e a prioridade de fontes; o painel completo de letras e seus controles (auto-scroll, offset, tamanho, busca opt-in, edição, cópia, leitura assistiva) continuam obrigatórios; clique em linha para seek segue proposta | O adendo proíbe reduzir a cobertura; três linhas não promovem os demais controles a concluídos |
| D-LYR-03 | Ordem de fontes: LRC importado para a faixa na sessão, depois metadados embutidos, depois estado informativo. Sem descoberta de arquivos auxiliares nem provedores silenciosos (exigiria requisito separado); busca online segue opt-in | Alinhado a "sem rede involuntária" de REQ-E03-09 |
| D-LYR-04 | Texto sem timestamps nunca é tratado como sincronizado; "Desligar" é preferência global de exibição e "Instrumental" é marca reversível por faixa, nunca deduzida da ausência de tags; ambos persistem por identidade estável de faixa da biblioteca e nunca alteram tags | LYR-08, LYR-11, LYR-12 |
| D-LYR-05 | Concorrência: leitura/importação pertence à faixa que a iniciou; resultado obsoleto é descartado ou guardado só no cache da faixa correta e nunca religa a exibição após Desligar | LYR-AC-08 |
| D-LYR-06 | A tabela de famílias de tags (ID3v2.3/2.4, FLAC, Ogg/Opus, M4A, WAV/RIFF, LRC) é **baseline relatada** do protótipo, não suporte nativo; o nativo depende de TASK-E03-04 e do plano 04 (metadados e CUE) da `vynamp-decode`. SYLT com timestamps em quadros MPEG: a referência não converte; o nativo pode converter com testes próprios | Adendo §4; sem alegar "suporte universal" |
| D-LYR-07 | Persistir arquivos LRC importados no produto **não está especificado** (na referência só há cache de sessão e preferências leves); exige requisito próprio. Limites de leitura da referência (16 MiB por leitura, 32 MiB de orçamento, 512 KiB de texto) são relatados; os do produto ficam a definir | Aberto; evita pressupor o que não existe |
| D-LYR-08 | A referência (protótipo Codex, revisão 14, histórico; fonte atual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 rev.38; `interactive/*`, validações `lyrics-*`, `LETRAS-SINCRONIZADAS.md`, `MVP-COMPLIANCE.md`) **não existe neste repositório**: é citada como evidência relatada, sem links; registrá-la no repositório do produto fica pendente. Níveis sempre separados: especificado, implementado no protótipo, validado no protótipo, nativo (hoje: nada) | Mesmo tratamento de D-DSP-16 |
| D-LYR-09 | O componente de letras e o VU têm estados independentes; o componente adapta-se ao espaço sem reduzir nem distorcer os 12 VUs, e não é imposto no modo Mini sem espaço | LYR-15; adendo §6 |

### Divergências de letras

| Onde | O que diverge | Tratamento |
|---|---|---|
| Adendo §2 (seções 5.13, 5.16, 5.18, 8, 9, 12, 13 e MVP-09 da spec web v1.2) × repositório | essas seções e o grupo MVP-09 não existem neste repositório (a spec web v1.2 é externa) | mapeado para os docs do repositório (biblioteca, layouts, acessibilidade, requisitos); nenhuma seção externa editada |
| Adendo §1 (atualizar `SKIN-REQUIREMENTS-CODEX.md` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md`) × escopo deste PR | os arquivos de skin estão fora do repositório (hoje GPT-MVP v1.3 e CODEX v1.2) | **Encerrada em 2026-11-05**: os dois arquivos foram descartados como fonte de requisito (D-MVP-01). Não há o que atualizar — as letras valem por este repositório |
| Adendo §5 (privacidade e persistência) × `docs/security/02-privacy-licensing.md` | o documento de privacidade não cobre persistência de preferência/marca instrumental nem leitura local de letras | não atualizado neste PR; pendente |
| Adendo §8 (LYR-AC) × `docs/quality` | não há item QA para LYR-AC-01..12 em `01-test-strategy.md` nem `04-integration-codecs-dsp.md` | ausente; pendente (ver TASK-E03-09) |
| Adendo §3 LYR-01 ("layout estendido aprovado" com VU) × [02-layouts](../design/02-layouts.md) | os layouts em DIP não definem região de VU estendido; o VU é descrito em [painel DSP §6](../design/05-dsp-panel.md#6-medidores-e-visualização) | posição relativa especificada (abaixo do VU); geometria exata pendente |
| Adendo §3 LYR-05 (relógio) × [engine](../audio/01-engine.md) | o protótipo usa o evento de posição de reprodução do navegador; o nativo usa o relógio consumido | vale o relógio consumido no nativo, como o adendo e REQ-E03-09 já preveem |
| Adendo §7 × painel completo de letras | controles do painel completo não estão no componente do protótipo | mantidos como obrigatórios e pendentes; sem promoção a concluídos (D-LYR-02) |

## Decisão: o MVP é a fonte única da UI (2026-11-05)

O mantenedor descartou os dois arquivos de skin como fonte de requisito visual. A UI nativa se baseia **exclusivamente** no MVP web de referência.

| ID | Decisão | Razão / revisão |
|---|---|---|
| D-MVP-01 | `SKIN-REQUIREMENTS-CODEX.md` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` **não são fonte de requisito** e saem da cadeia documental do produto. A fonte única da UI é o MVP em [referência do MVP](../design/06-mvp-reference.md), pinado por SHA | Decisão do mantenedor em 2026-11-05: "o vynamp se baseará pelo mvp". Os SKIN eram consumidores declarados do repositório, não origem: seus próprios changelogs citavam `DSP-UX`, `VU-INT` e `LYR` como documentos canônicos daqui. Manter duas fontes produzia duas especificações para o mesmo comportamento |
| D-MVP-02 | Onde um documento deste repositório atribui um comportamento ao SKIN, vale o que o **código do MVP comprova**. Comportamento que os SKIN pediam e o MVP não tem **não entra** por herança de documento descartado: ou é decisão nova registrada aqui, ou não existe | Impede que um documento descartado continue normativo por citação indireta. O MVP é aferível (código + `out/`); o SKIN era prosa |
| D-MVP-03 | Nenhuma TASK, REQ ou marco nasce dos SKIN. Cobertura do nativo continua sendo a matriz ([03-requirements](../product/03-requirements.md)) e o catálogo de recursos; o MVP **mostra** a UI, não promete fase nativa | Gestão de escopo já fixada: requisito exige ID, fase, critério mensurável e TASK |
| D-MVP-04 | Onde este repositório e o MVP divergem **em número**, vale o MVP. Hoje: o contrato VU-INT fala em 12 VU meters (número herdado da rev.15), e o pin rev.38 tem **16 medidores, todos selecionáveis**. A contagem certa é **16**; os 12 são anteriores aos quatro medidores acrescentados depois. A matriz VU-INT-06 (144 combinações = 12 skins × 12 VUs) é atualizada quando a TASK-E02-07 tratar a integração visual | Verificado em `vynamp-mvp-reference@0d9860b`: `out/svg/meters/` tem 16 diretórios, os 16 com `selectable: true`, e `mvp/player.js` lista os mesmos 16 em `meterChoices`. **Onde o 12 ainda aparece**: as 12 descrições de `out/themes/themes.json` dizem "encaixe universal para os 12 VUs aprovados". Isso é texto que ficou para trás — os quatro medidores restantes foram um acréscimo posterior, já pedido pelo mantenedor — e **não** é contagem válida: leia-se 16. São 12 **temas**, não 12 medidores |

**O grupo de IDs `MVP-01..18` não entra aqui.** O de mesmo nome em `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §13.2 é local àquele documento e não vira ID deste repositório. Nenhum documento daqui o cita como contrato; a única menção remanescente, `MVP-09` em [letras](../library/02-tags-lyrics-tools.md), descreve um grupo da spec web v1.2 externa, não um requisito deste repositório.

**O que isto não muda:** `[DEMO — …]` (D-DSP-04), a separação de status especificado · protótipo · nativo (D-DSP-16), os limites de honestidade do protótipo (D-DSP-17) e a integração visual dos VUs (D-DSP-18) continuam valendo — são decisões deste repositório, não dos SKIN.

### Comportamentos que os SKIN atribuíam ao produto e o MVP não tem

Verificado no pin `vynamp-mvp-reference@0d9860b` (rev.38). Nenhum destes entra na UI nativa por herança de documento; cada um exigiria decisão nova aqui.

| Onde era citado | O que se atribuía | Estado no MVP |
|---|---|---|
| [painel DSP §6](../design/05-dsp-panel.md#6-medidores-e-visualização) | "0 VU = −18 dBFS **ajustável**" | **Não existe.** Nenhuma constante de calibração no código; a escala de cada medidor vem de `scaleDB`/`scaleAngles` no `meta.json` e não é configurável pelo usuário |
| [painel DSP §6](../design/05-dsp-panel.md#6-medidores-e-visualização) | "slot de 28 DIP" | **Não existe.** O instrumento é fluido: `#meter-stage` com `width:100%`/`height:auto` |
| [vitrola §3](../audio/10-vitrola.md) | Atalho da Vitrola **no inspector da cadeia** | **Não existe.** Os atalhos são a barra do equalizador e a faixa de áudio junto ao instrumento |
| [vitrola §3](../audio/10-vitrola.md) | "intensidade 0 por seção = bypass da seção", via API de host | **Não existe** contrato de intensidade nem API de host. O MVP tem bypass por módulo e um botão único da categoria |
| [vitrola §4](../audio/10-vitrola.md) | Presets "Subtle/Warm/Rustic" `[proposal]` | **Diverge**: os presets são **Sutil/Quente/Rústico**, implementados como presets reais, sem marcação de proposta |
| [equalizadores §2](../audio/08-equalizers.md) | Passo de teclado de 0,5 dB como requisito "do SKIN" | **Existe no MVP**, mas por comportamento nativo do `input[type=range]` (`step=.5`), não por handler. Shift+seta (±2 dB) só no editor avançado; o clássico não tem |

### Divergências encerradas por esta decisão

| Onde | O que divergia | Tratamento agora |
|---|---|---|
| Adendo §1 (atualizar os dois arquivos de skin) | `docs/audio/10-vitrola.md:88` e `docs/product/05-decisions.md` §letras registravam a atualização dos SKIN como **pendente** | **Encerrada.** Os arquivos foram descartados; não há o que atualizar. A pendência sai em vez de ficar aberta |
| `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §13.1 ("nenhuma dispensa por fase") × gestão de escopo | a regra era correta **para o protótipo** e contraditória se lida como regra do produto | D-MVP-03: a regra morre com o documento; fases M1–M5 continuam governando o nativo |
| `SKIN-REQUIREMENTS-CODEX.md` §12/§13 ("é o repositório") × [referência do MVP](../design/06-mvp-reference.md) | os SKIN tratavam o protótipo como se fosse deste repositório; o MVP vive em `vynamp-mvp-reference` por SHA | Resolvido pela fonte única: [06-mvp-reference](../design/06-mvp-reference.md) é a regra |

