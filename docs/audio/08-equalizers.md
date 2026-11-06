# Equalizadores — gráfico 10/20/31/64 e paramétrico até 32 filtros

Versão: 0.7.4 · Data: 2026-11-05 · Estado: especificação proposta; **nenhum equalizador está implementado no player nativo**. No MVP web de referência (`vynamp-mvp-reference@0d9860b`, rev.38, fora deste repositório) o EQ e o PEQ existem e o código foi conferido para este documento — o que foi verificado e o que não foi está na §14.

Subcapítulo obrigatório do [DSP](03-dsp.md). Consolida o que já estava em [03-dsp](03-dsp.md) (v0.7.0), no [contrato do sinal](07-dsp-signal-contract.md) e nas tarefas [TASK-E04-01](../../tasks/items/TASK-E04-01.md), [TASK-E04-04](../../tasks/items/TASK-E04-04.md), [TASK-E02-07](../../tasks/items/TASK-E02-07.md) e [TASK-E12-10](../../tasks/items/TASK-E12-10.md). Decisões e divergências estão em [decisões DSP](../product/05-decisions.md#decisões-dsp-e-equalizadores-2026-10-25).

Legenda de estado usada em todo o texto: **implementado** (há código revisado; hoje nada de DSP), **planejado** (tem TASK no backlog), **proposto** (definido aqui, sem TASK ou ainda sem aprovação) e **dependente de investigação** (precisa de benchmark, spike ou decisão).

## 1. Matriz mandatória

São cinco modalidades selecionáveis. O Gráfico clássico e o Gráfico avançado 10 compartilham o mesmo estado (EQ-10). A diferença entre eles é só de apresentação (§4).

| ID | Modalidade | Bandas / filtros | Ganho por banda | MVP HTML/SVG/JS | Engine nativa (marco · TASK) | Estado |
|---|---|---|---|---|---|---|
| EQ-10 | Gráfico clássico (painel compacto) + Gráfico avançado 10 | 10: 31, 62, 125, 250, 500 Hz; 1, 2, 4, 8, 16 kHz | Clássico ±12 dB; avançado ±15 dB | **Obrigatório e interativo** | Clássico M1 · TASK-E04-01; editor avançado M3 · TASK-E02-07 | planejado |
| EQ-20 | Gráfico avançado 20 | 20, meia oitava (§5) | ±15 dB | **Obrigatório e interativo** | M3 · TASK-E04-04 | planejado (grade de frequências: proposta) |
| EQ-31 | Gráfico avançado 31 | 31, 1/3 de oitava ISO 266 / IEC 61260-1 (§6) | ±15 dB | **Obrigatório e interativo** | M3 · TASK-E04-04 | planejado |
| EQ-64 | Gráfico avançado 64 | 64, logarítmica densa de ≈1/6,3 oitava (§7); **não é 1/3 de oitava** | ±15 dB | **Obrigatório e interativo, com 64 controles reais** | **M5** · TASK-E12-10 | planejado no nativo (M5); grade proposta |
| EQ-PEQ32 | Paramétrico | até 32 filtros independentes (§10) | −24..+24 dB, onde o tipo usa ganho | **Obrigatório e interativo** | M3 · TASK-E04-04; 64 filtros só em M5 · TASK-E12-10 | planejado |

Regra de aceite do MVP: há cinco seleções alcançáveis (**Gráfico 10, Gráfico 20, Gráfico 31, Gráfico 64 e Paramétrico**). "64" quer dizer 64 bandas configuráveis, não 10 funcionais e 54 ornamentais. O paramétrico configura até 32 filtros. Ter a interface no MVP **não** antecipa o marco nativo, que continua M5 para o EQ-64.

## 2. Controles comuns a todos os modos

| Controle | Faixa / comportamento | Estado |
|---|---|---|
| Preamp | −24..+12 dB, independente das bandas, guardado por modo (§11) | planejado (faixa); por modo: proposto |
| Ganho EQ-10 no painel clássico | ±12 dB | planejado |
| Ganho no editor gráfico avançado (10/20/31/64) | ±15 dB | planejado |
| Ganho de filtro paramétrico | −24..+24 dB, sujeito a validação da engine (estabilidade e headroom) | planejado |
| On/Off do EQ | bypass real do nó, com crossfade (§13) | planejado |
| Reset | todas as bandas do modo ativo em 0 dB; o preamp não muda, salvo "Reset total" | proposto |
| Passo de teclado | ↑/↓ 0,5 dB; Shift+↑/↓ 2 dB; Home/End mínimo/máximo; duplo clique = 0 dB | **no MVP** (ver abaixo); no nativo: proposto |
| Leitura | frequência central e ganho em dB sempre visíveis por banda; o separador decimal segue o locale | planejado |
| Atualização | o valor e a curva mudam no mesmo quadro de UI; o áudio muda em até 10–50 ms (smoothing) | planejado |
| Headroom | aviso quando o pico da curva combinada passa de 0 dB sem preamp que compense; sugestão de preamp `−máx(0, pico_dB)` aplicada só com ação do usuário | proposto (base no [contrato](07-dsp-signal-contract.md)) |
| Clipping | indicador por estágio (pós-EQ e final); a proteção final true-peak continua na posição fixa | planejado |

**Passo de teclado no MVP.** O editor avançado ([referência do MVP](../design/06-mvp-reference.md) `mvp/advanced-eq.js`) usa `input[type=range]` com `min=-15`, `max=15`, `step=.5`, então ↑/↓ deslocam 0,5 dB e Home/End vão ao mínimo/máximo pelo comportamento nativo do controle — não há handler próprio para isso. Shift+seta aplica ±2 dB com clamp em ±15 dB, e duplo clique zera a banda em 0 dB. O editor clássico (`mvp/player.js`) também usa `step="0.5"` e duplo clique = 0 dB, mas **não** implementa o passo de 2 dB com Shift. A tabela acima vale como comportamento do MVP; levá-la ao nativo é requisito deste capítulo.

## 3. Topologia comum dos modos gráficos

- Cada banda é um filtro **peaking (bell) biquad de Q constante** (RBJ Audio EQ Cookbook). Os filtros ficam em cascata depois do preamp. Coeficientes e estados em **f64**, como manda o [engine](01-engine.md). **Proposto**: usar shelves nas bandas extremas fica como opção "a validar"; não é o padrão.
- O Q de cada modo vem da largura de banda nominal `N` em oitavas: `Q = √(2^N) / (2^N − 1)`.
- Os filtros vizinhos se sobrepõem. Por isso, a **resposta real nos centros não é igual ao valor do slider**. A curva mostrada é sempre a resposta calculada da cascata (§8), nunca a linha poligonal dos sliders. **Dependente de investigação**: uma "compensação de interação" (resolver por mínimos quadrados os ganhos internos para que a resposta nos centros siga os sliders) pode virar opção. Ela muda a semântica do preset e precisa de benchmark de CPU e de estabilidade.

## 4. EQ-10 — clássico e avançado 10

| Banda | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---|---|---|---|---|---|---|---|---|---|
| Rótulo | 31 Hz | 62 Hz | 125 Hz | 250 Hz | 500 Hz | 1 kHz | 2 kHz | 4 kHz | 8 kHz | 16 kHz |
| Centro efetivo (Hz) | 31,25 | 62,5 | 125 | 250 | 500 | 1000 | 2000 | 4000 | 8000 | 16000 |

- Fórmula: `f_k = 1000 · 2^(k−5)`, com k = 0..9 (oitavas exatas). Q ≈ 1,41 (N = 1). **Proposto**: os rótulos já estavam fixados; os centros efetivos e o Q são propostos.
- **Painel clássico** (planejado M1; geometria em [layouts](../design/02-layouts.md)): preamp + 10 sliders, On/Off, Auto, Reset, Presets ▾ e mini curva calculada. Não comprime 20/31/64 bandas no painel.
- **Avançado 10** (planejado M3): mesmas bandas e mesmo estado, escala ±15 dB, gráfico grande e tabela.
- **Coerência entre painéis** (proposto): se uma banda passa de ±12 dB no avançado, o painel clássico mostra o slider no fim da escala com o marcador "fora da escala do painel clássico" e o valor numérico real. O valor nunca é cortado em silêncio.

## 5. EQ-20 — meia oitava

Grade proposta: `f_k = 1000 · 2^((k−11)/2)`, com k = 0..19. São centros em meia oitava exata de 22,1 Hz a 16 kHz. Nos índices pares estão todas as bandas do EQ-10, o que facilita a conversão de presets. Q ≈ 2,87 (N = 1/2). Faixa útil: 22 Hz–16 kHz, válida em toda Fs ≥ 44,1 kHz.

| k+1 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---|---|---|---|---|---|---|---|---|---|
| Rótulo | 22 | 31 | 44 | 63 | 88 | 125 | 177 | 250 | 354 | 500 |
| Efetivo (Hz) | 22,1 | 31,25 | 44,2 | 62,5 | 88,4 | 125 | 176,8 | 250 | 353,6 | 500 |

| k+1 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
|---|---|---|---|---|---|---|---|---|---|---|
| Rótulo | 707 | 1k | 1,4k | 2k | 2,8k | 4k | 5,7k | 8k | 11,3k | 16k |
| Efetivo (Hz) | 707,1 | 1000 | 1414,2 | 2000 | 2828,4 | 4000 | 5656,9 | 8000 | 11313,7 | 16000 |

## 6. EQ-31 — 1/3 de oitava

Os centros nominais seguem a série de 1/3 de oitava de ISO 266 / IEC 61260-1, de 20 Hz a 20 kHz. O centro efetivo usa a base 10: `f_k = 1000 · 10^(k/10)`, com k = −17..13. Q ≈ 4,32 (N = 1/3). A UI mostra o rótulo nominal e, no tooltip ou na tabela, o centro efetivo e o Q.

| Nominal | 20 | 25 | 31,5 | 40 | 50 | 63 | 80 | 100 | 125 | 160 | 200 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Efetivo (Hz) | 19,95 | 25,1 | 31,6 | 39,8 | 50,1 | 63,1 | 79,4 | 100 | 125,9 | 158,5 | 199,5 |

| Nominal | 250 | 315 | 400 | 500 | 630 | 800 | 1k | 1,25k | 1,6k | 2k |
|---|---|---|---|---|---|---|---|---|---|---|
| Efetivo (Hz) | 251,2 | 316,2 | 398,1 | 501,2 | 631 | 794,3 | 1000 | 1258,9 | 1584,9 | 1995,3 |

| Nominal | 2,5k | 3,15k | 4k | 5k | 6,3k | 8k | 10k | 12,5k | 16k | 20k |
|---|---|---|---|---|---|---|---|---|---|---|
| Efetivo (Hz) | 2511,9 | 3162,3 | 3981,1 | 5011,9 | 6309,6 | 7943,3 | 10000 | 12589,3 | 15848,9 | 19952,6 |

Em 44,1 kHz, a banda de 20 kHz (19 952,6 Hz) passa do limite de 0,45 × Fs (19 845 Hz) e fica **indisponível** (§9).

## 7. EQ-64 — logarítmica densa (não é 1/3 de oitava)

Grade proposta: `f_k = 20 · 1000^(k/63)`, com k = 0..63. É uma distribuição logarítmica uniforme de 20 Hz a 20 kHz, com razão de 1,11588 entre vizinhos, ou seja, ≈ 0,158 oitava (≈ 1/6,3 de oitava). O Q padrão ≈ 9,12 corresponde a uma largura igual ao espaçamento. **Não descrever como 1/3 de oitava.** Rótulos com 3 algarismos significativos.

| # | Hz | # | Hz | # | Hz | # | Hz | # | Hz | # | Hz | # | Hz | # | Hz |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 20,0 | 9 | 48,1 | 17 | 116 | 25 | 278 | 33 | 668 | 41 | 1606 | 49 | 3861 | 57 | 9283 |
| 2 | 22,3 | 10 | 53,7 | 18 | 129 | 26 | 310 | 34 | 746 | 42 | 1792 | 50 | 4309 | 58 | 10360 |
| 3 | 24,9 | 11 | 59,9 | 19 | 144 | 27 | 346 | 35 | 832 | 43 | 2000 | 51 | 4808 | 59 | 11560 |
| 4 | 27,8 | 12 | 66,8 | 20 | 161 | 28 | 386 | 36 | 928 | 44 | 2232 | 52 | 5365 | 60 | 12900 |
| 5 | 31,0 | 13 | 74,6 | 21 | 179 | 29 | 431 | 37 | 1036 | 45 | 2490 | 53 | 5987 | 61 | 14390 |
| 6 | 34,6 | 14 | 83,2 | 22 | 200 | 30 | 481 | 38 | 1156 | 46 | 2779 | 54 | 6681 | 62 | 16060 |
| 7 | 38,6 | 15 | 92,8 | 23 | 223 | 31 | 537 | 39 | 1290 | 47 | 3101 | 55 | 7455 | 63 | 17920 |
| 8 | 43,1 | 16 | 104 | 24 | 249 | 32 | 599 | 40 | 1439 | 48 | 3460 | 56 | 8319 | 64 | 20000 |

- Pontos em aberto (**dependentes de investigação**): o Q padrão ≈ 9,12 em cascata de 64 biquads pode gerar ondulação (ripple) e ganho acumulado. O benchmark de CPU/latência e a validação numérica ficam em TASK-E12-10 (M5). No MVP, a cascata é calculada da mesma forma para desenhar a curva.
- Em 44,1 kHz só a banda 64 (20 kHz) fica indisponível; em 32 kHz, as bandas 62–64.

## 8. Curva calculada da resposta

- **Fonte da curva** (planejado): os coeficientes reais na Fs atual do nó. A resposta é `H(f) = G_preamp · ∏ H_k(e^{j2πf/Fs})`, avaliada em ≥ 512 pontos logarítmicos de 10 Hz até min(40 kHz, Fs/2), com o resultado em dB. Bandas indisponíveis entram como bypass. **Nunca é um desenho decorativo.**
- Camadas: resposta por banda ou filtro (fina, opcional), resposta combinada (principal) e marcador de 0 dB. O eixo X é logarítmico, com rótulos 20/50/100/200/500/1k/2k/5k/10k/20k. O eixo Y fica em dB, na escala do modo.
- **Previsão × medição**: a curva teórica tem o rótulo "Prevista (filtros)". Uma curva medida (sweep/impulso no tap pós-EQ, ou analyzer em tempo real) tem o rótulo "Medida" e mostra fonte, Fs e janela. As duas nunca são apresentadas como a mesma coisa. Medição em tempo real: **dependente de investigação** (TASK-E06-05/E17-09).
- Pontos arrastáveis: no modo gráfico, o arrasto vertical muda o ganho da banda mais próxima. No paramétrico, X muda a frequência, Y o ganho e roda/teclas o Q. Toda edição por arrasto tem equivalente por teclado e campo numérico.

## 9. Nyquist e sample rate

- Faixa de referência do editor para filtros ajustáveis: **10 Hz a min(40 kHz, 0,45 × Fs)**, sujeita à implementação real (planejado).
- Banda ou filtro acima do limite: fica **indisponível**, com o motivo exibido (ex.: "indisponível: 20 kHz > 19,8 kHz, limite de 0,45 × 44,1 kHz"). O valor guardado é preservado, o filtro entra em bypass e a curva não o inclui. **Proibido** mover a frequência em silêncio ou desenhar uma curva impossível.
- Troca de Fs (próxima faixa, troca de saída): o estado do modo é reavaliado fora da callback, os coeficientes são recalculados e trocados com crossfade, e a disponibilidade é anunciada. O EQ roda na taxa do stream, antes do resampler final ([contrato](07-dsp-signal-contract.md)).

| Fs | Limite | EQ-10 indisp. | EQ-20 indisp. | EQ-31 indisp. | EQ-64 indisp. |
|---|---|---|---|---|---|
| 32 kHz | 14,4 kHz | 16k | 16k | 16k, 20k | 62–64 |
| 44,1 kHz | 19,845 kHz | — | — | 20k | 64 |
| 48 kHz | 21,6 kHz | — | — | — | — |
| ≥ 88,2 kHz | ≥ 39,7 kHz (máx. 40 kHz) | — | — | — | — |

## 10. EQ-PEQ32 — paramétrico até 32 filtros

| Tipo | Frequência | Ganho | Q | Slope | Estado |
|---|---|---|---|---|---|
| Peaking / bell | 10 Hz–limite | −24..+24 dB | 0,1..18 | — | planejado |
| Low shelf / high shelf | 10 Hz–limite | −24..+24 dB | 0,1..18 (ou S equivalente, exibido) | — | planejado |
| Low pass / high pass | 10 Hz–limite | — | 0,1..18 (12 dB/oct) | 12 dB/oct nativo (biquad); 24/36/48 dB/oct por cascata Butterworth/Linkwitz-Riley | 12 dB/oct planejado; demais **a validar** |
| Notch | 10 Hz–limite | — | 0,1..18 | — | planejado |
| Band pass | 10 Hz–limite | — | 0,1..18 | — | planejado |

- Limite: até **32 filtros armazenados e ativos** no M3. Acima disso, "Adicionar" fica desabilitado e mostra o motivo ("limite de 32 filtros nesta versão"). **64 filtros só em M5** (TASK-E12-10), depois de benchmark.
- Por filtro: incluir, remover, duplicar, ativar/bypass individual, tipo, frequência, ganho (só nos tipos que usam ganho), Q, slope (quando pertinente) e canal **Both/Left/Right**. Editor multicanal: M5.
- Validação: valor fora da faixa, NaN ou texto inválido é rejeitado no campo com a mensagem e a faixa válida, e o valor anterior é mantido. Import inválido mantém o preset anterior e aponta o erro por JSON pointer (já planejado).
- Estabilidade: coeficientes verificados (polos dentro do círculo unitário) antes do swap. Filtro instável é recusado com erro, não aplicado.

## 11. Modelo de dados e persistência

Proposto (vale para TASK-E04-04; o [schema](../../schemas/audio-preset.schema.json) atual, v1, só representa um paramétrico de até 32 bandas, sem modo, sem `enabled` por filtro e sem `slope`):

```text
EqState {
  active_mode: graphic10 | graphic20 | graphic31 | graphic64 | parametric
  enabled: bool
  modes: {
    graphic10 { preamp_db, gains_db[10] }
    graphic20 { preamp_db, gains_db[20] }
    graphic31 { preamp_db, gains_db[31] }
    graphic64 { preamp_db, gains_db[64] }
    parametric { preamp_db, filters[≤32] { id, enabled, type, frequency_hz, gain_db?, q, slope_db_oct?, channel } }
  }
}
```

- Cada modo guarda os seus valores **de forma independente**. Trocar de modo não apaga nem converte nada sem pedido. Converter um preset entre modos só acontece com uma prévia, sem alegar equivalência exata (já planejado no [contrato](07-dsp-signal-contract.md)).
- Presets: namespaces separados por modo (`eq10`, `eq20`, `eq31`, `eq64`, `peq`), com salvar, carregar, renomear, duplicar, excluir, importar/exportar versionado, reset e desfazer/refazer na sessão. Presets de fábrica são somente leitura (duplicar para editar). O indicador "*" marca preset modificado.
- Precedência (planejado): sessão > faixa > gênero > dispositivo > global. "Auto" escolhe pelo contexto e mostra a origem. O perfil por dispositivo usa o stable ID ([saídas](04-outputs-hires.md)).
- Pendência: `audio-preset.schema.json` v2 com `mode`, `gains_db`, `enabled`, `slope_db_oct` e migração v1→v2, entregue com TASK-E04-04. Este PR altera só Markdown.

## 12. UI/UX do equalizador

- **Seletor de modo** no editor avançado (960×600): segmented control "Gráfico 10 · Gráfico 20 · Gráfico 31 · Gráfico 64 · Paramétrico", com setas, Tab e estado selecionado visível. Trocar de modo reconstrói os controles, a curva e as leituras, sem interromper a reprodução.
- **Classic** mantém 10 bandas visíveis e não ganha rolagem. 20/31/64 aparecem só no editor ampliado.
- **64 bandas**: 64 sliders reais com rolagem horizontal e zoom (e opção de agrupamento visual por oitava). O alvo mínimo é preservado (24 DIP desktop, 48 dp Android; [acessibilidade](../design/03-accessibility-i18n.md)). Um minimapa da curva mostra a janela visível. A virtualização de renderização é permitida; a de estado não (todos os 64 valores existem e são editáveis por teclado).
- **Paramétrico**: gráfico com nós arrastáveis + **tabela de filtros** (nº, ativo, tipo, frequência, ganho, Q, slope, canal, remover) + contador visível "n/32".
- **Acessibilidade**: cada slider anuncia nome, frequência e valor ("Banda 12 de 64, 66,8 hertz, mais 3,0 decibéis"). Foco visível, setas, Home/End e estados active/disabled/focus/indisponível sem depender só da cor. Leitores de tela não recebem eventos por quadro.
- **Estados**: desligado (bandas esmaecidas, mas editáveis, **proposto**), preset modificado, bloqueado pelo modo transparente (aviso + "Sair do modo transparente"), carregando preset, erro de preset, banda indisponível por Nyquist, headroom baixo e clipping.
- **Controles (DSP-UX)**: knobs, chaves e edição exata sob demanda não substituem os sliders por banda: os gráficos 10/20/31/64 mantêm todos os sliders reais e editáveis, e o PEQ32 mantém todos os filtros (DSP-UX-07, DSP-UX-AC-10). Um preset válido com valor fora do passo do gesto é preservado, sem quantizar (DSP-UX-14). Contrato completo: [painel DSP §8](../design/05-dsp-panel.md#8-acesso-e-controles-dsp-ux).
- Detalhes do painel DSP e dos medidores: [painel DSP](../design/05-dsp-panel.md).

## 13. Bypass, transições, bit-perfect e segurança

- Ligar/desligar e trocar de modo: crossfade equal-power entre a cadeia antiga e a nova (10–50 ms, valor exato em TASK-E04-03), sem cliques e sem reiniciar a reprodução. Mudança de parâmetro interpolada (smoothing) em 10–50 ms.
- EQ desligado = **bypass real**. A saída é sample-exact igual à entrada no caminho float, e o caminho inteiro transparente não passa pelo grafo ([saídas](04-outputs-hires.md)).
- Ligar o EQ em modo transparente: diálogo "Sair do modo transparente?". Sem confirmação, nada muda. Com confirmação, o selo é retirado. A `vynamp-output` nunca aplica ganho, dither ou resampling e não cai de modo em silêncio, então o player declara o caminho real ([DSP §bit-perfect](03-dsp.md#bit-perfect-exclusivo-e-saída)).
- Fase e latência: os biquads IIR têm latência zero e fase mínima (a fase muda junto com a magnitude). Modos lineares em fase ficam no FIR ([catálogo](09-dsp-catalog.md)), não no EQ gráfico.
- CPU: 64 biquads por canal é pouco em CPU moderna, mas o número real será **medido**, sem estimativa assumida (TASK-E12-10).

## 14. MVP HTML/SVG/JS — contrato documental

Contrato do protótipo, hoje cumprido pela [referência do MVP](../design/06-mvp-reference.md) (`vynamp-mvp-reference@0d9860b`, rev.38). Os itens abaixo continuam valendo como critério de aceite: verificam o que o MVP já faz e barram regressão.

1. Cinco seleções alcançáveis por mouse, toque e teclado. Cada uma contém exatamente 10, 20, 31 ou 64 controles gráficos, ou até 32 filtros. Verificado no pin: `modes=['10','20','31','64','parametric']` e, para 20 e 64, centros gerados por `20*1000**(i/(n-1))`, com 64 `input[type=range]` reais (`mvp/advanced-eq.js`).
2. Estado independente por modo, persistido ao recarregar a página e ao trocar de tema ou layout.
3. Curva SVG calculada com as mesmas fórmulas deste capítulo (ou `BiquadFilterNode.getFrequencyResponse` quando ligado ao Web Audio), determinística.
4. **Verdade funcional**: o protocolo declara exatamente quais modos processam som (ex.: `BiquadFilterNode` conectado e testado). Os demais exibem o rótulo visível **`[DEMO — sem processamento de áudio neste modo]`** (en: `[DEMO — no audio processing in this mode]`). Um controle manipulável não prova processamento audível. Um gráfico SVG não é chamado de DSP real.
5. Evidência exigida: contagem de controles por modo, capturas, a matriz PASS/FAIL/BLOCKED e o estado do console. Mockup estático não é aceito.

### Status da revisão 15 (2026-10-28)

Todos os itens abaixo são **evidência relatada da referência externa rev. 14/15, não verificada neste repositório** (protótipo Codex rev.14/15, histórico; fonte atual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 rev.38; o código `interactive/*` e os `*-validation.json` do protótipo não estão aqui; a validação relatada foi em Chrome/Windows com fixtures controladas). Separação: **especificado** · **implementado no protótipo (relatado)** · **validado no protótipo (relatado)** · **nativo (hoje: nada)**.

O que já foi conferido **neste repositório** sobre o pin: os cinco modos existem; 20 e 64 têm centros logarítmicos `20*1000**(i/(n−1))`; as bandas são `input[type=range]` reais com `step=.5`; Shift+seta aplica ±2 dB com clamp em ±15 dB; duplo clique zera; e o rótulo `[DEMO — sem processamento de áudio neste modo]` está no código (`mvp/advanced-eq.js`, `mvp/dsp-workspace.js`). O percurso de áudio real do EQ principal também está no código: o status declara `[REAL] … uma instância Web Audio` quando o nó está habilitado, a cadeia global ligada e o nó na rota viva (`mvp/eq-registry.js`). **O que continua relatado** é a *medição*: latência, CPU, qualidade e as validações com fixtures não foram executadas aqui.

| Item | Especificado | Protótipo: implementado (relatado) | Protótipo: validado (relatado) | Nativo |
|---|---|---|---|---|
| EQ gráfico 10/20/31/64 como instância do grafo modular | este capítulo | áudio real dentro do grafo: biquads peaking em série, ±15 dB por banda, preamp −24/+12 dB; EQ10 em centros clássicos, EQ31 em 1/3 de oitava, EQ20/EQ64 em distribuição logarítmica 20 Hz–20 kHz; Q e centros no inspetor | render offline com EQ64 e WAV estéreo local pela cadeia real (`modular-audio-validation`, `modular-live-validation`); controles reais de 64 bandas (`modular-editing-validation`) | nada; EQ10 M1, EQ20/31 M3, **EQ-64 continua M5** (§15) |
| PEQ até 32 filtros como instância do grafo | §10 | áudio real: peaking, shelves, passes, notch, bandpass; Both/Left/Right; bypass individual; frequência/ganho/Q; **sem seletor de slope arbitrário** nesta visão | PEQ por canal em render offline; 32 filtros reais (`modular-audio-validation`, `modular-editing-validation`) | nada; M3 (TASK-E04-04) |
| Editor avançado anterior (modos e rótulos `[DEMO]`) | §14 itens 1–5 | conserva rótulos e ligação de áudio originais: o EQ clássico de 10 bandas tem adaptação de áudio web; outros modos sem adaptação seguem `[DEMO — sem processamento de áudio neste modo]` | nenhuma validação relatada específica | nada |

Limites que não podem ser omitidos:

- A capacidade real de EQ/PEQ é das **instâncias do grafo**; a nova capacidade de EQ real não retroage ao editor avançado anterior. Não marcar retroativamente um editor desconectado como funcional no áudio; um controle manipulável continua não provando processamento.
- A referência não implementa slopes, presets globais/por dispositivo, auto-seleção, FIR nem LUFS; esses requisitos continuam no contrato deste capítulo e dos demais.
- O marco nativo **não muda**: EQ-64 gráfico e qualquer PEQ de 64 filtros seguem em **M5** (TASK-E12-10) salvo decisão formal (D-DSP-01). Processamento no Web Audio não é evidência de latência, CPU ou qualidade sonora do nativo.
- Contrato do grafo: [DSP, grafo modular](03-dsp.md#grafo-modular-protótipo--nativo). Testes: [matriz §2.2](../quality/05-dsp-test-matrix.md#22-grafo-modular-e-vu-int) e [§2.1](../quality/05-dsp-test-matrix.md#21-aceite-de-acesso-e-controles-dsp-ux-ac), planejados para o nativo.

## 15. Roadmap: MVP visual × engine nativa

| ID | MVP HTML/SVG/JS | Nativo: processamento | Nativo: UI | Observação |
|---|---|---|---|---|
| EQ-10 | Obrigatório agora | M1 · TASK-E04-01 | Classic M1 (TASK-E02-03); avançado M3 (TASK-E02-07) | — |
| EQ-20 | Obrigatório agora | M3 · TASK-E04-04 | M3 · TASK-E02-07 | grade §5 a aprovar em E04-04 |
| EQ-31 | Obrigatório agora | M3 · TASK-E04-04 | M3 · TASK-E02-07 | — |
| EQ-64 | Obrigatório agora (64 controles reais; rev. 15 relata áudio real no grafo) | **M5** · TASK-E12-10 | Até M5: seleção visível no nativo, com o estado "indisponível nesta versão (M5)" | marco M5 mantido até decisão formal |
| EQ-PEQ32 | Obrigatório agora (≤ 32 filtros) | M3 · TASK-E04-04 | M3 · TASK-E02-07 | 64 filtros: M5 · TASK-E12-10 |

Roadmap DSP completo: [roadmap](../delivery/01-roadmap.md#roadmap-dsp-e-equalizadores).

## 16. Testes e aceite por modalidade

Testes planejados (nenhum foi executado). Oráculo numérico independente (TASK-E10-03). Cenário agregado: QA-032 em [estratégia de testes](../quality/01-test-strategy.md). Matriz geral: [testes DSP](../quality/05-dsp-test-matrix.md).

| ID | Objetivo | Método | Evidência | Critério de aprovação | Modos |
|---|---|---|---|---|---|
| EQ-T-01 | Seletor com cinco modos | UI test por ponteiro e teclado; ler o nome acessível de cada opção | captura + log da árvore de acessibilidade | exatamente 10/20/31/64/Paramétrico alcançáveis; seleção visível | todos |
| EQ-T-02 | Contagem real de controles | contar sliders editáveis por modo e editar o primeiro, o do meio e o último | contagem registrada + capturas | 10, 20, 31 e 64 controles editáveis; nenhum ornamental | EQ-10/20/31/64 |
| EQ-T-03 | Centros nominais e efetivos | comparar a tabela da UI e a do estado serializado com §4–§7 | diff da tabela | igualdade nos rótulos; efetivos com erro ≤ 0,1 % | EQ-10/20/31/64 |
| EQ-T-04 | Resposta numérica dos filtros | impulso/sweep por banda a ±6 e ±15 dB em 44,1/48/96/192 kHz | CSV da resposta + referência | curva-alvo ±0,2 dB nos pontos definidos; sem NaN; estável perto de Nyquist | todos |
| EQ-T-05 | Nyquist e Fs | trocar Fs (32/44,1/48 kHz) com bandas altas ativas | log de disponibilidade + captura | bandas da tabela §9 indisponíveis com motivo; valores preservados; sem reajuste oculto | todos |
| EQ-T-06 | Limites e entradas inválidas | valores fora da faixa, NaN, texto, Q 0,05/19, frequência 5 Hz/45 kHz | log da validação | rejeição com mensagem; valor anterior mantido | todos |
| EQ-T-07 | Paramétrico 32 filtros | adicionar até 32, tentar o 33º, remover, ativar/desativar, mudar tipo/canal/slope | contador + estado serializado | 32 aceitos; 33º bloqueado com motivo; ganho só nos tipos aplicáveis | EQ-PEQ32 |
| EQ-T-08 | Independência entre modos | editar todos os modos, alternar 20 vezes, recarregar | diff do estado antes/depois | nenhum valor perdido ou convertido sem pedido | todos |
| EQ-T-09 | Curva sincronizada | mudar cada banda e cada filtro e comparar a curva com o cálculo de referência | captura + dump dos pontos | curva = cascata calculada ±0,1 dB; atualização no mesmo quadro | todos |
| EQ-T-10 | Bypass, reset e presets | On/Off, Reset, salvar/carregar/renomear/duplicar/excluir/importar/exportar | estado + arquivos de preset | bypass sample-exact (nativo); reset = 0 dB; import inválido mantém o anterior | todos |
| EQ-T-11 | Sem cliques | alternar On/Off, modo e preset durante tom de 1 kHz e ruído rosa | captura + detector de descontinuidade | nenhum degrau acima do limiar de E10-03; sem zipper noise | todos (nativo) |
| EQ-T-12 | Headroom com boosts | todas as bandas a +15 dB, preamp 0 | medidores pós-EQ e final + QA-027 | aviso de headroom visível; final ≤ teto +0,1 dBTP com a proteção ligada; clipping indicado com ela desligada | todos |
| EQ-T-13 | Acessibilidade | NVDA/Narrator/VoiceOver/TalkBack; só teclado | gravação do anúncio | nome + frequência + dB anunciados; Home/End/setas funcionam; foco visível | todos |
| EQ-T-14 | Verdade do MVP | inspecionar o grafo Web Audio e os rótulos por modo | matriz PASS/FAIL/BLOCKED + console | modos não ligados mostram `[DEMO — sem processamento de áudio neste modo]`; nenhum claim indevido | MVP, todos |
| EQ-T-15 | CPU e latência EQ-64/PEQ | benchmark 64 bandas/32 filtros por canal a 44,1–384 kHz | relatório de bench (protocolo de [performance](../quality/02-performance-observability.md)) | dentro do budget de callback; números publicados antes do aceite M5 | EQ-64, EQ-PEQ32 |
