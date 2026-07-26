# Roadmap GitHub — raptor (Milestones · Labels · Issues)

| Campo | Valor |
|---|---|
| Data | 2026-07-26 (sincronizado com o tracker real) |
| Fonte original | Épicos de `resolucao-auditoria.md` (R5+R7), traduzidos para o vocabulário nativo do GitHub |
| Revisa | ADR-12 do Discovery — "nenhuma [estrutura] agora" foi superada por decisão do autor |
| Estado | **Sincronizado.** As 35 issues originais mais as 25 abertas na rodada de alinhamento (ver `docs/alinhamento-issues.md`) já existem no GitHub com número real. Este documento não é mais o rascunho de onde as issues nascem — é o mapa de milestones e a taxonomia de labels, com as issues referenciadas pelo número real (`#N`) |
| Uso | Corpo e labels de cada issue são o GitHub, não este documento — evita a divergência que motivou a rodada de alinhamento (ver §5). Ao abrir uma issue nova, decidir o milestone aqui, criar no GitHub, e só então acrescentar a linha na tabela correspondente |

---

## 1. Labels — taxonomia completa

Convenção `prefixo:valor`, sem as labels default do GitHub (removidas ao criar o repo — ADR-12). Cinco prefixos:

| Prefixo | Valores | Uso |
|---|---|---|
| `type:` | `epic`, `story`, `task`, `spike`, `bug` | `epic` marca issue guarda-chuva que agrupa outras — seja de um milestone inteiro, seja de um bloco coeso dentro dele (ex.: `ledger` completo, `returns`/`risk`); toda `epic` deve declarar no corpo que será decomposta ao iniciar o milestone. `spike` é investigação com saída em decisão/dado, não em código de produto |
| `area:` | `domain`, `parse`, `analytics`, `wasm`, `web`, `repo` | Os quatro primeiros são as crates (`raptor-domain`/`parse`/`analytics`/`wasm`, ADR-01); `web` e `repo` são extensão desta rodada (ADR-12 revisada) — demo/fronteira de consumo e artefatos de comunidade/release não pertencem a nenhuma crate |
| `priority:` | `p0`, `p1`, `p2` | `p0` = bloqueia o milestone corrente; `p1` = no milestone mas não bloqueia; `p2` = desejável, primeiro a sair se o escopo apertar |
| `effort:` | `xs`, `s`, `m`, `l`, `xl` | Estimativa de esforço relativo, não prazo — `xl` é sinal de que a issue provavelmente devia virar duas |
| `needs:` | `decision`, `research`, `design` | Marca bloqueio explícito: issue não começa até a label ser removida (decisão tomada, pesquisa concluída, ou design definido) |

*Correção desta rodada (§8, Bloco 1 de `alinhamento-issues.md`):* a definição de `type:epic` mudou de "só no issue-guarda-chuva de cada milestone" para o texto acima, para bater com o uso real de `#17` e `#29`. A tabela traz a versão completa; a label do GitHub tem limite de 100 caracteres na descrição, então a forma real a aplicar lá é a curta abaixo (97 caracteres):

> Guarda-chuva que agrupa outras issues (milestone ou bloco coeso); decompor ao iniciar o milestone

**Ação manual:** `gh label edit "type:epic" --repo fossio/raptor --description "Guarda-chuva que agrupa outras issues (milestone ou bloco coeso); decompor ao iniciar o milestone"` — nenhuma ferramenta desta sessão edita descrição de label via API.

Aplicação manual pela UI enquanto for só o mantenedor (ADR-12) — sync automatizado fica para se/quando o volume justificar.

---

## 2. Milestones (= Épicos)

| # | Milestone | Estado |
|---|---|---|
| 0 | Fundação verificável | Em andamento — domínio real ainda não commitado (`#39`–`#46` abrem o trabalho) |
| 1 | Fatia vertical mínima — piloto Nubank | Próximo |
| 2 | Parsing resiliente sobre a dor conhecida | Planejado |
| 3 | Recorrência e consolidação | Planejado |
| 4 | Analytics de gasto da persona leiga | Planejado |
| 5 | Fronteira WASM madura (Web Worker) | Planejado |
| 6 | Analytics avançado de investimento | Planejado |
| 7 | Hardening e lançamento público | Planejado |
| — | **Backlog — pós-1.0 (H1–H3)** | Visão proposta, não vinculante — ver §4 |

Descrição de cada milestone (campo "description" do GitHub):

> **Milestone 0 — Fundação verificável.** Workspace multi-crate e domínio tipado compilando para WASM, com medição de tamanho de binário desde já (gate do A#9, não descoberta tardia).

> **Milestone 1 — Fatia vertical mínima: piloto Nubank.** Um banco (Nubank), um dialeto, uma métrica (total gasto), ponta a ponta no browser. Existe produto real e demonstrável antes de perseguir completude (resposta a A#29).

> **Milestone 2 — Parsing resiliente sobre a dor conhecida.** Os 5 bancos tradicionais (BB, Bradesco, Itaú, Caixa, Santander) + robustez adicional do Nubank + parser XML 2.x completo; corpus semeado por dor pública catalogada; fuzzing; canal de reporte de dialeto opt-in.

> **Milestone 3 — Recorrência e consolidação.** Juntar exportações de 60 em 60 dias num histórico contínuo; evolução mês a mês; persistência local do que já foi importado. O coração do valor recorrente do produto.

> **Milestone 4 — Analytics de gasto da persona leiga.** Gasto por estabelecimento, ticket médio, extremos, anomalias, utilização de crédito, parcelas em aberto.

> **Milestone 5 — Fronteira WASM madura.** API completa (Grupo 1–3) rodando dentro de Web Worker — nunca trava a aba.

> **Milestone 6 — Analytics avançado de investimento.** Retorno e risco (TWR, Sharpe, VaR, MDD) sobre extrato de corretora, modo separado da persona leiga.

> **Milestone 7 — Hardening e lançamento público.** Corpus completo, otimização de binário, PWA, artefatos de comunidade.

**Milestone de backlog, ainda não criado no GitHub** — nenhuma ferramenta disponível na rodada de alinhamento cria milestones via API. Descrição pronta para colar ao criar:

> Horizontes H1–H3 do `docs/evolucao-ofx.md`. **Visão proposta, não decisão** — nenhum item aqui é vinculante até virar ADR no Discovery. Existe no tracker para que a discussão sobre o que vem depois do 1.0 tenha lugar, não para comprometer escopo.

As issues `#56`–`#63` pertencem a este milestone e estão sem atribuição até ele existir — atribuir manualmente.

---

## 3. Issues por milestone

Título e labels abaixo refletem o estado real no GitHub em 2026-07-26. **Corpo completo de cada issue vive só no GitHub** — não duplicado aqui, para não recriar a divergência que motivou `docs/alinhamento-issues.md`. A coluna "Origem" marca o que mudou nesta rodada de sincronização.

### Milestone 0 — Fundação verificável

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#2`](../../../issues/2) | Medir tamanho do `.wasm` do domain isolado | `type:task` `area:domain` `p0` `effort:s` | corpo — nota de precedência (depende de `#39`/`#40`) |
| [`#3`](../../../issues/3) | Fixar toolchain (decidido — A#28) | `type:task` `area:repo` `p0` `effort:s` | corpo — checklist do estado real |
| [`#39`](../../../issues/39) | Landing do código da Fase −1 em `raptor-domain` | `type:task` `area:domain` `p0` `effort:m` | **novo** — destrava `#2`/`#14`/M3/M4 |
| [`#40`](../../../issues/40) | Modelo de domínio: `Document`/`Account`/`Transaction` + 5 Message Sets | `type:story` `area:domain` `p0` `effort:l` | **novo** |
| [`#41`](../../../issues/41) | `domain::diagnostic` (achado #48) | `type:task` `area:domain` `p0` `effort:s` | **novo** |
| [`#42`](../../../issues/42) | `Transaction` com dois campos temporais (achado #66) | `type:task` `area:domain` `p0` `effort:s` | **novo** |
| [`#43`](../../../issues/43) | `Transaction.installment` + janela declarada (achados #76/#80) | `type:task` `area:domain` `p1` `effort:s` | **novo** |
| [`#44`](../../../issues/44) | Determinismo de serialização (achado #82) | `type:task` `area:domain` `p1` `effort:xs` | **novo** |
| [`#45`](../../../issues/45) | Registrar a decisão de nomenclatura `ofx` vs. `raptor` | `type:spike` `area:repo` `p2` `effort:xs` `needs:decision` | **novo** |
| [`#46`](../../../issues/46) | `CLAUDE.md` inicial (ADR-11) | `type:task` `area:repo` `p1` `effort:s` | **novo** |

### Milestone 1 — Fatia vertical mínima: piloto Nubank

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#4`](../../../issues/4) | Spike: confirmar dialeto real do OFX de fatura Nubank | `type:spike` `area:parse` `p0` `effort:xs` `needs:research` | corpo — marcadores de header restaurados |
| [`#5`](../../../issues/5) | Parser mínimo para o dialeto Nubank confirmado em `#4` | `type:story` `area:parse` `p0` `effort:l` | título + corpo — referencia `#4` |
| [`#6`](../../../issues/6) | Tratar ambiguidade de data do Nubank | `type:task` `area:domain` `p1` `effort:s` | — |
| [`#7`](../../../issues/7) | WASM mínimo: `parse_ofx` + `getTotalSpent` | `type:story` `area:wasm` `p0` `effort:m` | corpo — dependência cruzada com `#17` |
| [`#8`](../../../issues/8) | Onboarding: como exportar OFX do Nubank | `type:task` `area:web` `p1` `effort:s` | — |
| [`#47`](../../../issues/47) | `SECURITY.md` | `type:task` `area:repo` `p1` `effort:xs` | **novo** — achado #73: abre com o primeiro parser |

### Milestone 2 — Parsing resiliente sobre a dor conhecida

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#9`](../../../issues/9) | Parser SGML 1.x: tokenizer + tabela de aridade | `type:story` `area:parse` `p0` `effort:xl` | corpo — nota de decomposição |
| [`#10`](../../../issues/10) | Corpus sintético semeado pela dor pública catalogada | `type:task` `area:parse` `p0` `effort:l` | corpo — referências restauradas + backlog R4 |
| [`#11`](../../../issues/11) | Fuzzing do tokenizer SGML e do reader XML | `type:task` `area:parse` `p1` `effort:m` | — |
| [`#12`](../../../issues/12) | Canal de reporte de dialeto anonimizado, opt-in | `type:story` `area:web` `p1` `effort:m` | — |
| [`#13`](../../../issues/13) | Robustez adicional Nubank: PJ e fatura malformada | `type:story` `area:parse` `p2` `effort:m` | — |
| [`#48`](../../../issues/48) | Parser XML 2.x completo | `type:story` `area:parse` `p0` `effort:l` | **novo** — lacuna identificada em `plano-issues-github.md` §1 |
| [`#49`](../../../issues/49) | Testes de paridade SGML/XML (achado #55) | `type:task` `area:parse` `p0` `effort:m` | **novo** |
| [`#50`](../../../issues/50) | Hook `PreToolUse` contra commit de dado bancário real (achado #4) | `type:task` `area:repo` `p0` `effort:s` | **novo** — única garantia estrutural do achado #4 |
| [`#51`](../../../issues/51) | `.claude/rules/` + skill `nova-metrica-analytics` | `type:task` `area:repo` `p2` `effort:s` | **novo** |

### Milestone 3 — Recorrência e consolidação

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#14`](../../../issues/14) | Consolidação multi-fonte (`consolidate_by_fitid`) | `type:story` `area:domain` `p0` `effort:l` | corpo — nota de precedência (depende de `#39`) |
| [`#15`](../../../issues/15) | `period_series` e comparação com histórico próprio | `type:story` `area:analytics` `p0` `effort:m` | corpo — critério de aceite acrescentado |
| [`#16`](../../../issues/16) | Persistência local na demo de referência | `type:story` `area:web` `p1` `effort:m` | título + corpo + labels — `needs:design` removido, `task`→`story` |

### Milestone 4 — Analytics de gasto da persona leiga

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#17`](../../../issues/17) | `ledger` completo: NCF, ADB, AT, extremos, gasto por estabelecimento | `type:epic` `area:analytics` `p0` `effort:xl` | título + corpo — reconhece `#7` |
| [`#18`](../../../issues/18) | `anomaly::z_score` sobre compras | `type:story` `area:analytics` `p1` `effort:m` | corpo — user story + critério |
| [`#19`](../../../issues/19) | `credit::utilization` com precondição de `CREDITLIMIT` | `type:story` `area:analytics` `p1` `effort:s` | — |
| [`#20`](../../../issues/20) | `integrity::health_report` | `type:story` `area:analytics` `p0` `effort:l` | corpo — user story + critério |
| [`#21`](../../../issues/21) | `ledger::open_installments` | `type:story` `area:analytics` `p1` `effort:m` | corpo — user story + critério |
| [`#22`](../../../issues/22) | Drill-down: quais transações compuseram este número (decidido — A#22) | `type:story` `area:domain` `area:analytics` `p0` `effort:m` | labels + corpo — `+area:domain`, coordenar com `#14` |
| [`#23`](../../../issues/23) | Visão "Contas agendadas" (Bill Pay, decidido — A#5) | `type:story` `area:analytics` `p2` `effort:m` | — |
| [`#24`](../../../issues/24) | Spike: Taxes Message Set populado por banco BR? | `type:spike` `area:parse` `p2` `effort:xs` `needs:research` | — |
| [`#52`](../../../issues/52) | Precondições de métrica (achado #78) | `type:task` `area:analytics` `p0` `effort:m` | **novo** — regra geral, não exceção de uma métrica |

### Milestone 5 — Fronteira WASM madura

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#25`](../../../issues/25) | Web Worker: `.wasm` fora da main thread | `type:story` `area:wasm` `p0` `effort:l` | — |
| [`#26`](../../../issues/26) | API completa do Grupo 1–3 assíncrona | `type:story` `area:wasm` `p0` `effort:l` | corpo — user story + critério |
| [`#27`](../../../issues/27) | Atualizar diagramas do Discovery para sintaxe `await` | `type:task` `area:repo` `p2` `effort:s` | — |
| [`#53`](../../../issues/53) | `CHANGELOG.md` | `type:task` `area:repo` `p2` `effort:xs` | **novo** |

### Milestone 6 — Analytics avançado de investimento

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#28`](../../../issues/28) | Toggle de modo avançado (decidido — A#21) | `type:story` `area:web` `p0` `effort:l` | — |
| [`#29`](../../../issues/29) | `returns`/`risk`: TWR, IRR, Sharpe, VaR, MDD | `type:epic` `area:analytics` `p1` `effort:xl` | corpo — user story + critério + decomposição |
| [`#30`](../../../issues/30) | Golden values de IRR/TWR contra numpy-financial/LibreOffice | `type:task` `area:analytics` `p0` `effort:m` | — |

### Milestone 7 — Hardening e lançamento público

| # | Título | Labels | Origem |
|---|---|---|---|
| [`#31`](../../../issues/31) | Corpus por dialeto completo | `type:task` `area:parse` `p1` `effort:l` | corpo — user story + critério |
| [`#32`](../../../issues/32) | Otimização de binário (`wasm-opt -Oz`, `twiggy`) | `type:task` `area:wasm` `p1` `effort:m` | corpo — user story + critério |
| [`#33`](../../../issues/33) | PWA da demo de referência | `type:task` `area:web` `p2` `effort:m` | corpo — user story + critério |
| [`#34`](../../../issues/34) | Artefatos de comunidade (CONTRIBUTING, CODE_OF_CONDUCT, RELEASING) | `type:task` `area:repo` `p2` `effort:s` | corpo — escopo ampliado (+ PR template, Dependabot, NOTICE) |
| [`#35`](../../../issues/35) | Automatizar release cross-registry (decidido — A#27) | `type:task` `area:repo` `p1` `effort:m` | — |
| [`#54`](../../../issues/54) | Guia "como exportar OFX" por instituição | `type:story` `area:web` `p2` `effort:m` | **novo** — `#8` cobria só Nubank |
| [`#55`](../../../issues/55) | `ISSUE_TEMPLATE` para "arquivo OFX que falha no parsing" | `type:task` `area:repo` `p2` `effort:xs` | **novo** |

### Backlog — pós-1.0 (H1–H3), sem milestone atribuído

| # | Título | Horizonte |
|---|---|---|
| [`#56`](../../../issues/56) | Motor de categorização por regras | H1.1 |
| [`#57`](../../../issues/57) | Detecção de transferência entre contas próprias | H1.2 |
| [`#58`](../../../issues/58) | Detecção de recorrência | H1.3 |
| [`#59`](../../../issues/59) | Patrimônio em série temporal | H1.4 |
| [`#60`](../../../issues/60) | Projeção de fluxo e de fatura | H2.1 |
| [`#61`](../../../issues/61) | Orçamento como avaliação | H2.2 |
| [`#62`](../../../issues/62) | CSV como formato de entrada | H3.2 — decisão em aberto (RA-31) |
| [`#63`](../../../issues/63) | Anti-roadmap: registro das decisões "nunca" | §5 de `evolucao-ofx.md` |

**Deliberadamente ausente:** métrica de sucesso do projeto (A#6) — a R6 fechou com "nenhuma ação agora"; abrir issue contradiria decisão fechada. Ver `docs/alinhamento-issues.md` §6.

---

## 4. Lacunas conhecidas desta sincronização

Duas classes de trabalho que a rodada de alinhamento (`docs/alinhamento-issues.md`) identificou e que nenhuma ferramenta disponível resolveu via API — pendentes de ação manual no GitHub:

1. **Milestone "Backlog — pós-1.0 (H1–H3)"** não existe; `#56`–`#63` estão sem milestone.
2. **Descrição da label `type:epic`** não foi atualizada no GitHub; só o texto deste documento (§1) reflete a definição corrigida.

---

## 5. O que fica fora deste documento

Detalhamento de tasks técnicas dentro de cada story (ex.: subtarefas de `#9`) fica para quando o milestone for aberto — decompor cedo demais é o mesmo _documentation theater_ que a ADR-10/12 já rejeitam. Corpo completo, discussão e histórico de cada issue vivem no GitHub — este documento é o mapa de milestones e taxonomia, não a fonte do texto. Isso é uma mudança de papel deliberada em relação à versão anterior deste documento: manter corpo duplicado aqui foi exatamente o que permitiu `plano-issues-github.md` (15 correções escritas) ficar sem aplicar por meses sem que ninguém notasse a divergência — ver `docs/alinhamento-issues.md` §3.A.
