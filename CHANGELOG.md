# Changelog / Histórico

Versão: 0.9.0 · Data: 2026-11-05 · Estado: especificação proposta, implementação não iniciada.

Each entry appears in English and Portuguese. The two texts say the same thing;
where they appear to differ, the Portuguese text of that entry is the original
and prevails.

Cada entrada aparece em inglês e português. Os dois textos dizem a mesma coisa;
onde parecerem divergir, o texto em português daquela entrada é o original e
prevalece.

## 0.1.0 — 2026-10-14

**EN** — Initial specification of an independent Rust product; own themes and
plugins; original classic visual and tokens; local, hi-res, DSP and streaming
features; future platform; architecture, contracts, quality and security;
traceable tasks and validated examples. Assumptions and phases made explicit. No
player implementation and no Winamp code was used.

**PT** — Especificação inicial de produto independente Rust; temas/plugins
próprios; visual clássico original e tokens; funcionalidades
locais/hi-res/DSP/streaming; plataforma futura;
arquitetura/contratos/qualidade/segurança; tarefas rastreáveis e exemplos
validados. Hipóteses e fases explicitadas. Não houve implementação do player ou
uso de código do Winamp.

## 0.2.0 — 2026-10-14

**EN** — Windows/Linux/macOS Intel+Silicon/Android mandatory from proofs to 1.0.
Linux/macOS promoted M5→M3 with IDs preserved; Android core in Rust plus native
shell; mobile/tablet/backends/QA/CI/distribution added. 16 new
tasks/requirements, epics E13/E14, release gates updated. No audio
implementation or testing in the product.

**PT** — Windows/Linux/macOS Intel+Silicon/Android obrigatórios desde provas até
1.0. Linux/macOS promovidos M5→M3 com IDs preservados; Android/core Rust+shell
nativo, mobile/tablet/backends/QA/CI/distribuição adicionados. 16 novas
tarefas/requisitos, épicos E13/E14, gates de release atualizados. Não houve
implementação/teste de áudio no produto.

## 0.3.0 — 2026-10-14

**EN** — Root project prepared for Codex/Claude with Superpowers mandatory;
worktree/claim/handoff/Gitflow/private GitHub; CI/release/hardware workflows,
tooling tests and installer/signing recipes. 146 tasks/15 epics. No plugin was
installed, no private repository was accessed or pushed, and no application or
installer was compiled in this preparation.

**PT** — Projeto raiz preparado para Codex/Claude com Superpowers obrigatório;
worktree/claim/handoff/Gitflow/GitHub privado; workflows CI/release/hardware,
tooling tests e recipes de installer/assinatura. 146 tarefas/15 épicos. Nenhum
plugin foi instalado, repositório privado acessado/pushado, aplicativo ou
instalador compilado nesta preparação.

## 0.4.0 — 2026-10-14

**EN** — Automation migrated from Python to Rust/`cargo xtask`. Real workspace,
Rust 1.99.0 and `Cargo.lock`; Git/claims/doctor/import, validators, CI and
hardware gates, and signing/installer/release recipes. Hooks and workflows
updated; CI distinguishes `xtask` from real player sources. The package keeps
release/application flags false. Local execution documented in
`docs/evidence/rust-tooling.json`; remote CI, signing, player and hardware were
not executed in this delivery.

**PT** — Automação migrada de Python para Rust/cargo xtask. Workspace, Rust
1.99.0 e Cargo.lock reais; Git/claims/doctor/importação, validadores,
CI/hardware gates e receitas de assinatura/instaladores/releases. Hooks e
workflows atualizados; CI distingue xtask de fontes reais do player. Package
mantém release/application flags false. Execução local documentada em
docs/evidence/rust-tooling.json; CI remoto/assinatura/player/hardware não
executados nesta entrega.

## 0.5.0 — 2026-10-14

**EN** — Name Vynamp and identity C — Pulso chosen by the user. Includes three
original reference PNGs with SHA and dimensions, a multiplatform guide, brand
tokens, ADR-008 and export gates. Renames namespaces/contracts/templates/previews
and the ZIP, updates the renamed GitHub target and preserves historical claims
and evidence. Does not export or fake vectors, native icons or executed
installers.

**PT** — Nome Vynamp e identidade C — Pulso escolhidos pelo usuário. Inclui
três PNGs originais de referência com SHA/dimensões, guia
multiplataforma/brand tokens, ADR-008 e gates de export. Renomeia
namespaces/contratos/templates/previews e ZIP, atualiza GitHub informado
renomeado e preserva claims/evidências históricas. Não exporta/finge
vetores/ícones nativos ou instaladores executados.

## 0.6.0 — 2026-10-14

**EN** — Completeness audit: associations/Open With on five targets,
batch/IPC/grants, owner per profile/portable/backup/deep links; catalogue of 68
bindings, USAC/immersive and Bluetooth separated; trim/DRC/metadata; true-peak
protection after master/mix/resampler; visualizer and proxy contract. Two new
epics/30 tasks, total 176 TASK/REQ across 17 epics; QA-022–031 and four new
schemas. Corrects distribution divergences while keeping eight mandatory
artefacts. Documentation and Rust validation only; no player implementation,
native export or remote publication.

**PT** — Auditoria de completude: associações/Open With em cinco alvos,
batch/IPC/grants, owner por perfil/portable/backup/deep links; catálogo de 68
bindings, USAC/imersivos e Bluetooth separado; trim/DRC/metadata; proteção
true-peak após master/mix/resampler; contrato de visualizers e proxy. Dois novos
épicos/30 tarefas, total 176 TASK/REQ em 17 épicos; QA-022–031 e quatro novos
schemas. Corrige divergências de distribuição mantendo oito artefatos
obrigatórios. Documentação e validação Rust; nenhuma implementação de player,
export nativo ou publicação remota.

## 0.7.0 — 2026-10-14

**EN** — Consolidates Windows corrections for verbatim/UNC paths, checks running
in a cache distinct from the executable, and cache aliases. Import avoids
colliding with an existing remote bootstrap branch. New `prepare-agents`
preserves the import branch/PR, creates Codex with a claim and Claude detached
with its own index; it does not depend on a `develop` branch that does not exist
yet. ZIP in a single `vynamp-media-player` root with package/repo, PowerShell
wrappers, automatic prompts and local caches/tmp/logs. Keeps 176 TASK/REQ and 68
planned catalogue entries, app/release/export flags false, and native evidence
still required.

**PT** — Consolida correções Windows de caminhos verbatim/UNC, checks em cache
distinto do executable e aliases de cache. Import evita colidir com branch de
bootstrap remota existente. Novo prepare-agents preserva import branch/PR, cria
Codex com claim e Claude detached/índice próprio; não depende de develop local
ausente. ZIP em raiz única vynamp-media-player com pacote/repo, wrappers
PowerShell, prompts automáticos e caches/tmp/logs locais. Mantém 176 TASK/REQ e
68 entradas planejadas, flags de app/release/exports false e evidências nativas
ainda exigidas.

## Unreleased — 2026-10-23 / Não lançado — 2026-10-23

**EN** — Workflow migrated to a single agent: Codex was removed from the tooling
(`doctor`/`install` for Claude only, no `prepare-agents`, no
`.worktrees/codex`), from the bootstrap, from the Superpowers lock and from the
live docs. Claude Code is the only author and reviewer (Opus orchestrates, Sonnet
implements and reviews, Haiku runs checks); merge and release stay with the
maintainer. ADR-007 received a partial-supersession note. Historical evidence and
plans that mention Codex were preserved.

**PT** — Fluxo migrado para agente único: o Codex foi removido do tooling
(`doctor`/`install` só para Claude, sem `prepare-agents` nem worktrees
`.worktrees/codex`), do bootstrap, do lock do Superpowers e dos docs vivos.
Claude Code é o único autor e revisor (Opus orquestra, Sonnet implementa e
revisa, Haiku executa checks); merge e release permanecem com o mantenedor.
ADR-007 recebeu nota de superação parcial. Evidências e planos históricos que
citam o Codex foram preservados.

## Unreleased — 2026-10-25 / Não lançado — 2026-10-25

**EN** — DSP documentation revised to 0.7.1 in the files touched: functional DSP
overview with a Mermaid diagram of the real chain ([03-dsp](docs/audio/03-dsp.md));
a mandatory equalizer subchapter — Graphic 10/20/31/64 plus Parametric up to 32
filters, with frequency grids, Nyquist, data model, UI, MVP versus native and
tests ([08](docs/audio/08-equalizers.md)); module catalogue
([09](docs/audio/09-dsp-catalog.md)); vitrola integration by reference, with the
design approved on 2026-10-23 ([10](docs/audio/10-vitrola.md)); DSP
panel/meters, test matrix, QA-032/033, DSP roadmap, risks, glossary and
decisions D-DSP-01..10 with divergences. Native PEQ32 stays M3; native Graphic
EQ-64 and PEQ64 stay M5; the HTML/SVG/JS MVP now covers the UI of the five
modes. Markdown only; no code, schema, backlog or pipeline changed.

**PT** — Documentação DSP revisada para 0.7.1 nos arquivos tocados: visão
funcional do DSP com diagrama Mermaid da cadeia real
([03-dsp](docs/audio/03-dsp.md)); subcapítulo obrigatório de equalizadores
Gráfico 10/20/31/64 + Paramétrico até 32 filtros, com grades de frequência,
Nyquist, modelo de dados, UI, MVP × nativo e testes
([08](docs/audio/08-equalizers.md)); catálogo de módulos
([09](docs/audio/09-dsp-catalog.md)); integração por referência do vitrola, com
design aprovado em 2026-10-23 ([10](docs/audio/10-vitrola.md)); painel
DSP/medidores, matriz de testes, QA-032/033, roadmap DSP, riscos, glossário e
decisões D-DSP-01..10 com divergências. PEQ32 nativo segue M3; EQ-64 gráfico e
PEQ64 nativos seguem M5; o MVP HTML/SVG/JS cobre agora a UI dos cinco modos. Só
Markdown; nenhum código, schema, backlog ou pipeline alterado.

## Unreleased — 2026-10-28 (DSP v1.4) / Não lançado — 2026-10-28 (DSP v1.4)

**EN** — DSP v1.4 documentation: Audio and effects panel with DSP-UX-01..21
access and controls and VU-INT-01..06 integration in
[design/05-dsp-panel.md](docs/design/05-dsp-panel.md) (§6.1, §8, §9);
DSP-UX-AC-01..12 criteria and graph/VU-INT tests in
[quality/05-dsp-test-matrix.md](docs/quality/05-dsp-test-matrix.md) (§2); modular
graph versus native in [audio/03-dsp.md](docs/audio/03-dsp.md); revision 15
status in [audio/08-equalizers.md](docs/audio/08-equalizers.md) (§14); decisions
D-DSP-11..18 in [product/05-decisions.md](docs/product/05-decisions.md). The
HTML/SVG/JS prototype is external: reported evidence of the external reference
rev. 14/15, not verified in this repository; nothing native implemented.
Markdown only; no code, schema, backlog or pipeline changed.

**PT** — Documentação DSP v1.4: painel Áudio e efeitos com acesso e controles
DSP-UX-01..21 e integração dos VUs VU-INT-01..06 em
[design/05-dsp-panel.md](docs/design/05-dsp-panel.md) (§6.1, §8, §9);
critérios DSP-UX-AC-01..12 e testes do grafo/VU-INT em
[quality/05-dsp-test-matrix.md](docs/quality/05-dsp-test-matrix.md) (§2); grafo
modular × nativo em [audio/03-dsp.md](docs/audio/03-dsp.md); status da revisão
15 em [audio/08-equalizers.md](docs/audio/08-equalizers.md) (§14); decisões
D-DSP-11..18 em [product/05-decisions.md](docs/product/05-decisions.md). O
protótipo HTML/SVG/JS é externo: evidência relatada da referência externa
rev. 14/15, não verificada neste repositório; nada nativo implementado. Só
Markdown; nenhum código, schema, backlog ou pipeline alterado.

## Unreleased — 2026-10-28 (lyrics) / Não lançado — 2026-10-28 (letras)

**EN** — Lyrics: a summarised component below the VU and its sources in
[library/02-tags-lyrics-tools.md](docs/library/02-tags-lyrics-tools.md), with IDs
LYR-01..15 and LYR-AC-01..12, layout in
[design/02-layouts.md](docs/design/02-layouts.md), decisions D-LYR-01..09 in
[product/05-decisions.md](docs/product/05-decisions.md) and the task criteria in
[TASK-E03-09](tasks/items/TASK-E03-09.md). The prototype reference is external:
reported evidence of the external reference rev. 14, not verified in this
repository; nothing native implemented. Markdown only; no code, schema, backlog
or pipeline changed (TASK-E03-09 was updated in text only: deliverable,
dependencies and criteria).

**PT** — Letras: componente resumido sob o VU e fontes em
[library/02-tags-lyrics-tools.md](docs/library/02-tags-lyrics-tools.md), com IDs
LYR-01..15 e LYR-AC-01..12, layout em
[design/02-layouts.md](docs/design/02-layouts.md), decisões D-LYR-01..09 em
[product/05-decisions.md](docs/product/05-decisions.md) e critérios da tarefa em
[TASK-E03-09](tasks/items/TASK-E03-09.md). A referência do protótipo é externa:
evidência relatada da referência externa rev. 14, não verificada neste
repositório; nada nativo implementado. Só Markdown; nenhum código, schema,
backlog ou pipeline alterado (TASK-E03-09 foi atualizada apenas em texto:
entrega, dependências e critérios).

## Unreleased — 2026-11-03 (MVP reference) / Não lançado — 2026-11-03 (referência do MVP)

**EN** — Web MVP rev.37 frozen by hash as the specification of the native UI
(egui) in [design/06-mvp-reference.md](docs/design/06-mvp-reference.md); extracted
package (12 themes, 91 icons, 16 meters, rendered SVGs) by the separate
repository `vynamp-mvp-reference`; QA-034 and TASK-E02-11..16 (visual baseline,
motion, behaviour, gaps and the egui_kittest harness). resvg versus Chrome
verification in [evidence](docs/evidence/PR/task-e02-11/README.md). Nothing
native implemented.

The MVP package was withdrawn from this repository: no MVP file stays here; it
and the extracted package live only in `vynamp-mvp-reference`.

**PT** — MVP web rev.37 congelado por hash como especificação da UI nativa (egui)
em [design/06-mvp-reference.md](docs/design/06-mvp-reference.md); pacote extraído
(12 temas, 91 ícones, 16 medidores, SVGs renderizados) pelo repositório próprio
`vynamp-mvp-reference`; QA-034 e TASK-E02-11..16 (fases de gabarito visual,
movimento, comportamento, lacunas e harness egui_kittest). Verificação resvg ×
Chrome em [evidência](docs/evidence/PR/task-e02-11/README.md). Nada nativo
implementado.

Pacote do MVP retirado deste repositório: nenhum arquivo do MVP fica aqui; ele e
o pacote extraído vivem só em `vynamp-mvp-reference`.

## Unreleased — 2026-11-03 (MVP rev.38 references) / Não lançado — 2026-11-03 (referências ao MVP rev.38)

**EN** — Specs now cite the web MVP rev.38 by URL, fixed SHA and path:
`vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` (`mvp/...`,
`out/...`); no MVP file is copied into this repository.
[MVP reference](docs/design/06-mvp-reference.md) rewritten (v0.8.0): the 16
meters are selectable, real `out/` paths, content hashes and git tree labelled,
no `SOURCE.json`; at pin time, `tools/package.cjs` and the reference
repository's README still copied `out/` into `assets/reference/mvp/`, which was
removed in `vynamp-mvp-reference@741316f` (`tools/pin.cjs`). TASK-E02-01..10,
E02-12..16 and the UI tasks pending outside E02 (E06-01, 03, 06, 07, 08, 10,
E10-06, E13-05, E13-09, E17-09 and E17-10) point at the pin; TASK-E02-11 remains
as history of rev.37. The TASK-E02-16 harness runs locally only; the CI Gate
marks it as not executed. iOS is no longer a target in TASK-E02-15 (outside
1.0). Citations of `interactive/` (Codex prototype rev.14/15) marked as
historical.

**PT** — As specs passam a citar o MVP web rev.38 por URL, SHA fixo e caminho:
`vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` (`mvp/...`,
`out/...`); nenhum arquivo do MVP é copiado para este repositório.
[Referência do MVP](docs/design/06-mvp-reference.md) reescrita (v0.8.0): os 16
medidores são selecionáveis, caminhos reais de `out/`, hashes de conteúdo e git
tree rotulados, sem `SOURCE.json`; no pin, `tools/package.cjs` e o README do
repositório de referência ainda copiavam `out/` para `assets/reference/mvp/`, o
que foi removido em `vynamp-mvp-reference@741316f` (`tools/pin.cjs`).
TASK-E02-01..10, E02-12..16 e as TASKs de UI pendentes fora de E02 (E06-01, 03,
06, 07, 08, 10, E10-06, E13-05, E13-09, E17-09 e E17-10) apontam para o pin;
TASK-E02-11 permanece como histórico da rev.37. O harness de TASK-E02-16 roda só
localmente; o CI Gate o marca como não executado. iOS deixa de constar como alvo
em TASK-E02-15 (fora da 1.0). Citações de `interactive/` (protótipo Codex
rev.14/15) marcadas como históricas.

## Unreleased — 2026-11-03 (own audio stack) / Não lançado — 2026-11-03 (pilha de áudio própria)

**EN** — [ADR-011](docs/architecture/adr/ADR-011-own-audio-stack.md): decode,
output, effects and tags now come from the project's own libraries
`vynamp-decode`, `vynamp-output` and `vitrola` (fixed SHAs, real versus planned
API state and gap table); the resampler will be a new own library (name to be
defined, D-DSP-08 decided) and tag writing will extend `vynamp-decode-meta`.
ADR-002 marked partially superseded; stack, formats, outputs, backends, DSP
catalogue, references, licensing specs and TASK-E04-02 rewritten without the
third-party library names. Documentation only.

**PT** — [ADR-011](docs/architecture/adr/ADR-011-own-audio-stack.md): decode,
saída, efeitos e tags passam a vir das bibliotecas próprias vynamp-decode,
vynamp-output e vitrola (SHAs fixos, estado API real × planejado e tabela de
lacunas); o resampler será uma nova lib própria (nome a definir, D-DSP-08
decidido) e a escrita de tags estenderá `vynamp-decode-meta`. ADR-002 marcada
como parcialmente substituída; specs de stack, formatos, saídas, backends,
catálogo DSP, referências, licenças e TASK-E04-02 reescritas sem os nomes de
bibliotecas de terceiros. Somente documentação.

## Unreleased — 2026-11-05 (shell integration and icon packs) / Não lançado — 2026-11-05 (integração com o shell e pacotes de ícones)

**EN** — [ADR-012](docs/architecture/adr/ADR-012-shell-integration.md) formalises
native shell integration: the modern Windows 11 menu through a signed sparse MSIX
package and `IExplorerCommand`, the classic HKCU menu for Windows 10/"Show more
options", Desktop Actions and file-manager extensions (Dolphin KIO ServiceMenu,
Nautilus, Thunar/Nemo) on Linux, Quick Actions (`NSServices`) and
`CFBundleDocumentTypes` on macOS, and Intent Filters
(`ACTION_VIEW`/`ACTION_SEND`/`ACTION_SEND_MULTIPLE`) with SAF on Android. A
"Vynamp" submenu with 7 canonical verbs (Play, Open in new playlist, Add to
playlist, Add to queue, Edit tags, Convert, Restore with Vynamp Restore) and the
corresponding CLI contract (`--play`, `--new-playlist`, `--add-playlist`,
`--enqueue`, `--edit-tags`, `vynamp-converter`, `vynamp-restore`), with atomic
batching through `IShellItemArray`. Vynamp Restore and Vynamp Converter become
separate desktop executables. A file-icon pack system with 3 families
(Document, Disc, Signal) for 17 formats, multi-resolution `.ico` (16–256 px) on
Windows, `hicolor` hierarchy on Linux and in-app application on macOS/Android;
catalogue in [assets/icons](assets/icons/README.md) with manifest and interactive
preview. [03-system-integration](docs/platforms/03-system-integration.md)
rewritten to 0.8.0; TASK-E16-03/04/05/08 and the backlog updated; glossary and
index extended. Documentation and assets only; no integration implemented or
tested.

**PT** — [ADR-012](docs/architecture/adr/ADR-012-shell-integration.md) formaliza
a integração nativa com o shell: menu moderno do Windows 11 via pacote esparso
MSIX assinado e `IExplorerCommand`, menu clássico HKCU para Windows 10/"Mostrar
mais opções", Desktop Actions e extensões de gerenciador (Dolphin KIO
ServiceMenu, Nautilus, Thunar/Nemo) no Linux, Quick Actions (`NSServices`) e
`CFBundleDocumentTypes` no macOS, e Intent Filters
(`ACTION_VIEW`/`ACTION_SEND`/`ACTION_SEND_MULTIPLE`) com SAF no Android. Submenu
"Vynamp" com 7 verbos canônicos (Reproduzir, Abrir em nova lista, Adicionar à
lista, Adicionar à fila, Editar etiquetas, Converter, Restaurar com Vynamp
Restore) e contrato CLI correspondente (`--play`, `--new-playlist`,
`--add-playlist`, `--enqueue`, `--edit-tags`, `vynamp-converter`,
`vynamp-restore`), com batching atômico via `IShellItemArray`. Vynamp Restore e
Vynamp Converter passam a ser executáveis desktop separados. Sistema de pacotes
de ícones de arquivo com 3 famílias (Documento, Disco, Sinal) para 17 formatos,
`.ico` multi-resolução (16–256px) no Windows, hierarquia `hicolor` no Linux e
aplicação in-app no macOS/Android; catálogo em
[assets/icons](assets/icons/README.md) com manifesto e preview interativo.
[03-system-integration](docs/platforms/03-system-integration.md) reescrito para
0.8.0; TASK-E16-03/04/05/08 e o backlog atualizados; glossário e índice
ampliados. Somente documentação e assets; nenhuma integração implementada ou
testada.

## Unreleased — 2026-11-05 (file tools: Restore and Converter) / Não lançado — 2026-11-05 (ferramentas de arquivo: Restore e Converter)

**EN** — [ADR-013](docs/architecture/adr/ADR-013-vynamp-restore.md) fixes Vynamp
Restore as a dedicated desktop tool: a separate process (the player's real-time
engine is untouched), the restoration engine as a reusable internal library, and
an explicit scope of what restoration is not (AI voice separation, master
reconstruction, hi-res upscaling). The spec
[tools/01-vynamp-restore.md](docs/tools/01-vynamp-restore.md) closes the
`vynamp-restore` contract: a normative chain of 11 DSP stages in fixed order
(AR de-crackle, linear-phase LPF, MP3 artefact removal, BWE, de-click, DC offset,
de-rumble, hum notch, gate, anti-saturation, region blend), 12 defect detectors
with severity and numerical evidence, A/B comparison, batch, metadata,
deterministic export with atomic write and post-write validation, and a
`.vynamp-restore` project. The spec
[tools/02-vynamp-converter.md](docs/tools/02-vynamp-converter.md) closes the
`vynamp-converter` scope for PR5: format matrix, mandatory lossless/lossy
labelling, persisted job queue, and the own library `vynamp-encode` as the single
source of encoders. MVP prototype divergences are recorded: the de-crackle
actually running in the MVP is the AR model (not the inert mid/side module), the
Wiener denoise is unreachable and out of scope for 1.0, and `kind: overlay` was a
placeholder. Epic [E18](tasks/epics/E18.md) created with TASK-E18-01 (Restore)
and TASK-E18-02 (Converter), REQ-E18-01/02 in the normative matrix (184
requirements/tasks across 18 epics) and backlog entries. Documentation only; not
one line of restoration, conversion or encoder code was implemented or executed.

**PT** — [ADR-013](docs/architecture/adr/ADR-013-vynamp-restore.md) fixa o Vynamp
Restore como ferramenta desktop dedicada: processo separado (o motor de tempo
real do player fica intocado), motor de restauração como biblioteca interna
reaproveitável e escopo explícito do que não é restauração (separação de voz por
IA, reconstrução de master, upscaling hi-res). A spec
[tools/01-vynamp-restore.md](docs/tools/01-vynamp-restore.md) fecha o contrato do
`vynamp-restore`: cadeia normativa de 11 estágios DSP em ordem fixa (de-crackle
AR, LPF linear-phase, remoção de artefatos MP3, BWE, de-click, offset DC,
de-rumble, notch de hum, gate, anti-saturação, blend de regiões), 12 detectores
de defeito com severidade e evidência numérica, comparação A/B, lote, metadados,
exportação determinística com escrita atômica e validação pós-escrita, e projeto
`.vynamp-restore`. A spec
[tools/02-vynamp-converter.md](docs/tools/02-vynamp-converter.md) fecha o escopo
do `vynamp-converter` para o PR5: matriz de formatos, rótulagem lossless/lossy
obrigatória, fila de jobs persistida e a lib própria `vynamp-encode` como única
fonte de codificadores. As divergências do protótipo MVP ficam registradas: o
de-crackle realmente executado no MVP é o modelo AR (não o módulo mid/side
inerte), o denoise Wiener é inalcançável e está fora de escopo da 1.0, e
`kind: overlay` era placeholder. Épico [E18](tasks/epics/E18.md) criado com
TASK-E18-01 (Restore) e TASK-E18-02 (Converter), REQ-E18-01/02 na matriz
normativa (184 requisitos/tarefas em 18 épicos) e entradas no backlog. Somente
documentação; nenhuma linha de código de restauração, conversão ou encoder foi
implementada ou executada.

## Unreleased — 2026-11-05 (GitHub Actions, public repositories and governance) / Não lançado — 2026-11-05 (GitHub Actions, repositórios públicos e governança)

**EN** — CI moved back to GitHub Actions ([ADR-014](docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md),
which supersedes [ADR-010](docs/architecture/adr/ADR-010-woodpecker-only-ci.md)):
`ci.yml`, `release.yml` and `quality.yml` under `.github/workflows/`, actions
pinned by full commit SHA, the `gate` job published as `CI Gate`, and the
Woodpecker server, its Cloudflare tunnel and `infra/ci/` removed from the
repository. `xtask` was inverted to require the workflows instead of forbidding
them, and `release` validates its tag through `GITHUB_REF`/`GITHUB_REF_NAME`.
Verified in practice: the workflow ran on pull request #30 and again on push to
`develop`, green in `docs`, the four `tooling` targets (`windows-2022`,
`ubuntu-22.04`, `macos-15-intel`, `macos-15`) and `CI Gate`, with `core` and
`android` skipped because `cargo xtask ci detect` finds no sources for them. The
repository is still **private** and its branch protection is still unavailable,
so `CI Gate` is a reviewed discipline rather than a GitHub-enforced block.

Repository hygiene and governance (TASK-E19-02): the project finally has a
licence — `MIT OR Apache-2.0`, with `LICENSE`, `LICENSE-MIT` and `LICENSE-APACHE`
— recorded as D-LIC-01 because the previous state was an explicit decision *not*
to have one; `LEGAL.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, issue forms and a
bilingual `README.md`, `SECURITY.md`, `CONTRIBUTING.md` and this changelog.
`SECURITY.md` now points at GitHub Security Advisories (D-GOV-01) instead of
declaring that no channel existed. Documentation and repository files only; no
player code.

**PT** — CI voltou para GitHub Actions
([ADR-014](docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md),
que substitui a [ADR-010](docs/architecture/adr/ADR-010-woodpecker-only-ci.md)):
`ci.yml`, `release.yml` e `quality.yml` em `.github/workflows/`, actions fixadas
por SHA completo de commit, job `gate` publicado como `CI Gate`, e o servidor
Woodpecker, seu túnel Cloudflare e `infra/ci/` removidos do repositório. O
`xtask` foi invertido para exigir os workflows em vez de proibi-los, e o
`release` valida a tag por `GITHUB_REF`/`GITHUB_REF_NAME`. Verificado na prática:
o workflow rodou no pull request #30 e de novo no push para `develop`, verde em
`docs`, nos quatro alvos de `tooling` (`windows-2022`, `ubuntu-22.04`,
`macos-15-intel`, `macos-15`) e no `CI Gate`, com `core` e `android` pulados
porque `cargo xtask ci detect` não encontra fontes para eles. O repositório
continua **privado** e a branch protection segue indisponível, então o `CI Gate`
é disciplina revisada, não bloqueio imposto pelo GitHub.

Higiene e governança do repositório (TASK-E19-02): o projeto passa a ter
licença — `MIT OR Apache-2.0`, com `LICENSE`, `LICENSE-MIT` e `LICENSE-APACHE` —
registrada como D-LIC-01 porque o estado anterior era uma decisão explícita de
*não* ter licença; `LEGAL.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, formulários
de issue e `README.md`, `SECURITY.md`, `CONTRIBUTING.md` e este changelog
bilíngues. O `SECURITY.md` passa a apontar para GitHub Security Advisories
(D-GOV-01) em vez de declarar que não havia canal. Somente documentação e
arquivos de repositório; nenhum código de player.

## Não lançado — 2026-11-05 (o MVP web é a fonte única da UI nativa)

Decisão do mantenedor: os dois arquivos de skin da raiz do projeto (`SKIN-REQUIREMENTS-CODEX.md` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md`) **não são fonte de requisito** e saem da cadeia documental do produto; a fonte única da UI nativa é o MVP web em [design/06-mvp-reference](docs/design/06-mvp-reference.md), pinado por SHA. Registrado em [05-decisions](docs/product/05-decisions.md) como D-MVP-01..04, com duas tabelas: os comportamentos que os SKIN atribuíam ao produto e o MVP **não** tem (0 VU = −18 dBFS ajustável, slot fixo de 28 DIP, atalho da Vitrola no inspector da cadeia, bypass de seção por "intensidade 0", presets Subtle/Warm/Rustic, passo de 2 dB no editor clássico), e as divergências encerradas por esta decisão. D-MVP-02 fixa que só herda comportamento que o código do MVP comprova; D-MVP-03, que nenhuma TASK, REQ ou marco nasce dos SKIN; D-MVP-04, que em divergência de **número** vale o MVP — caso atual, os 12 medidores do contrato VU-INT (número da rev.15) contra os **16, todos selecionáveis**, do pin rev.38. [08-equalizers](docs/audio/08-equalizers.md), [10-vitrola](docs/audio/10-vitrola.md) e [05-dsp-panel](docs/design/05-dsp-panel.md) passam a citar o código do pin no lugar dos SKIN, apenas onde ele de fato implementa; oito afirmações foram conferidas contra `vynamp-mvp-reference@0d9860b`. A matriz VU-INT-06 não foi reescrita — atualizá-la é da TASK-E02-07. Somente documentação: 191 requisitos, 191 tarefas, 19 épicos, nenhuma alteração de código, workflow ou backlog.

## Não lançado — 2026-11-06 (limpeza de planos de sessão superados)

Removidos quatro arquivos que eram **rascunho de execução**, não documento vivo: `docs/superpowers/plans/2026-10-15-windows-restart.md` (checklist de reinício do Windows, concluído em 2026-10-15) e o conjunto da E15-04 Woodpecker — `plans/2026-10-18-e15-04-woodpecker.md`, `plans/2026-10-19-e15-04-woodpecker-only-ci.md` e `specs/2026-10-18-e15-04-woodpecker-cloudflare-design.md` —, todos substituídos pela [ADR-014](docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md), que devolveu o CI ao GitHub Actions. A decisão e o histórico do Woodpecker continuam registrados na [ADR-010](docs/architecture/adr/ADR-010-woodpecker-only-ci.md), marcada como substituída, e na evidência [E15-04](docs/evidence/PR/task-e15-04/README.md). Ficaram os planos que uma TASK viva cita (E11-01, E15-01, E15-02, E15-03) e todos os documentos normativos. Nenhuma alteração de código, workflow ou backlog; 351 arquivos Markdown, 191 requisitos, 191 tarefas, 19 épicos.
