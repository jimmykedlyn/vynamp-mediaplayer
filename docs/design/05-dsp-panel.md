# Painel Audio DSP, equalizador e medidores — UI/UX

Versão: 0.7.3 · Data: 2026-11-05 · Estado: especificação proposta. No nativo, nada está implementado. No MVP web de referência (`vynamp-mvp-reference@0d9860b`, rev.38, fora deste repositório) a UI DSP existe; o código foi conferido onde este documento o cita, com as ressalvas da §6.

Base: [visual](01-visual-system.md), [layouts](02-layouts.md), [acessibilidade](03-accessibility-i18n.md), [DSP](../audio/03-dsp.md), [equalizadores](../audio/08-equalizers.md), [vitrola](../audio/10-vitrola.md) e [visualizadores](../extensions/04-visualizer-contract.md). A UI nativa da cadeia e do EQ avançado é planejada em TASK-E02-07 (M3).

## 1. Princípios

- **Divulgação progressiva** (Hick): a visão **Básica** mostra preset de uso, On/Off geral, EQ e intensidades. A visão **Avançada** mostra parâmetros completos, roteamento, latências e medição.
- **Consistência** (Jakob): o EQ clássico mantém a metáfora de 10 sliders verticais do player clássico. Os demais módulos usam os mesmos sliders, toggles e menus do Vynamp.
- **Fitts**: alvos ≥ 24 DIP no desktop e ≥ 48 dp no Android, mesmo no editor de 64 bandas (rolagem/zoom em vez de encolher).
- **Gestalt**: os módulos são agrupados por família e a cadeia aparece como uma linha de nós na ordem real.
- **Feedback**: todo ajuste reflete valor, curva e estado na hora; o áudio muda em até 50 ms.
- **Honestidade**: "processado" × "transparente" × "DEMO" nunca se confundem.

## 2. Estrutura do painel Audio DSP

| Área | Conteúdo | Estado |
|---|---|---|
| Cabeçalho | toggle **DSP geral** (bypass global com crossfade), preset de uso, A/B, desfazer/refazer, perfil do dispositivo atual | proposto |
| Presets de uso | **Original/Neutro** (DSP desligado), **Fones**, **Caixas estéreo**, **Home Theater** (só se o caminho multicanal for suportado; senão, indisponível com motivo), personalizados | proposto |
| Entrada | **Áudio e efeitos** no cabeçalho do layout estendido (ícone + rótulo) e quatro atalhos junto ao instrumento: **EQ**, **Dinâmica**, **Estéreo**, **Vitrola** ([§8](#8-acesso-e-controles-dsp-ux)) | proposto |
| Navegação | Oito famílias: **Ganho/EQ · Dinâmica · Acústica · Estéreo · Tempo/formato · Criativos · Vitrola** (Vinil, Válvula) **· Saída**, mais lista pesquisável de módulos. Medidores não são família: são área própria ([§6](#6-medidores-e-visualização)) | proposto |
| Cadeia | inspector linear da [cadeia real](../audio/03-dsp.md#cadeia-real-de-processamento): nome, on/bypass (ícone + texto), parâmetros-chave e latência declarada. Arrastar para reordenar **só** nos nós reordenáveis e validados | planejado (E02-07); reordenação: proposta |
| Rodapé | headroom estimado, indicador de clipping por estágio, latência total, CPU do DSP, buffer/underruns | proposto |

As oito famílias organizam o catálogo; não são limite de oito efeitos nem switches de processamento, e não alteram a ordem da cadeia. Estados de cada módulo (ícone + texto, nunca só cor): **Ativo** · **Inativo** · **Bypass** · **Incompatível** (motivo: transparente, DSD, multicanal, Fs) · **Erro** (bypass seguro + diagnóstico) · **DEMO** (somente no MVP) · **Indisponível nesta versão** (marco).

## 3. Equalizador

- **Classic compacto** (576×180): preamp, 10 bandas, On/Off, Auto, Reset, Presets ▾ e mini curva calculada.
- **Editor avançado** (960×600): seletor **Gráfico 10 · Gráfico 20 · Gráfico 31 · Gráfico 64 · Paramétrico (até 32 filtros)**, gráfico grande (log de 10 Hz a min(40 kHz; 0,45×Fs), eixo em dB), curva prevista e curva medida separadas, pontos arrastáveis, tabela de bandas/filtros, preamp, presets por modo, reset, bypass, aviso de headroom/clipping e contador "n/32" no paramétrico. Especificação completa: [equalizadores §12](../audio/08-equalizers.md#12-uiux-do-equalizador).
- **64 bandas**: 64 sliders reais com rolagem horizontal, zoom e minimapa. Não pode haver 10 sliders reais e o resto decorativo.
- **Android**: 10 bandas com rolagem horizontal explícita; 20/31/64 e paramétrico em editor de tela cheia.

## 4. Cards Vinil e Válvula

Dois cards independentes, conforme [vitrola §5](../audio/10-vitrola.md#5-ux-hospedeira):

- **Vinil**: intensidade, ruído/estalos/rotação progressivos, rpm, rituais (com aviso de que alteram a duração), lado/disco atual em texto, desgaste e "Comprar disco novo". Nenhum ruído com o card desligado.
- **Válvula**: aquecimento (Aquecendo… / Pronto), perfil, saturação, sag e transformador. Qualidade de oversampling automática, mostrada como informação.

Cada card tem bypass próprio, presets, A/B com loudness igualado, restaurar padrões e o selo **"altera o áudio intencionalmente"**. É proibido ilustrar toca-discos ou válvulas.

## 5. Mensagens e fluxos

| Situação | Mensagem (pt-BR) | Ação |
|---|---|---|
| Ligar DSP em transparente | "Sair do modo transparente? O processamento será ativado e o selo removido." | Confirmar / Cancelar |
| Boost sem headroom | "A curva soma +X dB. Risco de clipping." | "Ajustar preamp (−X dB)" / Ignorar |
| Clipping detectado | "Clipping em <estágio>" (indicador persistente por 2 s) | abrir medidores |
| Latência alta | "Latência do DSP: N ms (FIR/lookahead)" | ver nós |
| Banda acima de Nyquist | "Indisponível: F > limite de 0,45 × Fs" | — |
| Módulo incompatível | "Indisponível: <motivo>" | link para a causa |
| Preset inválido | "Preset não carregado: <campo> inválido. Preset anterior mantido." | detalhes |
| vitrola desativado | "Vitrola desativado: caminho bit-perfect / DSD nativo / transparente" | "Sair do modo transparente" |
| MVP sem áudio | `[DEMO — sem processamento de áudio neste modo]` | — |

Fluxos principais: (1) primeira execução com EQ plano e desligado; (2) escolher o preset de uso "Fones" → EQ + AutoEQ importado + crossfeed (quando existir) → A/B; (3) abrir o editor avançado → trocar para 64 → ajustar → salvar preset do modo; (4) ligar o vitrola → intensidade sutil → painel avançado.

## 6. Medidores e visualização

Instrumentos de exibição **não são efeitos DSP** e nunca alteram a saída. O comportamento de interação vem do MVP ([referência](../design/06-mvp-reference.md) `mvp/player.js`): clique no instrumento alterna para o próximo VU, clique com Shift alterna para o anterior, botão direito abre a escolha entre os modelos, e a tecla **V** percorre os modelos — **V** avança, **Shift+V** recua, e **Alt+V** desliga o VU. O menu de modelos abre com **Ctrl+Shift+V** (⌘+Shift+V no macOS), não com Shift+V; **Ctrl+V** sozinho não faz nada, porque o atalho de edição tem precedência. Contrato técnico: [visualizadores](../extensions/04-visualizer-contract.md) (TASK-E06-05/06, E17-09).

**Duas convenções que os arquivos de skin atribuíam a este painel não existem no MVP** e por isso não entram por herança ([D-MVP-02](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05)): "0 VU = −18 dBFS ajustável" (o MVP não tem constante de calibração nem ajuste; a escala de cada medidor vem de `scaleDB`/`scaleAngles` no `meta.json`) e "slot de 28 DIP" (o instrumento é fluido: `#meter-stage` com `width:100%`/`height:auto`). Se qualquer uma for desejada no nativo, é decisão nova registrada em [05-decisions](../product/05-decisions.md), não requisito herdado.

**Contagem de modelos.** O contrato VU-INT abaixo fala em **12 VU meters aprovados**, número herdado da rev.15. No pin atual (rev.38) o pacote tem **16 medidores e os 16 são selecionáveis** ([§3 da referência](06-mvp-reference.md#3-pacote-de-referência)). A divergência está registrada em [D-MVP-04](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05); a matriz de aceite VU-INT-06 não é reescrita aqui.

| Medidor | Definição / metodologia | Estado |
|---|---|---|
| Spectrum FFT | janela Hann, FFT 2048/4096 padrão, smoothing/hold/decay em unidades reais, escala log/linear | planejado |
| VU (balístico) | integração ~300 ms. **Não** é peak. **Referência de 0 VU: a definir.** O MVP não fixa um valor dBFS: cada medidor traz a própria escala em `scaleDB`/`scaleAngles` no `meta.json` (os analógicos vão de −20 a +3 dB). "0 VU = −18 dBFS (com −14/−20 opcionais)" era convenção da skin e **não** existe no MVP; adotá-la é decisão nova | escala do MVP verificada; referência nativa pendente |
| Peak / PPM | amplitude de sample peak com hold | planejado |
| True Peak (dBTP) | oversampling ≥ 4× conforme ITU-R BS.1770 (algoritmo pinado na fixture); medido no tap final | planejado |
| Loudness (LUFS) | momentary 400 ms, short-term 3 s, integrated com gating (BS.1770 / EBU R128), LRA (EBU Tech 3342) | planejado (analyzer E08-03); medidor em tempo real: proposto |
| Osciloscópio, goniômetro/vectorscope, correlação | do tap escolhido | planejado (E06-05/06) |
| Curva do EQ/FIR | calculada dos filtros reais; "prevista" × "medida" | planejado |
| Redução de ganho, CPU, buffer, latência | contadores RT coletados fora da callback | planejado |

- **Pontos de medição**: source / pre-EQ / post-EQ / **final (padrão)**. A unidade e a Fs aparecem no tooltip. Medidores pré/pós por módulo (ex.: pré/pós-vitrola) são **propostos**.
- **Taxa de UI**: 30 fps padrão, 60 opcional, 10 em economia; suspenso quando minimizado ou em background (Android: zero render). Reduced motion: estático com números.
- **Coerência**: VU, peak, true peak e LUFS **não** mostram o mesmo número com rótulos diferentes.
- Orçamento: visualizer ≤ 10% de CPU total agregado ([performance](../quality/02-performance-observability.md)).

### 6.1 Integração visual dos VU meters (VU-INT)

Contrato de apresentação dos 12 VU meters estéreo aprovados, vindo da referência externa rev. 15. Terminologia em [GLOSSARY](../GLOSSARY.md) (acabamento de VU).

| Status | Situação |
|---|---|
| Especificado | VU-INT-01..06 abaixo |
| Implementado no protótipo (relatado) | três acabamentos (Original, Metal adaptativo, Integrado ao painel, este o padrão), preferência persistida em Aparência; evidência relatada da referência externa rev. 15, não verificada neste repositório |
| Validado no protótipo (relatado) | 144 combinações de 12 skins × 12 VUs, claro/escuro/mobile, reversão ao original; Chrome/Windows, não verificado aqui |
| Nativo | nada implementado |

- **VU-INT-01 — Arquivos aprovados intactos:** preservar byte a byte os 12 SVGs aprovados; não editar escalas, ponteiros, vidro, LEDs, marcações, coordenadas, proporções nem identidade dos mostradores.
- **VU-INT-02 — Só o gabinete externo:** adaptar apenas gabinete, junta/contorno e rótulos externos na cópia exibida; não apagar elementos, recortar instrumentos nem simplificar desenhos. Madeira e champagne preservam seus materiais em todos os acabamentos.
- **VU-INT-03 — Sem moldura concorrente:** nenhuma moldura, sombra ou fundo de container adicional; o host fornece dimensão, foco e interação, o SVG fornece o instrumento.
- **VU-INT-04 — Tokens semânticos:** usar tokens da skin ([sistema visual](01-visual-system.md#acabamento-de-vu-meters)); proibido recolorir LEDs, escala ou ponteiros para combinar com o tema.
- **VU-INT-05 — Interação preservada:** tamanho natural responsivo pelo viewBox, opção Ampliar, clique para alternar e botão direito para escolher entre os 12 modelos; o acabamento não altera ordem nem funcionamento do menu. O espaço dos VUs estéreo originais não é reduzido por controles permanentes do DSP (DSP-UX-20).
- **VU-INT-06 — Matriz de aceite:** validar as 144 combinações de 12 skins × 12 VUs, temas claros/escuros, mobile e restauração do original; legibilidade de detalhes pequenos vem da ampliação, nunca de distorcer o desenho. A adaptação é de apresentação em tempo de execução e não é embutida nos pacotes de tema.

Acabamento de VU é apresentação, nunca efeito DSP.

## 7. Acessibilidade

Teclado completo (Tab entre grupos, setas/Home/End nos sliders), leitores de tela com nome, valor e unidade, contraste conforme os tokens, estados sem depender da cor, tooltips com unidade, sem eventos acessíveis por quadro, foco que nunca some ao trocar de modo e 200% de zoom sem perder controles.

## 8. Acesso e controles (DSP-UX)

Origem: ajuste pedido no protótipo ("muitos inputs", botões redondos e slides, acesso ao DSP escondido). Interpretação: **knobs circulares, sliders e chaves apropriados ao parâmetro**, com edição numérica exata sob demanda. Não transformar toda ação em botão circular nem trocar os sliders dos equalizadores gráficos por knobs. Status de todos os itens: especificado. No protótipo, implementação e validação são relatadas pela referência externa rev. 14/15 (não verificadas neste repositório); no nativo, nada. A referência web não prova processamento no produto nativo. Casos de aceite: [matriz DSP](../quality/05-dsp-test-matrix.md).

### 8.1 Acesso visível e navegação por famílias

- **DSP-UX-01 — Entrada principal:** **Áudio e efeitos** no cabeçalho do layout estendido, com ícone e rótulo visível; preservar o acesso por transporte/preferências e o painel compacto de EQ.
- **DSP-UX-02 — Atalhos contextuais:** junto ao instrumento, **EQ** (abre o editor avançado), **Dinâmica**, **Estéreo** e **Vitrola** (levam ao módulo/família). Na referência, os destinos iniciais são compressor, largura estéreo e vinil. Sem duplicar estado de parâmetros entre acessos; Vitrola é entrada para a área existente, sem fundir os simuladores.
- **DSP-UX-03 — Categorias:** oito famílias no topo (Ganho/EQ, Dinâmica, Acústica, Estéreo, Tempo/formato, Criativos, Vitrola, Saída), com lista completa pesquisável e acesso ao EQ avançado. São navegação, não switches.
- **DSP-UX-04 — Seleção coerente:** ao escolher uma família, manter o módulo atual se pertencer a ela; senão abrir um módulo inicial apropriado. Destacar família e módulo, manter o item visível na lista e impedir que busca anterior esconda o destino.
- **DSP-UX-05 — Espaço e foco:** adaptar a navegação às resoluções suportadas sem esconder parâmetros essenciais nem reduzir alvos; em telas estreitas, a navegação de módulos abre/fecha. Fechar devolve o foco ao acionador; se invisível, à entrada principal.

As oito famílias não limitam o catálogo. A referência relata 39 módulos e 198 campos: números do protótipo, que não congelam a arquitetura nativa.

### 8.2 Escolha do controle conforme o parâmetro

- **DSP-UX-06 — Knob circular vetorial:** ganho, frequência, Q, intensidade, drive, ratio e outros contínuos; rótulo, unidade, posição e valor legíveis; gesto vertical preferido, sem exigir percurso circular. Tokens em [sistema visual](01-visual-system.md#knobs-de-controle-dsp).
- **DSP-UX-07 — Slider:** tempos, dry/wet, largura estéreo, balanço/pan e parâmetros de comparação linear. Equalizadores gráficos mantêm um slider por banda, todas editáveis.
- **DSP-UX-08 — Chave liga/desliga:** opções binárias com nome e estado acessíveis; "habilitado" não se confunde com "bypass" e o texto torna o efeito inequívoco.
- **DSP-UX-09 — Campo textual ou seletor:** nomes, metadados, tipos de filtro, canais, modelos e opções discretas mantêm componentes próprios; reduzir inputs não proíbe controles necessários.
- **DSP-UX-10 — Valor clicável + editor exato:** readout discreto junto do knob ou slider; clicar abre edição numérica com unidade, faixa e validação. Nenhuma grade de inputs numéricos permanentemente exposta.

Os metadados do parâmetro (tipo, unidade, mínimo, máximo, passo, default, escala, disponibilidade) são a fonte do componente; a aparência não muda significado, faixa nem precisão armazenada, e limites técnicos não são inferidos somente pela aparência de um knob. Princípios aplicados na fonte: Jakob (controles familiares de áudio), Fitts (alvos adequados, entrada principal visível), Hick (famílias, edição exata sob demanda), Gestalt (parâmetros relacionados agrupados) e Nielsen (estado, prevenção de erro, reversibilidade, foco), além de acessibilidade. Parâmetros relacionados são agrupados por função, com ordem de leitura lógica.

### 8.3 Interação, precisão e validação

- **DSP-UX-11 — Mouse/toque:** arraste vertical no knob e arraste no slider; ajuste fino com Shift no desktop; duplo clique restaura o default do parâmetro, com restauração também por ação acessível a teclado/toque.
- **DSP-UX-12 — Teclado:** controles contínuos com foco e ajuste por setas, Home/End e passos maiores; Shift para passo fino. Atalhos globais de reprodução não capturam a edição de texto/número. Editor exato: Enter confirma, Escape cancela, foco restaurado ao fechar.
- **DSP-UX-13 — Escala:** mapeamento logarítmico para frequência e onde tecnicamente apropriado, preservando valor real e unidade anunciados; posição normalizada não é o valor serializado; documentar mapeamento e passos.
- **DSP-UX-14 — Precisão importada:** preset válido com valor entre passos do gesto é preservado ao carregar e apresentar, sem arredondar ou quantizar ao renderizar; o passo de interação não é restrição de validade do arquivo ou da engine.
- **DSP-UX-15 — Editor exato:** valida número finito, faixa, unidade e relações entre parâmetros; na rejeição mantém o editor aberto, explica e preserva o último estado válido; cancelar não altera; sem confirmação de sucesso para valor recusado.
- **DSP-UX-16 — Validação conjunta:** crossovers e parâmetros dependentes validados como configuração consistente (ex.: crossover inferior estritamente menor que o superior; igualdade ou inversão rejeitadas sem sobrescrever com default). Readout, controle, gráfico e estado persistido mostram o mesmo valor efetivo.
- **DSP-UX-17 — Defaults e presets:** manter bypass, reset, presets, importação/exportação e independência de estado. Reset de parâmetro difere de reset do módulo e da cadeia; trocar tema, família ou ponto de entrada não reinicializa ajustes.

O protótipo usa controles range nativos ligados a SVG e a um estado canônico único. É referência web, não imposição de tecnologia para a interface nativa, que deve cumprir o mesmo contrato observável e acessível.

### 8.4 Hierarquia, acessibilidade e feedback

- **DSP-UX-18 — Estados:** distinguir hover, foco, arraste, ativo, bypass, disabled, erro e incompatível, com motivo de indisponibilidade em texto acessível (sem depender só de cor ou tooltip de hover). O estado da cadeia não é deduzido da posição de uma chave individual.
- **DSP-UX-19 — Acessibilidade:** cada controle anuncia nome, valor, unidade e limites; foco visível, contraste, alvos do design system, ampliação de texto, navegação sem mouse, alternativa numérica ao arraste, movimento reduzido e alto contraste. Leitores de tela exigem validação na plataforma final; teste visual não é certificação WCAG.
- **DSP-UX-20 — Harmonização:** tokens da skin para superfícies, tipografia, ícones, estados e espaçamento; sem bordas decorativas por parâmetro nem cores competindo com a música; o espaço dos VUs estéreo originais não diminui para acomodar controles permanentes.
- **DSP-UX-21 — Feedback e desempenho:** alteração refletida na UI de imediato, atualizando só o necessário. A integração com o áudio segue smoothing, segurança de thread e ausência de estalos das seções técnicas; animação suave de knob não prova transição suave do sinal.

### 8.5 Limites do ajuste

Preservam-se: Classic 10 bandas, Gráficos 10/20/31/64 e Paramétrico até 32 filtros (knobs não substituem o editor de bandas); Vinyl/Vitrola e Tube Amplifier com configuração, presets, prioridade e bypass independentes; visões Básica/Avançada, A/B e desfazer (ausência no protótipo não autoriza removê-los da spec); rótulo `[DEMO — sem processamento de áudio neste modo]` onde não houver adaptação. A organização por famílias não altera a ordem da cadeia, não habilita reordenação livre e não muda marcos nativos (EQ64 segue em M5, salvo decisão formal). Decisões: [D-DSP-11 em diante](../product/05-decisions.md).

## 9. Fluxo modular (grafo) — referência do protótipo

Vista **Fluxo modular**, acessível por Áudio e efeitos, complementa o editor tradicional por catálogo, que continua disponível. Os eixos abaixo vêm do histórico rev. 15; onde este documento cita código, ele foi conferido no pin rev.38 — em particular a montagem dos módulos avançados como `AudioWorkletNode` em `mvp/graph-audio.js` e o `real=true` de `mvp/graph-model.js` ([10-vitrola §5](../audio/10-vitrola.md)). O que segue **relatado e não verificado** é a validação: suítes de UI, render offline e contraste dos fios. O contrato nativo e a separação protótipo × nativo estão em [DSP: grafo modular](../audio/03-dsp.md#grafo-modular-protótipo--nativo). Nativo hoje: nada implementado.

| Eixo | Conteúdo |
|---|---|
| Especificado | contratos abaixo; módulos do catálogo em [catálogo](../audio/09-dsp-catalog.md) |
| Implementado no protótipo (relatado) | canvas, portas tipadas, histórico, presets de cadeia, A/B de sessão, processamento Web Audio das famílias declaradas reais |
| Validado no protótipo (relatado) | suítes de UI, render offline, medição estéreo e contraste dos fios, em Chrome/Windows; não certificam acessibilidade nem o produto |
| Nativo | nada |

**Modelo e portas.** Cada instância de nó tem parâmetros próprios; duplicar não compartilha valores, e "Copiar ajuste do catálogo" é ação explícita (o catálogo não controla instâncias em silêncio). Portas: áudio estéreo e controle CV; só saída→entrada do mesmo tipo é válida. A saída CV do LFO liga à modulação do nó Ganho; não é MIDI nem automação genérica. Rejeitar ciclos, conexões duplicadas, portas ausentes, autoconexões e presets incompatíveis; Audio Input/Output são fixos. Fan-out e soma de rotas paralelas são permitidos e podem exigir headroom. Nós desconectados são editáveis mas não processam; cadeia habilitada sem caminho entrada→saída resulta em silêncio, sem conexão seca oculta.

**Capacidades (tabela de capacidades da referência, §12.3, resumida; detalhes por módulo: [09](../audio/09-dsp-catalog.md)).**

| Família/nós | Estado relatado na rev. 15 | Limite a registrar |
|---|---|---|
| Entrada/saída, ganho, pré-amp, mixer | áudio real | estéreo; mixer soma rotas e pode elevar picos |
| EQ gráfico 10/20/31/64 e Paramétrico até 32 filtros | áudio real dentro do grafo | biquads em série, ±15 dB, preamp −24/+12 dB; o editor avançado anterior mantém sua ligação original, sem marcação retroativa |
| Filtros, shelves/bell, tonais | áudio real | frequência acima de 0,45 × Fs bloqueia ativação |
| Compressor | áudio real | makeup automático do navegador mais Makeup adicional |
| Limiter amostral | real com limitações | não é brickwall nem true peak; não substitui o True Peak Limiter exigido |
| Largura, Mid/Side, pan/mono, fase/delay, crossfeed | áudio real | largura com graves em mono sem adaptador; opção não implementada bloqueia ativação |
| Echo, Reverb, saturação suave | áudio real | IR sintética, não sala medida nem FIR; troca de decay/damping gera nova IR, não novo grafo; saturação não é o Tube Amplifier |
| LFO→Ganho | controle real | ±0,25 de ganho linear |
| Spectrum, pico/RMS | análise real | FFT 1024, dBFS, antes do volume; não são LUFS/dBTP; sem valores sintéticos para a coleção demo |
| Vinyl Simulator e Tube Amplifier | **áudio real** — `class Vinyl` e `class Tube` em `mvp/advanced-dsp.js`, montados como `AudioWorkletNode` (`vynamp-advanced37`) por `mvp/graph-audio.js`; `real=true` em `mvp/graph-model.js` | processamento existe; algoritmos e validação acústica continuam pendentes ([10-vitrola §5](../audio/10-vitrola.md)) |
| True Peak (limiter), LUFS (loudness-meter), gate e expander | **áudio real** — os quatro saem com `real=true`; `gate`/`expander` montam `AudioWorkletNode` `vynamp-dynamics36`, o limiter tem lookahead e FIR 4×, o loudness-meter é o medidor de LUFS | limiter exige oversample 4× e ser o último processador audível (2×/8× não disponíveis); o LUFS é medido, não é conformidade BS.1770 certificada |
| Demais sem adaptador | nós de configuração, sem áudio | habilitado na rota ativa impede a ativação, com motivo exibido; bypass explícito permite a rota |

**Texto atrasado no MVP.** A frase "Vinyl/Tube não têm adaptador", exibida pela mensagem de `Carregar` preset em `mvp/graph-editor.js`, contradiz o próprio `graph-model.js` do pin, que marca os dois com `real=true`. É texto que ficou para trás — não é fonte para esta spec.

**Canvas e interação.** Nós arrastáveis, seleção múltipla, grupos/racks, conexão por arraste ou por seletores de origem/destino no inspetor (alternativa sem arraste), fios selecionáveis, menus contextuais, zoom/pan/ajustar/minimapa, biblioteca pesquisável, presets de cadeia (Clean, Warm Analog, Vinyl, Studio, Spatial) carregados em bypass, A/B de sessão, histórico de até 60 alterações e JSON versionado. Reabertura e importação iniciam em bypass, sem reprodução automática. Os presets de cadeia carregam Vinyl em bypass por precaução de nível, não por falta de processamento — o módulo tem áudio (ver tabela acima). Atalhos: Delete remove a seleção, Ctrl+D duplica, Ctrl+Z e Ctrl+Shift+Z desfazem/refazem; campos de edição preservam seus atalhos. O nome Studio não é certificação de mastering; A/B não é compensação automática de loudness. Mover nós, pan, zoom e acabamento não reconstroem o áudio.

**Fios e estados.** Bézier acompanham as portas; áudio em linha contínua, controle tracejado com portas quadradas; paletas ciano/violeta, âmbar/verde ou tokens do tema; brilho pode ir a zero e a animação desligar. A animação de fluxo só aparece com áudio local em reprodução, cadeia habilitada e ambos os extremos na rota compilada, sem bypass; a coleção DEMO silenciosa nunca fabrica processamento. Movimento reduzido prevalece.

**Limites, segurança e recuperação.** Limites do adaptador web: 64 nós, 128 conexões, preset de até 512 KiB, 512 biquads e 32 MiB de IR sintética; não são capacidade nativa comprovada, e latência/CPU exigem medição antes de metas. Presets validados atomicamente; leitura atrasada não sobrescreve edição recente; falha de ativação volta ao bypass com motivo. Ajustes incompatíveis não são executados parcialmente nem ganham badge de processamento real. Foco visível, ações sem arraste, Escape e retorno de foco permanecem requisitos; leitor de tela e WCAG completo seguem pendentes.

**Pendente (fora do protótipo):** processadores nativos e sua qualidade sonora; validação acústica de Vinyl/Tube (o áudio existe, a validação não); true peak **certificado** e LUFS conforme BS.1770 (o limiter e o medidor têm áudio, não certificação); demais recursos do catálogo sem adaptador; saída/driver nativos; cobertura integral do MVP v1.2; importação de IR no grafo; compensação de latência entre ramos; reordenação livre no nativo.
