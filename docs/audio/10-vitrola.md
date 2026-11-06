# vitrola — Vinyl Simulator e Tube Amplifier Simulator no DSP

Versão: 0.7.4 · Data: 2026-11-05 · Estado: **planejamento pré-existente preservado**. O design da biblioteca foi aprovado em 2026-10-23. Biblioteca e integração não foram implementadas. No MVP web de referência (`vynamp-mvp-reference@0d9860b`, rev.38) Vinyl e Tube **têm processamento real** num AudioWorklet; a rev. 15, que só tinha configuração, é histórico (ver §5).

## 1. Fonte da verdade e escopo

- A definição acordada antes desta atualização é o design aprovado da biblioteca própria **vitrola** (`vitrola/docs/superpowers/specs/2026-10-23-vitrola-design.md`, repositório privado `jimmykedlyn/vitrola`, MIT OR Apache-2.0, Rust só com `std`, `#![forbid(unsafe_code)]`, sem bibliotecas de terceiros). Este documento a **integra por referência**: o DSP do Vynamp hospeda o vitrola como um módulo. Decisões, nomes, parâmetros, "inteligência" e restrições valem como lá estão. Aqui ficam só a integração, a UX hospedeira, os testes de integração e as divergências.
- Dois efeitos com identidade própria: **Vinyl Simulator** (simulação de um disco real tocando aquela faixa) e **Tube Amplifier Simulator** (amplificador valvulado). Os dois são **opcionais, colorativos e intencionais**, com bypass e intensidade gradual. Não são um equalizador, nem um exciter genérico, nem processamento obrigatório.
- Anti-objetivos herdados: sem UI visual de toca-discos/prato/braço/válvulas; sem emulação circuito a circuito nem nomes de marcas; sem loop de ruído gravado, padrão perceptível ou exagero caricato no padrão; sem cliques ao ligar/desligar; sem alterar o caminho bit-perfect ou DSD nativo (o efeito se desativa sozinho); sem alocação ou lock na callback.

## 2. Comportamentos aprovados (resumo da referência)

| Elemento | Comportamento acordado | Módulo vitrola |
|---|---|---|
| Raio do disco | a posição na faixa/lado vira raio. Perto do miolo há perda leve de agudos/definição e mais distorção (inner-groove); o crosstalk depende do raio | `vinyl::radius` |
| Superfície | chiado/crackle/pops guiados por um seguidor de loudness: aparecem em trechos quietos e nos silêncios entre faixas e são mascarados nas partes altas. Nunca formam padrão repetido | `vinyl::surface` |
| Rotação | wow ligado à volta do prato (33⅓/45 rpm, período = 60/rpm s) + flutter + oscilação periódica de disco empenado, via linha de atraso fracionária interpolada | `vinyl::transport` |
| Rituais | agulha descendo (~1,5 s) antes da 1ª faixa do lado; sulco final em loop (até parar, ou 30 s + fade); virada A→B opcional. Síntese procedural, sem samples gravados. **Alteram a duração** (`pre_roll()`/`tail()`) | `vinyl::rituals` |
| Cápsula | ressonância/roll-off e rumble leve | `vinyl::cartridge` |
| Envelhecimento | desgaste por disco (álbum): cada reprodução desgasta a região da faixa. Riscos permanentes voltam no mesmo ponto; poeira/crackle variam a cada audição. "Comprar disco novo" zera aquele álbum | `WearState` |
| Lados A/B | álbum em ordem: corte no limite de faixa mais próximo da metade, com ~22 min por lado (álbuns longos viram 2 discos/4 lados). Playlist/shuffle/avulsa: um lado virtual por faixa. Sem `Context` (rádio/stream): lado virtual de 20 min e rituais desligados | `vinyl::radius` |
| Aquecimento | alguns segundos até o som ficar cheio | `tube::warmup` |
| Saturação | depende da dinâmica: o sinal suave fica limpo; picos ganham harmônicos pares e compressão leve (triodo assimétrico, bias dinâmico) | `tube::preamp`, `tube::power` |
| Sag | a fonte "cede" em batidas e graves fortes | `tube::sag` |
| Transformador | roll-off LF/HF e saturação leve | `tube::transformer` |
| Perfis | "pré quente", "push-pull clássico", "single-ended" (genéricos, sem marcas) | `tube::power` |
| Controles | **um botão principal de intensidade** move tudo de forma coerente (`macro`), mais um **painel avançado por elemento**. Padrão **sutil** | `macro` |
| Inteligência | só seguidores em tempo real (loudness com ponderação perceptiva, transientes, energia de graves); sem pré-análise offline | — |
| Qualidade | oversampling polifásico próprio de 2×–8× conforme a taxa, automático (ex.: 8× a 44,1/48 kHz, 2× a ≥ 176,4 kHz); smoothing de tudo; bypass com crossfade | — |
| Taxas e canais | 44,1–384 kHz; mono/estéreo; em multicanal o vinil fica desligado e a válvula atua por canal | — |
| Opção avançada | "acoplar ao volume": a válvula "abre" com volume alto | `macro` |

## 3. Integração no grafo do Vynamp

- **Adaptador** (planejado após TASK-E04-03): a trait `Processor` do vitrola (`prepare(sample_rate, max_block, channels)` → `process(&mut block, &Context)` → `reset`) é encaixada no contrato de nó do [DSP](03-dsp.md#contrato-conceitual-dos-nós). A fronteira aceita f32 intercalado ou planar; a posição chega em samples.
- **`Context` por bloco** (fornecido pelo host): posição/duração da faixa, índice da faixa no álbum, total de faixas, duração do álbum, eventos de transporte (start, stop, pause, seek, fim de álbum, `track_changed` gapless no mesmo bloco) e contagem de reproduções. É opcional.
- **Posição** (decisão vitrola §8.1): depois do EQ e **antes do volume do usuário** (ganho master). Na cadeia real fica no segmento por stream, depois da correção de fone, antes de dinâmica/imagem/criativos e antes de mixer/master/resampler/proteção final ([diagrama](03-dsp.md#cadeia-real-de-processamento)). Ordem dentro do segmento: **proposta**. Reordenação livre não é prometida.
- **Conflito aberto D-DSP-05**: o vitrola diz "depois do resampler", mas o [contrato do sinal](07-dsp-signal-contract.md) tem um único resampler final, depois do master. A decisão do vitrola fica **preservada** como requisito e o contrato 07 **não** foi alterado. Opções: (A) o vitrola roda na taxa do stream, já suportada de 44,1 a 384 kHz, com o oversampling automático cobrindo o aliasing; (B) um resampler por stream antes dos efeitos, que exige emendar o 07 por decisão. Isso também resolveria a mistura de fontes de taxas diferentes no mixer, outro ponto **dependente de investigação**. Fontes abaixo de 44,1 kHz ficam fora da faixa do vitrola: o efeito fica indisponível com motivo, salvo decisão (B).
- **Ponto aberto D-DSP-06, crossfade**: a referência pressupõe um "disco" contínuo, com gapless via `track_changed`, e não define o crossfade. Candidato **a validar**: com o vitrola ligado, o crossfade fica desativado (com aviso), ou só a válvula continua por stream.
- **Rituais e transporte**: `pre_roll()`/`tail()` são inseridos pelo host no transporte. A duração apresentada e a posição consideram esse áudio extra. Rituais desligáveis.
- **WearState**: bytes estáveis e versionados, com migração. O Vynamp persiste **por disco (álbum)** na biblioteca local. "Comprar disco novo" exige confirmação. O WearState não faz parte de presets nem de exportação de preset (proposto).
- **Bit-perfect / DSD / transparente**: desativação automática com o motivo visível ("desativado: caminho bit-perfect / DSD nativo / transparente").
- **Uso isolado ou simultâneo**: Vinyl e Tube são **módulos separados** dentro da categoria Vitrola, cada um com `intensity` e `bypass` próprios, e o painel DSP expõe o toggle de bypass por módulo ([referência do MVP](../design/06-mvp-reference.md) `mvp/dsp-catalog.js`, `mvp/dsp-workspace.js`). Acima deles há o macro "Intensidade geral (Vinyl + Tube)", que move as duas intensidades juntas e declara que "condição, preset e bypass continuam independentes", e um botão único da categoria. Um módulo só entra na cadeia com o enable do módulo **e** o botão da Vitrola ligados. O MVP **não** tem contrato de "intensidade 0 = bypass da seção" nem API de host para isso: um enable separado por seção na UI nativa seria decisão nova de produto, não herança do MVP ([D-MVP-02](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05)).
- **Headroom e picos**: medidores pré e pós-vitrola. Estalos e pops sintéticos têm pico limitado (limite numérico a definir no construtor; teste VIT-T-04) e não escondem clipping, que continua indicado. A proteção final true-peak continua fixa.

## 4. Itens do pedido v1.3 versus referência

O pedido traz parâmetros "candidatos a validar". A regra: o que a referência define prevalece; o que ela não define fica **a validar**; conflitos ficam registrados abaixo.

| Item do pedido | Situação | Tratamento |
|---|---|---|
| Caráter tonal, saturação, separação estéreo | definido (radius, cartridge, crosstalk por raio) | manter |
| Hiss, crackle, clicks, rumble, desgaste | definido (surface, cartridge, WearState) | manter; controles por tipo no painel avançado (layout deixado ao construtor, §9 da referência) |
| "Ruídos desligados ou discretos por padrão" | compatível: o padrão é **sutil** | manter "sutil" |
| "Evitar ruído em pausa/silêncio" | **conflito**: a referência põe chiado e crackle **nos silêncios entre faixas**, e o sulco final em loop após o fim | **manter a referência**. Pausa → silêncio com fade curto (agulha levantada): **a validar**, porque a referência só lista `pause` como evento. Seek → `reset` sem estalo |
| Wow & flutter | definido | manter |
| 33⅓ / 45 rpm | definido | manter |
| 78 rpm | não definido | **a validar** (só como perfil de característica; nunca altera a velocidade digital) |
| RIAA | não definido | esclarecimento: a equalização RIAA **não é reaplicada** a arquivos digitais. Um filtro "inspirado em RIAA" só existiria como emulação justificada, **a validar** |
| Controles Amount/Tone/Noise/Crackle/Clicks/Rumble/Wow/Flutter/Stereo Character | Amount = macro de intensidade (definido); os demais correspondem a elementos do painel avançado | nomes finais e layout: construtor/UX, **a validar** |
| Random seed do usuário | **diverge**: a semente é automática (faixa+reprodução / disco+contagem) e fixa só nos testes | manter automática; seed exposta ao usuário: **a validar** |
| Ganho de saída compensado | não definido explicitamente | **a validar** (recomendado para A/B justo) |
| Presets Clean/Warm/Vintage Vinyl (pedido) | **diverge do pedido**: o MVP implementa três presets de fábrica do Vitrola — **Sutil**, **Quente** e **Rústico** — exibidos em "Presets do módulo" quando a categoria é Vitrola e aplicados por "Carregar" (Sutil → Vinyl "Bem conservado" / Tube "Limpo"; Quente → "Usado" / "Quente"; Rústico → "Gasto" / "Saturado") | vale o MVP ([D-MVP-02](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05)); a nomenclatura em inglês "Subtle/Warm/Rustic" e a marcação `[proposal]` não existem nele |
| Drive, Warmth/Tone | correspondem a saturação/macro e transformador | **a validar** como controles nomeados |
| Bias | **diverge**: na referência o bias é **dinâmico/interno** | manter interno; controle de usuário **a validar** |
| Mix dry/wet | não definido | **a validar** |
| Oversampling/Quality manual | **diverge**: o nível de qualidade é **automático** pela taxa | manter automático; override avançado **a validar** |
| Hum/noise da válvula | não definido | **a validar**; se existir, fica independente e desligado por padrão |
| Sag, assimetria, perfis | definidos | manter |
| Presets Clean/Warm/Driven Tube | não definidos; os perfis acordados são pré quente / push-pull clássico / single-ended | perfis mantidos; presets **a validar** |
| Ordem "correção tonal → emulação → controle de picos" | compatível com §8.1 | manter (ver D-DSP-05) |

## 5. UX hospedeira

- Local: Preferências › DSP tem o botão "Abrir DSP e Vitrola…", que abre o workspace DSP; lá a Vitrola é **categoria própria** ("Vitrola"), com Vinyl Simulator e Tube Amplifier. Atalhos: "Vitrola…" na barra do equalizador e "Vitrola" na faixa de atalhos de áudio junto ao instrumento. O MVP **não** tem atalho para a Vitrola no inspector da cadeia (Fluxo modular); um atalho ali seria decisão nova ([D-MVP-02](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05)). Só componentes padrão do Vynamp. Cards separados para Vinil e Válvula, com bypass próprio, intensidade, indicador "coloração intencional", presets, A/B, restaurar padrões e "guia de som".
- Nível 1: botão principal "Intensidade do vitrola" de 0–100% (padrão sutil), On/Off com crossfade e estado (Ativo / Desativado automaticamente com motivo / Aquecendo… / Pronto).
- Nível 2: painel avançado por elemento (lado A/B e disco atual em texto, rpm, superfície, rituais com aviso "altera a duração", cápsula, barra de desgaste somente leitura + "Comprar disco novo", perfil da válvula, sag, transformador, "acoplar ao volume").
- **Proibido**: ilustração de toca-discos, agulha, disco girando, braço ou válvulas animadas. Um ícone genérico pequeno de categoria é permitido.
- A/B com loudness igualado (**proposto**): o efeito não deve parecer "melhor" só por estar mais alto.
- **Atalho Vitrola (DSP-UX-02, v1.4).** O atalho "Vitrola" junto à área do instrumento e a família "Vitrola" do painel são **entradas para a área existente** (Vinil e Válvula, com configurações, presets, prioridades e bypass independentes). Não fundem os dois simuladores nem duplicam o estado dos parâmetros entre acessos. No protótipo o destino inicial é o vinil. Contrato: [painel DSP §8](../design/05-dsp-panel.md#8-acesso-e-controles-dsp-ux).

### Estado no protótipo × nativo

**No pin rev.38 há processamento real.** Vinyl e Tube estão em `mvp/advanced-dsp.js` como classes com kernels próprios (`class Vinyl`, `class Tube`), rodam num `AudioWorkletNode` montado por `mvp/graph-audio.js` (`VynampAdvancedDSP.TYPES = ['limiter','loudness-meter','tube','vinyl','clip']`), e `mvp/graph-model.js` marca os dois com `real=true`. O `class Vinyl` implementa superfície (hiss/crackle/pops por estado de ruído e PRNG), cápsula (peaking 13 kHz, lowpass 15 kHz, highshelf 6,5 kHz), transporte (wow/flutter/drift), raio e desgaste; o `class Tube` implementa saturação por perfil, compressão, sag, transformador e aquecimento, com oversampling 2× e latência declarada de 3 amostras. A descrição de cada um nos catálogos é explícita: "Modelo procedural de superfície, cápsula, traçado e transporte" e "Modelo não linear por canal, assimetria por perfil… Não é circuito físico calibrado". Ou seja: **não** é processamento de referência validado; é processamento existente.

O que a rev. 15 relatava — "configuração completa do catálogo, sem processamento" — **não vale mais**. A frase antiga do MVP (`graph-editor.js`: "Vinyl/Tube não têm adaptador") é texto que ficou para trás e contradiz o próprio código do pin.

**O que continua pendente**, e é o que a validação acústica cobriria:

| Item | Estado no pin | Efeito |
|---|---|---|
| Validação sonora (espectro, aliasing, densidade de crackle × loudness, período de wow) | não executada | os testes VIT-T-01..10 (§7) continuam sem resultado |
| Medição de CPU, latência e qualidade a 96 kHz | não executada | a meta de < 5% de um núcleo é meta, não gate |
| `WearState` persistido por álbum | o worklet acompanha desgaste em memória (`accumulatedWear`, até +15%); não há arquivo persistido | persistência e migração continuam decisão de integração |
| `Context` completo (rádio, gapless, `track_changed`) | o MVP passa `transport` (posição, lado, desgaste, fase) | o contrato completo da §3 continua especificado, não implementado |
| Plataformas e taxas | o MVP roda no navegador | nada em nativo |

**Especificado**: este documento e o design aprovado do vitrola. **Implementado no MVP (verificado no código)**: processamento de Vinyl e Tube no grafo modular. **Validado**: nada. **Nativo**: nada. Fases, prioridade e a pendência de TASK (seção 6) não mudam.

## 6. Roadmap (status herdado)

- Ordem acordada (vitrola §6; STATE.md): `vynamp-decode` Camada 1 → **TASK-E04-03** (grafo DSP, M2) → vitrola. A biblioteca pode ser construída e testada offline antes disso. O marco da integração é **M3, depois de E04-03** ([D-DSP-07](../product/05-decisions.md#decisões-dsp-e-equalizadores-2026-10-25)).
- O `ROADMAP-rodadas.md` da raiz lista só TASKs e **não** tem uma linha vitrola. Não existe TASK vitrola no backlog. **Pendência**: criar uma TASK de integração (fase M3, depois de E04-03) num PR que altere `tasks/backlog.json` e a matriz de requisitos. Este PR é só de Markdown e não a cria. A prioridade não foi deslocada.
- CPU: meta < 5% de um núcleo moderno a 96 kHz estéreo com tudo ligado. É meta, não gate; confirmar por benchmark.

## 7. Testes de integração planejados

Os testes da própria biblioteca (determinismo com semente, espectro de harmônicos, aliasing, densidade de crackle × loudness, período de wow, denormais, alocação zero, crossfade) estão na referência §5. No Vynamp (cenário QA-033, [estratégia](../quality/01-test-strategy.md)):

| ID | Objetivo | Método | Evidência | Critério |
|---|---|---|---|---|
| VIT-T-01 | Bypass transparente | vinil e válvula ligados → desligados; comparar com o sinal seco após o crossfade | diff de amostras | idêntico ao seco depois do fim do crossfade; sem degrau |
| VIT-T-02 | Desativação automática | entrar em transparente/DSD/DoP com o vitrola ligado | log de estado + UI | vitrola em bypass com motivo; selo correto |
| VIT-T-03 | Multicanal | fonte 5.1 | estado + análise por canal | vinil indisponível com aviso; válvula por canal |
| VIT-T-04 | Picos de estalos | crackle/pops no máximo sobre um silêncio digital | sample/true-peak pós-vitrola | pico ≤ limite declarado; clipping nunca escondido |
| VIT-T-05 | Pausa/seek/fim de álbum | pausar, seek rápido, fim do lado e rituais | captura + log de transporte | sem estalo no seek; tail/pre_roll inseridos e contabilizados na duração; comportamento em pausa conforme decisão |
| VIT-T-06 | Persistência do WearState | tocar, reiniciar o app, "Comprar disco novo" | bytes persistidos por álbum | desgaste preservado; reset só do álbum pedido |
| VIT-T-07 | Vinil + válvula + EQ +15 dB | combinação extrema | medidores por estágio + QA-027 | clipping cumulativo indicado; teto final respeitado |
| VIT-T-08 | Aliasing da válvula | senos 1/5/10/15 kHz, níveis −20/−6/0 dBFS, 44,1–192 kHz | espectro | aliasing abaixo do limiar definido na biblioteca |
| VIT-T-09 | CPU/latência | 96 kHz estéreo com tudo ligado | bench | publicar o número; meta < 5% de um núcleo |
| VIT-T-10 | A/B com loudness igualado | comparar on/off com a compensação | LUFS integrado das duas versões | diferença dentro da tolerância a definir |
