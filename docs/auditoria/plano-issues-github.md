# Plano de alteração das issues — roadmap GitHub `ofx`

| Campo | Valor |
|---|---|
| Data | 2026-07-21 |
| Documento-alvo | `roadmap-github.md` |
| Origem | Análise de consistência da especificação das 35 issues |
| Escopo | 1 issue nova · 14 issues alteradas · 1 mudança de taxonomia de label · 1 renumeração |
| Convenção | Cada bloco: **Como está** (verbatim do documento atual) → **Como deve estar** (texto pronto para substituir) |
| Estado | Duas decisões marcadas `[decisão do autor]` bloqueiam a aplicação dos blocos correspondentes; o resto é mecânico |

**Correção de contagem:** na análise que precedeu este plano eu disse "sete issues sem corpo" — são **nove** (`#4.2`, `#4.4`, `#4.5`, `#5.2`, `#6.2`, `#7.1`, `#7.2`, `#7.3`, `#7.4`). A lista estava certa, o número não.

---

## Sumário das alterações

| # | Tipo | Issue(s) | Bloqueado por decisão? |
|---|---|---|---|
| 1 | **Nova issue** | `#2.2` — Parser XML 2.x completo | **Sim** — onde colocar |
| 2 | Alteração | `#3.3` — remover `needs:design`, reescrever corpo | **Sim** — confirmar quebra do ciclo |
| 3 | Alteração | `#1.4` e `#4.1` — declarar dependência cruzada | Não |
| 4 | Alteração | `#4.6` — corrigir `area:` | Não |
| 5 | Alteração | 9 issues sem corpo | Não |
| 6 | Taxonomia | definição de `type:epic` | Não |
| 7 | Alteração | `#2.1` e `#6.2` — nota de decomposição `effort:xl` | Não |
| — | Renumeração | `#2.2`–`#2.5` → `#2.3`–`#2.6` | Consequência do item 1 |

---

## 1. Nova issue — Parser XML 2.x `[decisão do autor]`

**Problema:** nenhuma issue constrói o parser XML 2.x. O `#1.2` faz um parser *mínimo* para a família do dialeto Nubank (SGML **ou** XML, a depender do spike `#1.1`), e o `#2.1` cobre explicitamente só SGML 1.x. Qualquer que seja o resultado do spike, sobra uma família de formato sem dono — e o Discovery original tinha XML na Fase 0 e SGML na Fase 1, então isso se perdeu na tradução para épicos.

**Decisão necessária:** o parser XML completo entra no Milestone 2 (junto do SGML, tratando "parsing resiliente" como um bloco só) ou ganha milestone próprio? **Recomendo Milestone 2** — os dois parsers compartilham corpus, fuzzing e canal de reporte, e separá-los criaria um milestone que entrega meio parser.

### Como está

*(não existe)*

### Como deve estar

```
**#2.2 — Parser XML 2.x completo**
`type:story` `area:parse` `priority:p0` `effort:l`
> Como usuária de banco que exporta OFX 2.x, quero que meu arquivo seja lido com a mesma
> resiliência do caminho SGML, para não depender de qual família o meu banco escolheu.
>
> Contexto: o `#1.2` construiu um parser mínimo para a família do dialeto Nubank (SGML ou XML,
> conforme o spike `#1.1`). Esta issue cobre a família **XML 2.x** — do zero se o `#1.2` tiver
> sido SGML, ou completando o mínimo se o `#1.2` já tiver sido XML.
>
> Critério de aceite: `xml::to_document` com mapeamento próprio (ADR-02 — sem AST neutro
> compartilhado com o SGML); encoding declarado honrado, nunca presumido UTF-8 (achado #79);
> BOM consumido antes do sniff; normalização de vírgula decimal e timezone implementada aqui
> também (achados #6/#18 — duplicação consciente da ADR-02, coberta pelos testes de paridade).
```

---

## 2. `#3.3` — quebrar a dependência circular `[decisão do autor]`

**Problema:** a issue tem `needs:design`, que por definição significa "não começa até a label ser removida". Mas o corpo remete à RA-11/25, que adiou a decisão de versionamento de schema *até existir um consumidor real que persista dados* — e o `#3.3` é justamente esse consumidor. A issue está bloqueada esperando uma decisão que só pode ser tomada depois que ela comece.

**Decisão necessária:** confirmar que a saída é remover o bloqueio e deixar a decisão acontecer *dentro* da issue.

### Como está

```
**#3.3 — Persistência local versionada na demo de referência**
`type:task` `area:web` `priority:p1` `effort:m` `needs:design`
> Como usuária, quero que o produto lembre o que já importei, para não recomeçar do zero a cada
> visita. Versionamento de schema fica fora de escopo por ora (RA-11/25, risco documentado no §9
> do Discovery) — `needs:design` até decidir o modelo mínimo de armazenamento local.
```

### Como deve estar

```
**#3.3 — Persistência local na demo de referência**
`type:story` `area:web` `priority:p1` `effort:m`
> Como usuária, quero que o produto lembre o que já importei, para não recomeçar do zero a cada
> visita — é o que torna o acompanhamento da evolução (R2) possível na prática.
>
> Nota sobre o bloqueio removido: a RA-11/25 adiou a decisão de versionamento de schema até
> existir um consumidor real que persista dados. **Esta issue é esse consumidor** — manter
> `needs:design` criaria um ciclo (a issue esperando uma decisão que só ela viabiliza). O modelo
> mínimo de armazenamento é decidido *dentro* desta issue, não antes dela.
>
> Critério de aceite: documentos importados sobrevivem a um reload da página; a decisão tomada
> sobre carimbo de versão no `Document` (ou a ausência deliberada dele) volta para o Discovery
> como fechamento da RA-11/25.
```

**Mudanças:** removida a label `needs:design`; `type:task` → `type:story` (tem usuária e valor de usuário, ao contrário de uma task técnica); título perde "versionada", que prometia mais do que a issue entrega.

---

## 3. `#1.4` e `#4.1` — declarar a dependência cruzada

**Problema:** o `#1.4` (Milestone 1) precisa de `ledger::total_spent`, que está dentro do `#4.1` (Milestone 4). Isso é inerente a uma fatia vertical e até desejável — mas não está declarado em lugar nenhum, e o `#4.1` não reconhece que parte dele foi construída três milestones antes.

### 3a. `#1.4` — como está

```
**#1.4 — WASM mínimo: `parse_ofx` + `getTotalSpent` rodando no browser**
`type:story` `area:wasm` `priority:p0` `effort:m`
> Como usuária, quero arrastar minha fatura Nubank e ver o total gasto, para ter valor real na
> primeira interação.
> Critério de aceite: demo HTML mínima, sem estilo, mostra o número; nenhum byte sai do browser
> (checar aba de rede do devtools).
```

### 3a. `#1.4` — como deve estar

```
**#1.4 — WASM mínimo: `parse_ofx` + `getTotalSpent` rodando no browser**
`type:story` `area:wasm` `priority:p0` `effort:m`
> Como usuária, quero arrastar minha fatura Nubank e ver o total gasto, para ter valor real na
> primeira interação.
>
> Dependência cruzada declarada: exige uma versão **mínima** de `ledger::total_spent`, cuja forma
> completa vive no `#4.1` (Milestone 4). Isto é intencional — é o que faz desta fatia uma fatia
> *vertical* e não mais uma camada horizontal (A#29). O mínimo aqui é soma de compras e estornos
> excluindo pagamento de fatura (achado #52); o resto de `ledger` fica para o `#4.1`.
>
> Critério de aceite: demo HTML mínima, sem estilo, mostra o número; nenhum byte sai do browser
> (checar aba de rede do devtools).
```

### 3b. `#4.1` — como está

```
**#4.1 — `ledger`: total gasto, NCF, ADB, AT, extremos, gasto por estabelecimento**
`type:epic` `area:analytics` `priority:p0` `effort:xl`
> Guarda-chuva; decompor em uma issue por métrica ao iniciar o milestone.
```

### 3b. `#4.1` — como deve estar

```
**#4.1 — `ledger` completo: NCF, ADB, AT, extremos, gasto por estabelecimento**
`type:epic` `area:analytics` `priority:p0` `effort:xl`
> Guarda-chuva; decompor em uma issue por métrica ao iniciar o milestone.
>
> Escopo já parcialmente entregue: `ledger::total_spent` foi construído em versão mínima no
> `#1.4` (Milestone 1, fatia vertical). Esta issue **completa** `total_spent` (escopo consolidado,
> partição por moeda) e constrói as demais métricas do zero — não recomeça o que já existe.
```

---

## 4. `#4.6` — corrigir o `area:`

**Problema:** a issue está como `area:analytics`, mas a mudança bloqueante (fazer `Provenance` carregar a lista de `(AccountId, Fitid)`) é em `ofx-domain`. Além disso, o `#3.1` já mexe nessa mesma chave um milestone antes.

### Como está

```
**#4.6 — Drill-down: quais transações compuseram este número (decidido — A#22)**
`type:story` `area:analytics` `priority:p0` `effort:m`
> Como usuária leiga, quero clicar num total e ver quais transações o formaram, para verificar se
> o número está certo antes de perder confiança no produto inteiro. Critério de aceite:
> `Provenance` de `total_spent`/`extremes`/`group_by_payee` expõe a lista de `(AccountId, Fitid)`
> das transações somadas; a UI navega até o lançamento individual a partir de qualquer número
> dessas três métricas.
```

### Como deve estar

```
**#4.6 — Drill-down: quais transações compuseram este número (decidido — A#22)**
`type:story` `area:domain` `area:analytics` `priority:p0` `effort:m`
> Como usuária leiga, quero clicar num total e ver quais transações o formaram, para verificar se
> o número está certo antes de perder confiança no produto inteiro.
>
> Atravessa duas crates, por isso duas labels de área: a mudança estrutural é em `ofx-domain`
> (`Provenance` passa a carregar a lista), o consumo é em `ofx-analytics` (cada métrica popula).
> Coordenar com o `#3.1`, que já manipula a chave `(AccountId, Fitid)` no Milestone 3.
>
> Critério de aceite: `Provenance` de `total_spent`/`extremes`/`group_by_payee` expõe a lista de
> `(AccountId, Fitid)` das transações somadas; a UI navega até o lançamento individual a partir de
> qualquer número dessas três métricas.
```

---

## 5. As nove issues sem corpo

O §3 do roadmap declara que toda issue traz "corpo em formato user story + critério de aceite". Nove não trazem. Abaixo, o corpo a acrescentar em cada uma — o título e as labels ficam como estão.

| Issue | Corpo a acrescentar |
|---|---|
| **#4.2** | Como usuária, quero que uma compra parcelada não apareça como anomalia só por ser parcelada, para os alertas serem confiáveis. Critério de aceite: parcelas da mesma compra agregadas via `Transaction.installment` antes do cálculo (achado #76); janela e população declaradas como `ExternalInput` (achado #70). |
| **#4.4** | Como usuária leiga, quero saber se posso confiar nos números antes de agir sobre eles, para não tomar decisão sobre dado torto. Critério de aceite: relatório traduz `DiagnosticCode` cru para linguagem que o leigo entende; score de confiança composto e auditável, nunca número mágico. |
| **#4.5** | Como usuária, quero ver quanto ainda falta pagar de cada compra parcelada, para saber meu comprometimento futuro. Critério de aceite: agrega por compra via `installment`, expondo total, pago e restante; compra sem marcador reconhecido não aparece (dado ausente, não dado errado). |
| **#5.2** | Como consumidora da lib, quero toda a API do Grupo 1–3 (ADR-13) disponível de forma assíncrona, para construir uma UI que nunca trava. Critério de aceite: todas as funções dos Grupos 1, 2 e 3 retornam `Promise`; nenhuma chamada roda na main thread; erro de fronteira é objeto `{code, message}` estruturado (achado #83). |
| **#6.2** | Como usuária avançada, quero retorno e risco calculados sobre eventos de cashflow já classificados, para que aporte não seja confundido com performance. Critério de aceite: consome `cashflow::ClassifiedEvent`, nunca `TRNAMT`/`BALAMT` bruto (achado #13); cada métrica declara precondições e retorna diagnostic tipado em vez de `NaN`/`Infinity` (A#26/#78). Guarda-chuva — decompor por métrica ao iniciar o milestone. |
| **#7.1** | Como mantenedor, quero um gerador de corpus por dialeto cobrindo todos os bancos mapeados, para que regressão de parsing apareça no CI e não no usuário. Critério de aceite: um gerador por dialeto documentando qual peculiaridade real reproduz; nenhum arquivo real usado como fonte (achado #4). |
| **#7.2** | Como usuária em conexão lenta ou mobile, quero que o `.wasm` carregue rápido, para o produto ser usável fora do desktop. Critério de aceite: `wasm-opt -Oz` no pós-build; `twiggy top` decompondo o binário por crate; número final comparado contra a medição do `#0.2` na toolchain fixada pelo `#0.3`. |
| **#7.3** | Como usuária, quero usar o produto sem conexão, para meu extrato ser analisado onde quer que eu esteja. Critério de aceite: demo instalável como PWA e funcional offline após a primeira visita; nenhum `fetch` de rede no runtime (reforça ADR-09 e o CSP do achado #71). |
| **#7.4** | Como mantenedor, quero os artefatos de comunidade no lugar antes do primeiro contribuidor externo aparecer, para não improvisar governança sob pressão. Critério de aceite: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` e `RELEASING.md` (este último descrevendo o pipeline automatizado do `#7.5`, não um checklist manual). |

---

## 6. Taxonomia — definição de `type:epic`

**Problema:** a definição diz que `epic` é "só no issue-guarda-chuva de cada milestone", mas `#4.1` e `#6.2` usam a label como guarda-chuva de um *bloco coeso dentro* de um milestone. O uso é sensato; a definição é que está estreita demais.

### Como está

| Prefixo | Valores | Uso |
|---|---|---|
| `type:` | `epic`, `story`, `task`, `spike`, `bug` | `epic` só no issue-guarda-chuva de cada milestone (opcional, ver §3); `spike` é investigação com saída em decisão/dado, não em código de produto |

### Como deve estar

| Prefixo | Valores | Uso |
|---|---|---|
| `type:` | `epic`, `story`, `task`, `spike`, `bug` | `epic` marca issue guarda-chuva que agrupa outras — seja de um milestone inteiro, seja de um bloco coeso dentro dele (ex.: `ledger` completo, `returns`/`risk`); toda `epic` deve declarar no corpo que será decomposta ao iniciar o milestone. `spike` é investigação com saída em decisão/dado, não em código de produto |

---

## 7. `#2.1` e `#6.2` — nota de decomposição para `effort:xl`

**Problema:** a taxonomia diz que `xl` "é sinal de que a issue provavelmente devia virar duas". Três issues têm a label; só o `#4.1` reconhece isso no corpo.

**`#2.1` — acrescentar ao final do corpo:**

> `effort:xl` reconhecido: decompor ao iniciar o milestone — candidatos naturais de corte são tokenizer, tabela de aridade e mapeamento para o domínio como issues separadas.

**`#6.2` — já coberto** pelo corpo novo proposto no item 5 ("Guarda-chuva — decompor por métrica ao iniciar o milestone").

---

## 8. Renumeração decorrente da issue nova

Se a nova issue entrar como `#2.2` (recomendado — fica ao lado do parser irmão), as seguintes deslizam:

| Antes | Depois | Issue |
|---|---|---|
| `#2.2` | `#2.3` | Corpus sintético semeado pela dor pública catalogada |
| `#2.3` | `#2.4` | Fuzzing do tokenizer SGML e do reader XML |
| `#2.4` | `#2.5` | Canal de reporte de dialeto anonimizado, opt-in |
| `#2.5` | `#2.6` | Robustez adicional Nubank: extrato PJ e fatura malformada |

Nenhuma referência cruzada quebra: as únicas menções a issues específicas em outros pontos do documento são `#1.1`↔`#1.2` (inalteradas) e `#2.1` no §4 (inalterada).

**Alternativa sem renumeração:** entrar como `#2.6` no fim do milestone. Custa a leitura (o parser XML fica longe do SGML), poupa o deslize. Recomendo renumerar — são referências internas de um documento que ainda não virou issue real no GitHub, então o custo é zero.

---

## Ordem sugerida de aplicação

1. Confirmar as duas decisões (`[decisão do autor]`): posição do parser XML e quebra do ciclo do `#3.3`.
2. Aplicar o item 6 (taxonomia) — é o único que muda a regra que os outros seguem.
3. Aplicar itens 3, 4, 5, 7 (mecânicos, sem interdependência).
4. Criar a issue nova (item 1) e renumerar (item 8) por último, para o deslize não invalidar os blocos acima enquanto são aplicados.
