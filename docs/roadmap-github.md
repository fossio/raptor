# Roadmap GitHub — ofx (Milestones · Labels · Issues)

| Campo | Valor |
|---|---|
| Data | 2026-07-21 |
| Fonte | Épicos de `resolucao-auditoria.md` (R5+R7), traduzidos para o vocabulário nativo do GitHub |
| Revisa | ADR-12 do Discovery — "nenhuma [estrutura] agora" foi superada por decisão do autor |
| Mudança desta rodada | Nubank passa a ser o piloto do Milestone 1 (era BB/Bradesco) |
| Uso | Copiar labels na criação do repo, criar milestones na ordem abaixo, abrir as issues com o título/corpo/labels sugeridos |

---

## 1. Labels — taxonomia completa

Convenção `prefixo:valor`, sem as labels default do GitHub (removidas ao criar o repo — ADR-12). Cinco prefixos:

| Prefixo | Valores | Uso |
|---|---|---|
| `type:` | `epic`, `story`, `task`, `spike`, `bug` | `epic` só no issue-guarda-chuva de cada milestone (opcional, ver §3); `spike` é investigação com saída em decisão/dado, não em código de produto |
| `area:` | `domain`, `parse`, `analytics`, `wasm`, `web`, `repo` | Os quatro primeiros são as crates (ADR-01); `web` e `repo` são extensão desta rodada (ADR-12 revisada) — demo/fronteira de consumo e artefatos de comunidade/release não pertencem a nenhuma crate |
| `priority:` | `p0`, `p1`, `p2` | `p0` = bloqueia o milestone corrente; `p1` = no milestone mas não bloqueia; `p2` = desejável, primeiro a sair se o escopo apertar |
| `effort:` | `xs`, `s`, `m`, `l`, `xl` | Estimativa de esforço relativo, não prazo — `xl` é sinal de que a issue provavelmente devia virar duas |
| `needs:` | `decision`, `research`, `design` | Marca bloqueio explícito: issue não começa até a label ser removida (decisão tomada, pesquisa concluída, ou design definido) |

Aplicação manual pela UI enquanto for só o mantenedor (ADR-12) — sync automatizado fica para se/quando o volume justificar.

---

## 2. Milestones (= Épicos)

| # | Milestone | Estado |
|---|---|---|
| 0 | Fundação verificável | Em andamento (`ofx-domain` parcialmente entregue) |
| 1 | Fatia vertical mínima — piloto Nubank | Próximo |
| 2 | Parsing resiliente sobre a dor conhecida | Planejado |
| 3 | Recorrência e consolidação | Planejado |
| 4 | Analytics de gasto da persona leiga | Planejado |
| 5 | Fronteira WASM madura (Web Worker) | Planejado |
| 6 | Analytics avançado de investimento | Planejado |
| 7 | Hardening e lançamento público | Planejado |

Descrição de cada milestone (campo "description" do GitHub) — cole ao criar:

> **Milestone 0 — Fundação verificável.** Workspace multi-crate e domínio tipado compilando para WASM, com medição de tamanho de binário desde já (gate do A#9, não descoberta tardia).

> **Milestone 1 — Fatia vertical mínima: piloto Nubank.** Um banco (Nubank), um dialeto, uma métrica (total gasto), ponta a ponta no browser. Existe produto real e demonstrável antes de perseguir completude (resposta a A#29).

> **Milestone 2 — Parsing resiliente sobre a dor conhecida.** Os 5 bancos tradicionais (BB, Bradesco, Itaú, Caixa, Santander) + robustez adicional do Nubank; corpus semeado por dor pública catalogada; fuzzing; canal de reporte de dialeto opt-in.

> **Milestone 3 — Recorrência e consolidação.** Juntar exportações de 60 em 60 dias num histórico contínuo; evolução mês a mês; persistência local do que já foi importado. O coração do valor recorrente do produto.

> **Milestone 4 — Analytics de gasto da persona leiga.** Gasto por estabelecimento, ticket médio, extremos, anomalias, utilização de crédito, parcelas em aberto.

> **Milestone 5 — Fronteira WASM madura.** API completa (Grupo 1–3) rodando dentro de Web Worker — nunca trava a aba.

> **Milestone 6 — Analytics avançado de investimento.** Retorno e risco (TWR, Sharpe, VaR, MDD) sobre extrato de corretora, modo separado da persona leiga.

> **Milestone 7 — Hardening e lançamento público.** Corpus completo, otimização de binário, PWA, artefatos de comunidade.

---

## 3. Issues por milestone

Cada issue abaixo: título pronto para copiar, labels, e corpo em formato user story + critério de aceite. Numeração `#M.N` é só referência deste documento (M = milestone, N = ordem) — no GitHub, o número real é atribuído na criação.

### Milestone 0 — Fundação verificável

**#0.1 — Workspace multi-crate + domain compilando para wasm32-unknown-unknown**
`type:task` `area:domain` `priority:p0` `effort:m`
> Ação imediata do Discovery: `rustup target add wasm32-unknown-unknown` + `cargo build --target wasm32-unknown-unknown -p ofx-domain`. Critério de aceite: build verde no CI para o alvo wasm32.

**#0.2 — Medir tamanho do `.wasm` do domain isolado**
`type:task` `area:domain` `priority:p0` `effort:s`
> Gate do A#9: medir antes de investir no resto, não só na Fase 5. Critério de aceite: número registrado no README ou em `docs/binary-size.md`, com o comando de medição documentado e reproduzível.

**#0.3 — Fixar toolchain (rust-toolchain.toml + versões pinadas de wasm-opt/wasm-bindgen) (decidido — A#28)**
`type:task` `area:repo` `priority:p0` `effort:s`
> Sem isso, o orçamento de binário do ADR-07 não é reprodutível ao longo do tempo — recompilar o mesmo commit anos depois pode dar `.wasm` de tamanho diferente. Decidido para agora, não para quando o binário for medido/publicado. Critério de aceite: `rust-toolchain.toml` no repo; versões de `wasm-opt`/`wasm-bindgen` pinadas no CI.

---

### Milestone 1 — Fatia vertical mínima: piloto Nubank

**#1.1 — Spike: confirmar dialeto real do OFX de fatura Nubank**
`type:spike` `area:parse` `priority:p0` `effort:xs` `needs:research`
> Inspecionar um arquivo real de fatura Nubank (`Exportar OFX+` no app). Critério de aceite: documentar se o header é `OFXHEADER:100/DATA:OFXSGML` (1.x) ou `<?xml?>`/`<?OFX OFXHEADER="200"?>` (2.x), a versão exata (`VERSION:`), e o encoding declarado. Bloqueia #1.2 — remover a label `needs:research` só depois de confirmado.

**#1.2 — Parser mínimo para o dialeto Nubank confirmado em #1.1**
`type:story` `area:parse` `priority:p0` `effort:l`
> Como mantenedora, quero um parser mínimo (SGML ou XML, conforme #1.1) que leia uma fatura Nubank real e produza um `Document`, para ter o primeiro dado real fluindo pelo domínio.
> Critério de aceite: arquivo de fatura Nubank (do corpus, sintético a partir do formato confirmado) parseia sem erro fatal; `cargo test` verde.

**#1.3 — Tratar ambiguidade de data do Nubank (transação vs. fatura)**
`type:task` `area:domain` `priority:p1` `effort:s`
> A extensão comunitária `nubank-ofx` documenta que a data pode ser a da transação ou a de entrada na fatura (~1 dia de diferença). Critério de aceite: decidir qual data o parser mapeia para `DTPOSTED` e documentar a escolha no ADR-08 do Discovery; não é mudança na decisão do achado #66 (UTC vs. local), é dialeto específico de origem de dado.

**#1.4 — WASM mínimo: `parse_ofx` + `getTotalSpent` rodando no browser**
`type:story` `area:wasm` `priority:p0` `effort:m`
> Como usuária, quero arrastar minha fatura Nubank e ver o total gasto, para ter valor real na primeira interação.
> Critério de aceite: demo HTML mínima, sem estilo, mostra o número; nenhum byte sai do browser (checar aba de rede do devtools).

**#1.5 — Onboarding: como exportar OFX do Nubank**
`type:task` `area:web` `priority:p1` `effort:s`
> Guia curto: app → Faturas → escolher período → "Exportar OFX+"; alertar que a entrega pode vir por e-mail, não download direto (R3). Critério de aceite: texto revisado por alguém que nunca exportou OFX do Nubank antes, sem travar no passo.

---

### Milestone 2 — Parsing resiliente sobre a dor conhecida

**#2.1 — Parser SGML 1.x: tokenizer + tabela de aridade**
`type:story` `area:parse` `priority:p0` `effort:xl`
> Como usuária de banco com agência (BB, Bradesco, Itaú, Caixa, Santander), quero que meu extrato SGML 1.x seja lido mesmo com tags não fechadas, para não ver erro em vez do meu extrato.
> Critério de aceite: golden files dos 5 bancos tradicionais (sintéticos) parseiam; fallback heurístico de aridade coberto por teste com tag proprietária.

**#2.2 — Corpus sintético semeado pela dor pública catalogada**
`type:task` `area:parse` `priority:p0` `effort:l`
> FITID em branco/constante/reciclado, header malformado, encoding BR, vírgula decimal — cada um do achado #77/#65/#18 vira um gerador de dialeto (R4). Critério de aceite: um caso de teste por modo de falha catalogado, nenhum arquivo real usado como fonte (achado #4).

**#2.3 — Fuzzing do tokenizer SGML e do reader XML**
`type:task` `area:parse` `priority:p1` `effort:m`
> `cargo-fuzz` sobre os dois parsers — entregável, não intenção (A#23). Critério de aceite: corpus de fuzzing rodando em CI (ou noturno) por N horas sem crash/hang não tratado.

**#2.4 — Canal de reporte de dialeto anonimizado, opt-in**
`type:story` `area:web` `priority:p1` `effort:m`
> Como usuária cujo arquivo falhou, quero reportar a estrutura do problema sem expor meus dados, para ajudar a melhorar o parser (resolve A#19).
> Critério de aceite: o relatório gerado não contém valor monetário nem identificador pessoal; usuária revê o conteúdo antes de decidir enviar.

**#2.5 — Robustez adicional Nubank: extrato de conta (PJ) e fatura malformada**
`type:story` `area:parse` `priority:p2` `effort:m`
> Como usuária Nubank, quero que meu extrato de conta PJ ("Money 100/102") ou uma fatura malformada continue funcionando, complementando o caminho feliz do Milestone 1.

---

### Milestone 3 — Recorrência e consolidação

**#3.1 — Consolidação multi-fonte (`consolidate_by_fitid`)**
`type:story` `area:domain` `priority:p0` `effort:l`
> Como usuária, quero juntar minhas exportações de 60 em 60 dias num histórico contínuo, porque o banco nunca me dá tudo de uma vez (R3).
> Critério de aceite: chave `(AccountId, Fitid)` (achado #65), payload divergente vira `FitidConflicting` (achado #53), `FitidUnreliable` para fonte degenerada (achado #77) — os três com teste gêmeo.

**#3.2 — `period_series` e comparação com histórico próprio**
`type:story` `area:analytics` `priority:p0` `effort:m`
> Como usuária, quero ver minha evolução mês a mês comparada com meus próprios meses anteriores, para entender se estou gastando mais (resolve A#20).

**#3.3 — Persistência local versionada na demo de referência**
`type:task` `area:web` `priority:p1` `effort:m` `needs:design`
> Como usuária, quero que o produto lembre o que já importei, para não recomeçar do zero a cada visita. Versionamento de schema fica fora de escopo por ora (RA-11/25, risco documentado no §9 do Discovery) — `needs:design` até decidir o modelo mínimo de armazenamento local.

---

### Milestone 4 — Analytics de gasto da persona leiga

**#4.1 — `ledger`: total gasto, NCF, ADB, AT, extremos, gasto por estabelecimento**
`type:epic` `area:analytics` `priority:p0` `effort:xl`
> Guarda-chuva; decompor em uma issue por métrica ao iniciar o milestone.

**#4.2 — `anomaly::z_score` sobre compras (agregando parcelas)**
`type:story` `area:analytics` `priority:p1` `effort:m`

**#4.3 — `credit::utilization` com precondição de `CREDITLIMIT` ausente**
`type:story` `area:analytics` `priority:p1` `effort:s`
> Critério de aceite: arquivo sem `CREDITLIMIT` retorna diagnostic, nunca deriva de `LEDGERBAL`/`AVAILBAL` sem marcar a inferência (achados #59/#78).

**#4.4 — `integrity::health_report`**
`type:story` `area:analytics` `priority:p0` `effort:l`

**#4.5 — `ledger::open_installments` (parcelas em aberto)**
`type:story` `area:analytics` `priority:p1` `effort:m`

**#4.6 — Drill-down: quais transações compuseram este número (decidido — A#22)**
`type:story` `area:analytics` `priority:p0` `effort:m`
> Como usuária leiga, quero clicar num total e ver quais transações o formaram, para verificar se o número está certo antes de perder confiança no produto inteiro. Critério de aceite: `Provenance` de `total_spent`/`extremes`/`group_by_payee` expõe a lista de `(AccountId, Fitid)` das transações somadas; a UI navega até o lançamento individual a partir de qualquer número dessas três métricas.

**#4.7 — Visão "Contas agendadas" a partir do Message Set Bill Pay (decidido — A#5)**
`type:story` `area:analytics` `priority:p2` `effort:m`
> Como usuária, quero ver as contas que já configurei pra pagamento automático no banco, direto do dado estruturado do OFX, complementando (não substituindo) a detecção estatística de recorrência do Milestone 3. Implementa `ledger::scheduled_payments` / `getScheduledPayments(doc)` (ADR-13). Critério de aceite: só entra em desenvolvimento depois de confirmado no corpus que pelo menos um banco popula o bloco Bill Pay.

**#4.8 — Spike: Taxes Message Set é populado por algum banco brasileiro?**
`type:spike` `area:parse` `priority:p2` `effort:xs` `needs:research`
> O Tax Message Set do OFX foi desenhado para formulários fiscais americanos (1099); IR no Brasil usa documentos sem relação com esse bloco. Critério de aceite: verificar no corpus/documentação de bancos BR se o bloco aparece populado. Se não, Taxes permanece modelado por completude (ADR-04), sem visão associada — não abrir story de visão até esta spike concluir.

---

### Milestone 5 — Fronteira WASM madura

**#5.1 — Web Worker: `.wasm` fora da main thread**
`type:story` `area:wasm` `priority:p0` `effort:l`
> Já especificado na ADR-06 revisada do Discovery (worker + `postMessage` + RPC fino). Critério de aceite: arquivo grande/consolidação pesada não trava a UI (testar com throttling de CPU no devtools).

**#5.2 — API completa do Grupo 1–3 (ADR-13) assíncrona**
`type:story` `area:wasm` `priority:p0` `effort:l`

**#5.3 — Atualizar diagramas do §7 e §12 do Discovery para sintaxe `await`**
`type:task` `area:repo` `priority:p2` `effort:s`
> Cosmético — a ordem lógica das chamadas não muda, só a forma.

---

### Milestone 6 — Analytics avançado de investimento

**#6.1 — Toggle de modo avançado: reconfiguração completa de UI/UX (decidido — A#21)**
`type:story` `area:web` `priority:p0` `effort:l`
> Como usuária avançada, quero ativar um modo que reconfigura a interface inteira (navegação, vocabulário, telas) — não um switch que só libera campos na mesma tela — para não ter minha experiência misturada com a da persona leiga de cartão. Critério de aceite: nenhuma tela do modo leigo referencia conceito avançado (Sharpe, VaR) e vice-versa. Nota (A#10, resolvido): cache/memoização de série consolidada é responsabilidade deste consumidor — a lib não guarda handle para nenhum escopo.

**#6.2 — `returns`/`risk`: TWR, IRR, Sharpe, VaR, MDD sobre cashflow classificado**
`type:epic` `area:analytics` `priority:p1` `effort:xl`

**#6.3 — Golden values de IRR/TWR validados contra numpy-financial/LibreOffice**
`type:task` `area:analytics` `priority:p0` `effort:m`
> Resolve A#12/A#14: fonte de verdade externa para os testes fortes do §8, em vez de valor calculado só pelo próprio código. Critério de aceite: cada golden value de IRR/TWR/Sharpe/CAGR documenta a entrada exata usada e a ferramenta de referência (numpy-financial ou LibreOffice Calc) que produziu o número comparado.

---

### Milestone 7 — Hardening e lançamento público

**#7.1 — Corpus por dialeto completo (todos os bancos mapeados)**
`type:task` `area:parse` `priority:p1` `effort:l`

**#7.2 — Otimização de binário (`wasm-opt -Oz`, `twiggy`)**
`type:task` `area:wasm` `priority:p1` `effort:m`

**#7.3 — PWA da demo de referência**
`type:task` `area:web` `priority:p2` `effort:m`

**#7.4 — Artefatos de comunidade (CONTRIBUTING, CODE_OF_CONDUCT, RELEASING)**
`type:task` `area:repo` `priority:p2` `effort:s`

**#7.5 — Automatizar release cross-registry (crates.io + npm) via GitHub Actions (decidido — A#27)**
`type:task` `area:repo` `priority:p1` `effort:m`
> Avaliar `release-plz` ou equivalente pra workspace Rust multi-crate; pipeline cobre a ordem `domain` → `parse`/`analytics` → `wasm` → pacote npm gerado pelo `wasm-bindgen`. Critério de aceite: uma tag de release dispara o pipeline sem passo manual de versão/lockfile.

---

## 4. O que fica fora deste documento

Detalhamento de tasks técnicas dentro de cada story (ex.: subtarefas de `#2.1`) fica para quando o milestone for aberto — decompor cedo demais é o mesmo _documentation theater_ que a ADR-10/12 já rejeitam. Este documento traduz estrutura e nomenclatura; não substitui o julgamento de quebrar uma issue `effort:xl` em menores quando ela for de fato iniciada.
