# Alinhamento das issues aos objetivos do projeto

| Campo | Valor |
|---|---|
| Data | 2026-07-25 |
| Alvo | As 35 issues abertas em `fossio/raptor` (`#1`–`#35`) |
| Fonte | `discovery-ofx-rust-wasm.md`, `roadmap-github.md`, `evolucao-ofx.md`, `auditoria/plano-issues-github.md`, `auditoria/alteracoes-adr.md`, `auditoria/resolucao-auditoria.md` |
| Escopo | 21 issues alteradas (uma com escopo ampliado) · 17 novas no MVP · 8 no backlog de horizonte · 1 label redescrita · 1 milestone novo |
| Estado | **Proposta.** Nada foi aplicado ao tracker — este documento é o que se revisa antes de mexer nas issues |

O tracker está estruturalmente correto: a taxonomia de labels existe, os milestones M0–M7
existem, e as 35 issues mapeiam 1:1 com o `roadmap-github.md` §3, na ordem. O problema não é de
estrutura — é que as issues congelaram uma versão do roadmap que três documentos posteriores já
corrigiram, e deixam de fora partes centrais do objetivo declarado do projeto.

---

## 1. Premissas

Três decisões estão assumidas aqui. Cada uma é reversível, e o que muda se você discordar está
dito explicitamente.

**1.1 — A taxonomia canônica é a do `roadmap-github.md`.** `type: epic|story|task|spike|bug`,
`area:` incluindo `web` e `repo`, milestones M0–M7 por épico. É a que está implementada: as
labels existem no repositório, os milestones existem, as 35 issues já as usam.
`auditoria/hierarquia-artefatos-github.md` propõe outra taxonomia
(`type: feature|chore|docs|refactor|decision`, `area:` só as quatro crates, milestones = Fases
0–6 do Discovery) e fica como registro histórico superado — o próprio `roadmap-github.md`
declara no cabeçalho que revisa a ADR-12. *Se você preferir reconciliar as duas*, o custo é
relabelar as 35 issues e recriar 8 milestones; os valores que faltariam incorporar são
`type:decision` (para os achados `#48`/`#58`/`#66`/`#76`, que são decisão e não trabalho) e
`type:docs`/`type:refactor`.

**1.2 — O parser XML 2.x entra no Milestone 2.** Hoje ele não tem dono em issue nenhuma (§3.B).
Alternativa avaliada: milestone próprio — recusada, porque separá-lo do SGML cria um milestone
que entrega meio parser, e os dois compartilham corpus, fuzzing e canal de reporte.

*Atenção, porque isto contraria um documento vivo:* o §10 do Discovery dava a **Fase 0** como dona
única do parser XML 2.x — "o caminho fácil antes do difícil", achado `#73`. Não era contradição de
fato (a R5/R7 trocou o roadmap horizontal por fatia vertical, e o M1 constrói o parser da família
que o spike `#4` confirmar), mas os dois documentos ficavam escritos de formas incompatíveis.
**Corrigido no Bloco 5** — o §10 agora marca a si mesmo como narrativa conceitual superada pela
R5/R7, com as afirmações fáticas realinhadas ao tracker real.

**1.3 — A issue `#16` perde o `needs:design`.** Ela está bloqueada esperando uma decisão
(versionamento de schema, RA-11/25) que a própria RA adiou "até existir um consumidor real que
persista dados" — e `#16` **é** esse consumidor. Manter o bloqueio é um ciclo: a issue espera uma
decisão que só ela viabiliza.

*Tensão resolvida em 2026-07-27:* a RA-11/25 tinha amarrado a condição de reabertura ao **Épico 5**
("quando a demo de referência de fato persistir algo entre sessões"), mas a `#16` está no M3 — o
gatilho documentado na tabela de riscos do §9 do Discovery estava dois milestones atrasado em
relação a onde a decisão realmente ia acontecer. **Decisão do autor: o gatilho sobe para o M3**,
não a `#16` desce para o M5 — descer `#16` desfaria a priorização da R2/R3/R5+R7, que promoveu
persistência para o Épico 3 como o coração do valor recorrente do produto (resposta ao A#2).
§9 do Discovery corrigido para apontar para `#16`/M3.

---

## 2. Estado verificado

Levantado contra o repositório e a API do GitHub, não de memória.

| Verificação | Resultado |
|---|---|
| Issues `#1`–`#35` ↔ roadmap `#0.1`–`#7.5` | 1:1, na ordem de criação, sem desvio |
| Issues fechadas | Só `#1` (workspace multi-crate), pelo PR #36 |
| `auditoria/plano-issues-github.md` aplicado? | **Não.** `#16` ainda com `needs:design` e "versionada" no título; `#22` ainda só com `area:analytics`; `#17` com o título antigo; nenhuma issue de parser XML 2.x |
| Issues que constroem o domínio | **Zero.** `crates/domain/src/lib.rs` são 18 linhas de placeholder que dizem, no próprio comentário, que "os módulos reais entram numa issue dedicada" — essa issue não existe |
| Código da Fase −1 no repositório | **Ausente.** `money`, `metric`, `cashflow`, `dedupe`, `consolidation`, `predictive` (995 linhas, 24 testes + 5 doctests, clippy limpo) existem fora do repositório |
| Achados citados pelas issues | Resolviam para documentos não versionados até este commit — `docs/` tinha 3 arquivos |
| `docs/handoff-ux-design-frontend.md` | Estava numa revisão anterior à que gerou `#22`/`#23`/`#26`: faltava o parágrafo de Web Worker/assincronia, `scheduled_payments`, drill-down e 3 linhas do glossário |
| `#3` (fixar toolchain) | Parcialmente entregue sem registro: `rust-toolchain.toml` existe (1.97.1, target `wasm32-unknown-unknown`, componentes `rustfmt`/`clippy`); `wasm-opt`/`wasm-bindgen` **não** estão pinados no CI |
| Horizontes H0–H3 e anti-roadmap | Sem milestone, sem issue, sem registro no tracker |
| Labels `type:bug` e `needs:decision` | Existem no repositório, nenhuma issue as usa |

---

## 3. Diagnóstico

**A. Correções escritas e nunca aplicadas.** `auditoria/plano-issues-github.md` documenta 15
alterações com o texto pronto para substituir. Nenhuma chegou ao tracker. Nove issues seguem sem
o "corpo em formato user story + critério de aceite" que o §3 do roadmap promete para todas.

**B. O objetivo central não tem dono — e não é só o domínio.** A tradução de fases para épicos
perdeu duas classes inteiras de entregável nomeadas no §10 do Discovery: o **modelo de domínio**
(abaixo) e os **artefatos de ADR-10/ADR-11** (`CLAUDE.md`, o hook que barra commit de dado
bancário real, `.claude/rules/`, `CHANGELOG`, templates, Dependabot, `NOTICE` — §6). Nenhum tem
issue. O projeto é uma biblioteca de parsing + analytics sobre um modelo de domínio tipado; o
modelo não tem issue. M0 está "em andamento" com três issues sobre
*infraestrutura* (workspace, medir binário, fixar toolchain) e nenhuma sobre *o que a biblioteca
é*. Duas consequências concretas, não teóricas:

- `#2` (medir o `.wasm` do domain isolado) mediria hoje um placeholder de 18 linhas. O número
  registrado como gate do A#9 não teria significado.
- `#14` (consolidação multi-fonte) tem critério de aceite sobre `dedupe`/`consolidation` —
  descritos em `auditoria/alteracoes-adr.md` como "código existente e testado" — que não estão
  no repositório. A issue pressupõe um ponto de partida que não existe.

**C. Rastreabilidade quebrada.** Toda issue cita achados numerados (`achado #76`, `A#22`,
`RA-11/25`) de documentos que não estavam versionados. A cadeia Discovery → auditoria → achado →
issue existia nos arquivos do autor e sumia no repositório. *Corrigido neste commit para os
achados que têm documento* (§8, Bloco 0) — **com uma exceção que o commit não pôde fechar:**
`achado #26`–`#32` não resolve para documento nenhum, nem nos arquivos originais desta rodada.
`meta-auditoria-III.md` cita "meta-auditoria II" como quem fechou essa faixa; esse documento nunca
existiu entre os originais recebidos. Nenhuma issue cita achado nessa faixa hoje, então o buraco é
inerte — mas fica registrado (`docs/README.md`) para não virar referência quebrada silenciosa se
alguém citar `#26`–`#32` no futuro.

**D. Deriva entre issue e realidade.** `#4` teve os marcadores de header XML engolidos na criação
(sobraram dois pares de crases vazias no corpo, onde deviam estar a declaração XML e
`OFXHEADER="200"`). `#4` e `#5` se referenciam em prosa ("bloqueia a issue do parser mínimo",
"confirmado no spike de header") em vez de usar `#N`, que o GitHub linkaria. `#3` está
parcialmente entregue e parece intocada.

**E. Horizonte ausente.** `evolucao-ofx.md` define onde o produto ganha valor recorrente
(categorização, recorrência, forecast, orçamento — a resposta ao A#2) e o que deliberadamente
nunca entra (sync bancário, ML embarcado, telemetria — decidido como binário, "nem anônima").
Nada disso está no tracker, então cada pressão previsível será rediscutida do zero.

---

## 4. Mapa issue → ação

| # | Issue | Roadmap | M | Ação |
|---|---|---|---|---|
| 1 | Workspace multi-crate | `#0.1` | 0 | — (fechada) |
| 2 | Medir `.wasm` do domain | `#0.2` | 0 | **corpo** — nota de precedência: só mede depois de N1/N2 |
| 3 | Fixar toolchain | `#0.3` | 0 | **corpo** — vira checklist do que resta |
| 4 | Spike dialeto Nubank | `#1.1` | 1 | **corpo** — restaurar marcadores de header; linkar `#5` |
| 5 | Parser mínimo Nubank | `#1.2` | 1 | **título + corpo** — referenciar `#4` |
| 6 | Ambiguidade de data Nubank | `#1.3` | 1 | — |
| 7 | WASM mínimo | `#1.4` | 1 | **corpo** — declarar dependência cruzada com `#17` |
| 8 | Onboarding Nubank | `#1.5` | 1 | — (ver N12 para o guia por instituição) |
| 9 | Parser SGML 1.x | `#2.1` | 2 | **corpo** — nota de decomposição de `effort:xl` |
| 10 | Corpus sintético | `#2.2` | 2 | **corpo** — restaurar as referências `#77`/`#65`/`#18` perdidas |
| 11 | Fuzzing | `#2.3` | 2 | — |
| 12 | Canal de reporte de dialeto | `#2.4` | 2 | — |
| 13 | Robustez Nubank PJ | `#2.5` | 2 | — |
| 14 | Consolidação multi-fonte | `#3.1` | 3 | **corpo** — nota de precedência: pressupõe N1 |
| 15 | `period_series` | `#3.2` | 3 | **corpo** — acrescentar critério de aceite |
| 16 | Persistência local | `#3.3` | 3 | **título + corpo + labels** — remove `needs:design`, `task`→`story` |
| 17 | `ledger` | `#4.1` | 4 | **título + corpo** — reconhecer escopo parcialmente entregue em `#7` |
| 18 | `anomaly::z_score` | `#4.2` | 4 | **corpo** — user story + critério |
| 19 | `credit::utilization` | `#4.3` | 4 | — |
| 20 | `integrity::health_report` | `#4.4` | 4 | **corpo** — user story + critério |
| 21 | ~~`ledger::open_installments`~~ | `#4.5` | 4 | **corpo** — user story + critério; **fechada em 2026-07-27** (achado #76 revertido) |
| 22 | Drill-down | `#4.6` | 4 | **labels + corpo** — `+area:domain`; coordenar com `#14` |
| 23 | Contas agendadas (Bill Pay) | `#4.7` | 4 | — |
| 24 | Spike Taxes Message Set | `#4.8` | 4 | — |
| 25 | Web Worker | `#5.1` | 5 | — |
| 26 | API Grupo 1–3 assíncrona | `#5.2` | 5 | **corpo** — user story + critério (inclui erro estruturado, achado #83) |
| 27 | Diagramas para `await` | `#5.3` | 5 | — |
| 28 | Toggle de modo avançado | `#6.1` | 6 | — |
| 29 | `returns`/`risk` | `#6.2` | 6 | **corpo** — user story + critério + nota de decomposição |
| 30 | Golden values IRR/TWR | `#6.3` | 6 | — |
| 31 | Corpus por dialeto completo | `#7.1` | 7 | **corpo** — user story + critério |
| 32 | Otimização de binário | `#7.2` | 7 | **corpo** — user story + critério; comparar com `#2` na toolchain de `#3` |
| 33 | PWA | `#7.3` | 7 | **corpo** — user story + critério |
| 34 | Artefatos de comunidade | `#7.4` | 7 | **corpo** — user story + critério |
| 35 | Release cross-registry | `#7.5` | 7 | — |

14 issues sem alteração · 21 alteradas · 0 fechadas ou removidas.

---

## 5. Issues a alterar — o que muda

Os textos de substituição de `#7`, `#16`, `#17`, `#18`, `#20`, `#21`, `#22`, `#26`, `#29`,
`#31`, `#32`, `#33`, `#34` estão prontos em `auditoria/plano-issues-github.md` (§2 a §5,
"Como deve estar") e são copiados de lá sem reescrita. O texto de `#9` está no mesmo documento,
mas em §7 ("nota de decomposição para `effort:xl`"), fora dessa faixa — citado à parte porque é
um adendo de uma linha, não um "como deve estar" completo. O que segue é o que **não** está em
`plano-issues-github.md`.

**`#2` — Medir `.wasm` do domain isolado.** Acrescentar ao final:
> Precedência: esta medição só tem significado depois que o domínio real existir (N1/N2 do
> alinhamento). Medir o placeholder atual produziria um número sem relação com o orçamento do
> ADR-07.

**`#3` — Fixar toolchain.** Substituir o corpo por checklist do estado real:
> - [x] `rust-toolchain.toml` no repositório — canal `1.97.1`, target `wasm32-unknown-unknown`,
>       componentes `rustfmt` e `clippy`
> - [ ] `wasm-opt` pinado no CI
> - [ ] `wasm-bindgen` pinado no CI
>
> Sem os dois pins restantes, o orçamento de binário do ADR-07 não é reprodutível: recompilar o
> mesmo commit anos depois pode dar `.wasm` de tamanho diferente.

**`#4` — Spike do dialeto Nubank.** Os marcadores de formato foram perdidos na criação. Critério
de aceite corrigido: documentar se o header é `OFXHEADER:100` / `DATA:OFXSGML` (família 1.x) ou
declaração XML com `OFXHEADER="200"` (família 2.x), a versão exata (`VERSION:`) e o encoding
declarado. Última linha: "Bloqueia `#5` — remover `needs:research` só depois de confirmado."

**`#5` — Parser mínimo.** Título: "Parser mínimo para o dialeto Nubank confirmado em `#4`".
Corpo referencia `#4` em vez de "o spike de header".

**`#10` — Corpus sintético.** Duas coisas. Primeiro, o corpo perdeu as referências de achado que
o roadmap traz — restaurar FITID em branco/constante/reciclado (`#77`), header malformado
(`#65`), encoding BR e vírgula decimal (`#18`). Segundo, a camada 1 da R4 cataloga mais modos de
falha do que o corpo lista, com fonte pública, e diz literalmente que "a lista acima já é o
backlog inicial do corpus". Acrescentar:
> - `FITID` reusado/reciclado entre exportações — impede reimportar mais de uma vez sem colisão
>   (valida a chave `(AccountId, Fitid)` do achado `#65`)
> - data malformada ou valor inválido faz o parser pular a transação **em silêncio** — a
>   discrepância só aparece na conciliação (valida parsing parcial + diagnostics, ADR-08)
> - `INTU.BID`/`ORG` ausente — dialeto conhecido de campos institucionais faltando
> - encoding e header malformado de bancos brasileiros: o fork `ofxparse2` existe por causa
>   disso, e é evidência independente de que a dor é catalogada, não hipótese

**`#14` — Consolidação multi-fonte.** Acrescentar:
> Precedência: o critério de aceite descreve mudanças em `dedupe`/`consolidation`, tratados na
> auditoria como código existente. Eles ainda não estão no repositório — N1 precisa entrar antes.

**`#15` — `period_series`.** Falta critério de aceite. Propor:
> Critério de aceite: a série agrupa pela **data local emitida**, não pelo instante UTC (achado
> #66) — agrupar por UTC atribui compras de fim de noite ao mês errado da fatura em UTC−3; a
> janela declarada do extrato (`DTSTART`/`DTEND`, achado #80) entra na proveniência, para
> distinguir "mês sem gasto" de "arquivo faltando".

---

## 6. Issues a criar — o objetivo sem dono

### Obrigatórias — sem elas, um objetivo declarado não tem dono

**N1 · Landing do código da Fase −1 em `raptor-domain`** — M0 · `type:task` `area:domain`
`priority:p0` `effort:m`
> Os módulos `money`, `metric`, `cashflow`, `dedupe`, `consolidation` e `predictive` fecharam os
> achados `#1`/`#2`/`#7`/`#10`/`#13`/`#14`/`#15` em código compilado e testado (24 testes
> unitários + 5 doctests, `clippy -D warnings` limpo), mas nunca entraram no repositório —
> `crates/domain/src/lib.rs` é um placeholder de 18 linhas.
>
> Critério de aceite: os seis módulos no `raptor-domain`, com o rename `ofx-domain` →
> `raptor-domain` aplicado; `cargo test` e `cargo clippy -- -D warnings` verdes; contagem de
> testes registrada. **Destrava `#2`, `#14` e todo o M3/M4.**

**N2 · Modelo de domínio: `Document`/`Account`/`Transaction` + os 5 Message Sets (ADR-04)** —
M0 · `type:story` `area:domain` `priority:p0` `effort:l`
> Como consumidora da biblioteca, quero um modelo tipado que represente um arquivo OFX inteiro,
> para que parser e analytics conversem por tipos e não por convenção.
>
> Critério de aceite: Banking, Credit Card, Investments, Bill Pay e Taxes modelados (ADR-04);
> multi-conta como lista plana; `Money` com moeda obrigatória em todo valor monetário; compila
> para `wasm32-unknown-unknown`.

**N3 · `domain::diagnostic` — `Diagnostic`/`DiagnosticCode` (achado #48)** — M0 ·
`type:task` `area:domain` `priority:p0` `effort:s`
> Os tipos moram em `domain`, não em `parse`: é o que mantém o grafo de dependências acíclico com
> `integrity` dentro de `analytics` (sem isso, `analytics` dependeria de `parse`). `parse` é o
> produtor primário e os reexporta.
>
> Critério de aceite: enum inicial de `DiagnosticCode` com as três famílias do achado `#17`;
> `FitidConflicting` (`#53`) e `FitidUnreliable` (`#77`) presentes.

**N4 · `Transaction` com dois campos temporais (achado #66)** — M0 · `type:task` `area:domain`
`priority:p0` `effort:s`
> Instante UTC (ordenação, janelas, dedupe) **e** data local emitida (agrupamento contábil, mês
> da fatura). Colapsar nos dois num campo só atribui compras entre 21:00 e 23:59 ao mês errado em
> UTC−3 — exatamente para a persona-alvo.
>
> Critério de aceite: os dois campos no tipo; cada função registra na proveniência qual usa;
> offset ausente assume UTC e emite `DateTimezoneNormalized`.

**N5 · Janela declarada `DTSTART`/`DTEND` (achado #80)** — M0 · `type:task` `area:domain`
`priority:p1` `effort:s` — **retitulada em 2026-07-27.**
> Janela declarada da transaction list como campo do modelo. Pré-requisito da distinção "mês sem
> gasto" vs. "arquivo faltando" em `#20`.
>
> Título original cobria também o campo `installment: {n, of}` (achado #76), removido nesta
> revisão — decisão do autor: sem heurística de reconhecimento de marcador de parcela em
> `NAME`/`MEMO`, o modelo segue estritamente o spec OFX. Ver `#21` (fechada) e `#18` (corpo
> atualizado) para o efeito em cascata; detalhe completo no Discovery, ADR-04.

**N7 · `SECURITY.md`** — M1 · `type:task` `area:repo` `priority:p1` `effort:xs`
> O achado `#73` moveu o artefato para quando o **primeiro** parser existe — a superfície de
> input não confiável abre em `#5`, não na Fase 1. Critério de aceite: política de reporte de
> vulnerabilidade publicada antes de `#5` fechar.

**N8 · Parser XML 2.x completo** — M2 · `type:story` `area:parse` `priority:p0` `effort:l`
> Corpo pronto em `auditoria/plano-issues-github.md` §1. Como usuária de banco que exporta OFX
> 2.x, quero que meu arquivo seja lido com a mesma resiliência do caminho SGML.
>
> Critério de aceite: `xml::to_document` com mapeamento próprio (ADR-02, sem AST neutro
> compartilhado); encoding declarado honrado e nunca presumido UTF-8 (achado `#79`); BOM
> consumido antes do sniff; normalização de vírgula decimal e timezone implementada também aqui
> (`#6`/`#18` — duplicação consciente, coberta pelos testes de paridade).

**N9 · Testes de paridade SGML/XML (achado #55)** — M2 · `type:task` `area:parse`
`priority:p0` `effort:m`
> O detector de deriva entre as duas cópias de mapeamento que a ADR-02 aceita conscientemente. O
> gerador de corpus emite pares equivalentes do mesmo conteúdo em 1.x e 2.x; o teste afirma
> `sgml::to_document(a) == xml::to_document(b)`, tolerando só diagnostics específicos de formato.
> Cobre obrigatoriamente as normalizações duplicadas.

**N10 · Precondições de métrica (achado #78)** — M4 · `type:task` `area:analytics`
`priority:p0` `effort:m`
> Toda métrica declara precondições; violação retorna diagnostic tipado (`InsufficientData` |
> `UndefinedMetric`), nunca `NaN`, `Infinity` ou panic. Hoje a regra aparece solta no corpo de
> `#19` como se fosse exceção de uma métrica — é regra geral.
>
> Critério de aceite: um caso gêmeo por precondição — Sharpe com σ=0, z-score com n<2, CAGR com
> valor inicial ≤0, `runway` com burn rate ≤0, VaR/MDD com série menor que a janela, CU com
> limite zero, IRR não-convergente.

### Recomendadas

**N6 · Determinismo de serialização (achado #82)** — M0 · `type:task` `area:domain`
`priority:p1` `effort:xs` — agregações agrupadas saem em coleções ordenadas (`BTreeMap` ou sort
estável), nunca `HashMap` serializado, para golden test byte a byte ser possível.

**N11 · Registrar a decisão de nomenclatura `ofx` vs. `raptor`** — M0 · `type:spike`
`area:repo` `priority:p2` `effort:xs` — **fechada em 2026-07-26.**
> A RA-32 fechou o A#32 com "nome `ofx` mantido — renomear é barato depois (`git mv`), cedo
> demais pra mudar agora". Mas o repositório se chama `raptor` e os crates são `raptor-domain`,
> `raptor-parse`, `raptor-analytics`, `raptor-wasm`: o rename tinha acontecido depois da decisão,
> sem registro em documento nenhum.
>
> **Decisão (supera a RA-32): o nome do projeto é `raptor`.** As autorreferências do projeto nos
> documentos vivos (`discovery-ofx-rust-wasm.md`, `evolucao-ofx.md`, `handoff-ux-design-frontend.md`)
> foram atualizadas de `ofx` para `raptor`, incluindo os nomes de crate no Discovery (33
> ocorrências). Documentos históricos em `docs/auditoria/` não foram tocados — registram o que a
> RA-32 decidiu naquele momento, não o estado atual.

**N12 · Guia "como exportar OFX" por instituição** — M7 · `type:story` `area:web`
`priority:p2` `effort:m`
> `#8` cobre só o Nubank. A R3 levantou o caminho de exportação dos 5 bancos tradicionais + Nubank
> PF/PJ e chama o guia de **conteúdo de onboarding obrigatório**, não de suporte — a tabela pronta
> está lá.
>
> Critério de aceite: o guia cobre os 6 emissores da R3 e trata os dois padrões que quebram o
> pressuposto das jornadas do handoff: (a) o formato quase nunca se chama "OFX" na tela — é
> "Money 2000+", "Money 2000 em diante", "OFX+" ou "Money 100/102", quatro rótulos distintos; (b)
> no Nubank PF não há web, a geração é no app e **o arquivo pode chegar por e-mail** — a jornada
> precisa do passo "baixe o anexo" antes do drag-and-drop.

### Artefatos de ADR-10/ADR-11 — a classe que a tradução para épicos perdeu

O §10 do Discovery nomeia entregáveis de repositório e de Claude Code por fase. O
`roadmap-github.md` os perdeu ao traduzir para épicos, do mesmo modo que perdeu a modelagem de
domínio — e o `auditoria/hierarquia-artefatos-github.md`, o documento tratado aqui como superado,
**os tinha todos**. Nenhum tem issue.

**N22 · `CLAUDE.md` inicial (ADR-11)** — M0 · `type:task` `area:repo` `priority:p1` `effort:s`
> Entregável de Fase 0 no §10. O rascunho está na própria ADR-11, incluindo as invariantes que
> não regridem (moeda obrigatória em `Money`, consolidação particiona por moeda, `ledger` sobre
> eventos classificados, `runway` como família derivada).

**N23 · Hook `PreToolUse` contra commit de dado bancário real em `corpus/` (achado #4)** —
M2 · `type:task` `area:repo` `priority:p0` `effort:s`
> **A única garantia estrutural do achado #4.** A ADR-11 decidiu explicitamente que esta regra é
> *hook*, não instrução em `CLAUDE.md`: o raciocínio registrado é que "nunca commitar extrato
> real" em prosa é "vago o bastante para o modelo poder ignorar sob pressão de contexto" —
> textualmente equiparado a "nunca rode `rm -rf` em produção". Entra no M2 porque é onde `corpus/`
> nasce de fato, via `#10`.
>
> Critério de aceite: `git add`/`git commit` tocando `corpus/` sem o marcador de geração sintética
> é barrado pelo hook em `.claude/settings.json`, com teste manual documentado.

**N24 · `.claude/rules/` + skill `nova-metrica-analytics`** — M2/M5 · `type:task` `area:repo`
`priority:p2` `effort:s`
> `domain-invariants.md` com `paths: ["crates/domain/**"]` (evita que `#2`/`#7`/`#13` regridam
> quando `banking`/`creditcard`/`investment` forem escritos); `wasm-crate.md` com
> `paths: ["crates/wasm/**"]` (limite de input do achado `#9`, perfil de build do ADR-07); skill
> para adicionar métrica seguindo o padrão `Metric<V>`/`Provenance`/eval fraco-forte do §8.

**N25 · `CHANGELOG.md`** — M5 · `type:task` `area:repo` `priority:p2` `effort:xs`
> "Começa a valer na primeira release" (ADR-10) — o M5 é onde a fronteira WASM existe e 1.0 passa
> a ser avaliável.

**N26 · `ISSUE_TEMPLATE` para "arquivo OFX que falha no parsing"** — M7 · `type:task`
`area:repo` `priority:p2` `effort:xs`
> A ADR-10 destaca este template como tendo "valor real aqui", ao contrário dos genéricos: campos
> de dialeto do banco, versão, e se o caso pode ser sintetizado sem dado real — reforçando o
> achado `#4`. Pareia com o canal de reporte de dialeto do `#12`.

**Ampliação de escopo do `#34`**, em vez de mais uma issue: o corpo proposto cobre
`CONTRIBUTING.md`/`CODE_OF_CONDUCT.md`/`RELEASING.md`, mas a ADR-10 lista também
`PULL_REQUEST_TEMPLATE.md`, Dependabot e a checagem de `NOTICE` — esta última condicional, só
obrigatória se alguma dependência (`rust_decimal`, `quick-xml`, `encoding_rs`, `wasm-bindgen`,
`chrono`) propagar `NOTICE` próprio sob Apache-2.0.

### Deliberadamente ausente

**Métrica de sucesso do projeto (A#6) — não vira issue.** A R6 fechou o achado com "não é
importante para este momento do projeto… **nenhuma ação agora**": em tooling FLOSS sem pressão de
tração, definir KPI agora seria cerimônia sem função. Fica registrado aqui para que a ausência
seja legível como decisão, não como esquecimento. Se algum dia reabrir, a própria R6 deixou a
métrica sugerida — *cobertura de dialeto*: quantos bancos do corpus parseiam sem
`FatalParseError`.

---

## 7. Milestone novo — "Backlog — pós-1.0 (H1–H3)"

Sem data. Descrição a colar:

> Horizontes H1–H3 do `docs/evolucao-ofx.md`. **Visão proposta, não decisão** — nenhum item aqui
> é vinculante até virar ADR no Discovery. Existe no tracker para que a discussão sobre o que vem
> depois do 1.0 tenha lugar, não para comprometer escopo.

Oito issues-âncora, todas `type:epic` + `needs:decision`, decompostas só se e quando forem
promovidas:

| Issue | Horizonte | Por quê está aqui |
|---|---|---|
| N14 · Motor de categorização por regras | H1.1 | Capacidade nº 1 de toda ferramenta do mercado; regras declarativas preservam determinismo e proveniência, ML não |
| N15 · Detecção de transferência entre contas próprias | H1.2 | Sem ela, o escopo consolidado conta o pagamento da fatura duas vezes — extensão do achado `#52` para multi-conta |
| N16 · Detecção de recorrência | H1.3 | Pré-requisito de todo o H2; puramente estatístico, cabe no padrão `Metric`/`Provenance` existente |
| N17 · Patrimônio em série temporal | H1.4 | Composição de peças já planejadas (`ledger` + Investments) |
| N18 · Projeção de fluxo e de fatura | H2.1 | Parcelas futuras são o componente determinístico do forecast — vantagem estrutural no caso brasileiro |
| N19 · Orçamento como avaliação | H2.2 | A biblioteca nunca guarda o orçamento, só o avalia |
| N20 · CSV como formato de entrada | H3.2 | **Decidido em 2026-07-27: não decidir agora.** A RA-31 tirou o Open Finance do horizonte ativo e deixou em aberto se "foco 100% OFX" também valia pra CSV — o autor confirmou que sim: ideia de horizonte, sem trabalho associado, reabrir só com sinal concreto de demanda |
| N21 · Anti-roadmap: registro das decisões "nunca" | §5 | Ver abaixo |

**N21** merece nota. É uma issue aberta e fixada (ou `type:decision`, se a premissa 1.1 for
revertida) registrando o que está decidido que **não** entra: sync bancário e Direct Connect,
persistência embutida, categorização por ML no core, UI de produto, e contas
multiusuário/nuvem/telemetria — esta última tratada como binária, sem exceção nem "ping anônimo".
Cada item é uma pressão previsível ("por que não sincroniza direto?"); sem o registro, cada uma
vira discussão nova.

**O que este milestone não é.** Ele não é a resposta ao A#2 ("o produto entrega valor recorrente
ou é visita única?"). Esse achado já foi respondido e o tracker já reflete a resposta: a R2
promoveu recorrência a objetivo declarado, a R5/R7 a moveram para o Épico 3, e a R3 mostrou que a
janela de 60 dias imposta pelos bancos é o mecanismo de recorrência que o produto não precisava
inventar — está no M3, como `#14`/`#15`/`#16`. O backlog existe só para dar lugar à visão de
longo prazo, fora do caminho crítico das Fases 0–7.

---

## 8. Ordem de aplicação

A ordem importa: a taxonomia primeiro, porque é a regra que as outras alterações seguem; a issue
nova do parser XML por último, porque desloca a numeração interna do roadmap.

**Bloco 0 — rastreabilidade.** ✔ *Aplicado neste commit.* Documentos de auditoria versionados em
`docs/auditoria/`, `evolucao-ofx.md` em `docs/`, handoff atualizado para a revisão que gerou
`#22`/`#23`/`#26`, índice em `docs/README.md`. Sem isso, nenhuma issue é auditável.

**Bloco 1 — taxonomia.** Redescrever a label `type:epic`: de "só no issue-guarda-chuva de cada
milestone" para "issue guarda-chuva que agrupa outras — de um milestone inteiro ou de um bloco
coeso dentro dele; toda `epic` declara no corpo que será decomposta ao iniciar o milestone".
Alinha a definição ao uso real de `#17` e `#29`. Aplicar também ao `roadmap-github.md` §1.

**Bloco 2 — alterações mecânicas.** As 21 issues do §4, com os textos do §5 e de
`auditoria/plano-issues-github.md`. Sem interdependência entre elas.

**Bloco 3 — issues novas do MVP.** N1–N12. N1 antes de tudo: é o que destrava `#2` e `#14`.

**Bloco 4 — horizonte.** Milestone de backlog + N14–N21.

**Bloco 5 — sincronizar roadmap e Discovery.** ✔ *Aplicado.* `docs/roadmap-github.md` reescrito:
issues referenciadas pelo número real do GitHub (não mais o esquema `#M.N`), corpo completo
deixou de ser duplicado no documento (só título/labels/origem — o GitHub é a fonte do corpo, o
que evita a divergência que causou o §3.A), taxonomia do §1 com a definição corrigida de
`type:epic`, milestone de backlog documentado com sua descrição pronta. `docs/discovery-ofx-rust-wasm.md`
§10 marcado como narrativa conceitual superada pela R5+R7 (não mais cronograma de execução), com
a contradição do parser XML 2.x corrigida e os entregáveis de ADR-10/ADR-11 realinhados aos
Milestones reais onde cada issue vive.

---

## 9. Como verificar que o tracker reflete os objetivos

A checagem é a tabela, não a memória. Depois de aplicar:

1. **Cobertura do roadmap.** Todo entregável de `roadmap-github.md` §3 tem issue — a coluna
   "Roadmap" do §4 não tem buraco.
1b. **Cobertura do §10 do Discovery.** Todo entregável nomeado lá tem issue — **inclusive os que
   não são código**. Foi a travessia que faltava: o roadmap perdeu a classe inteira de artefatos
   de ADR-10/ADR-11 (`CLAUDE.md`, hook anti-commit de dado real, `.claude/rules/`, `CHANGELOG`,
   templates, Dependabot, `NOTICE`) ao traduzir fases em épicos, e nenhuma checagem baseada só no
   roadmap conseguiria notar.
2. **Cobertura dos achados.** Todo achado marcado `[decisão do autor]` em
   `auditoria/alteracoes-adr.md` (`#48`, `#51`, `#58`, `#66`, `#76`) e todo achado que a DOD
   daquele documento lista como tocando código tem issue ou está fechado no Discovery.
3. **Cobertura da auditoria de solução.** Todo `A#N` de `auditoria/auditoria-solucao.md` está
   resolvido em `resolucao-auditoria.md` **e**, quando a resolução gerou trabalho, esse trabalho
   tem issue — **e quando a resolução foi "nenhuma ação", não tem.** As duas metades importam: a
   primeira pega objetivo sem dono (A#3 → N12), a segunda pega ruído contra decisão fechada
   (A#6 → deliberadamente ausente, §6). O placar final do `resolucao-auditoria.md` é a lista de
   referência: 33 de 33 endereçados, com o "onde" de cada um.
4. **Rastreabilidade.** Todo número citado em corpo de issue resolve para um documento em
   `docs/`. `grep -o "achado #[0-9]*\|A#[0-9]*\|RA-[0-9]*" docs/**/*.md` como rede.
5. **Higiene de bloqueio.** `list_issues state:OPEN` — nenhuma issue com label `needs:*` sem o
   bloqueio nomeado no corpo e sem quem o remove.
6. **Sanidade do repositório.** `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
   verdes — este trabalho não toca código.

---

## 10. Fora de escopo

- **Escrever código.** N1–N10 *abrem* as issues do domínio; não as implementam.
- **Sub-issues nativas e GitHub Projects v2.** A ADR-12 os recusou com o argumento de que os
  milestones já são a decomposição épico→execução. Nada no levantamento mudou isso.
- **Relabelar as 35 issues** para a taxonomia de `auditoria/hierarquia-artefatos-github.md`
  (premissa 1.1).
- **Fechar as decisões `[decisão do autor]` pendentes** por conta própria: as premissas 1.2 e 1.3
  são recomendações aplicáveis com um comando, não fatos consumados.
