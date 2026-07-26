# Alterações no Discovery/ADR ofx — fechamento da terceira rodada (achados #48–#64)

| Campo | Valor |
|---|---|
| Documento-alvo | `discovery-ofx-rust-wasm.md` |
| Fonte dos achados | `analise-critica-adr.md` (terceira rodada, #48–#85 — três passadas) |
| Ordem | Por posição no documento (topo → fim), para aplicação mecânica em uma passada |
| Convenção | Cada bloco: **Onde** · **Achado** · **Alteração** · marcação `[decisão do autor]` quando a edição depende de confirmação antes de aplicar |
| **Status** | **Plano integralmente aplicado ao Discovery**, incluindo as decisões que dependiam do autor. Documento histórico — registro do plano; não editar para refletir aplicação.

## Decisões que precisam do autor antes da aplicação

Cinco achados não são edição mecânica — mudam decisão registrada como fechada ou criam decisão nova. Confirmar antes de editar; o restante do documento pode ser aplicado independente destas:

1. **#48** — onde moram `Diagnostic`/`DiagnosticCode`. Recomendação: mover para `ofx-domain::diagnostic` (mantém o grafo acíclico; muda quem versiona o enum do achado #17). Alternativa: aceitar aresta `analytics → parse` e atualizar §4.
2. **#51** — comportamento do escopo consolidado multi-moeda. Recomendação: partição por `CurrencyCode`, nunca conversão (coerente com achado #2 e ADR-09).
3. **#58** — LGPL-2.1-or-later com interpretação registrada vs. MPL-2.0. Recomendação: qualquer uma serve à prioridade de copyleft; o que não pode é publicar no crates.io sem o registro.
4. **#66** — semântica de data: instante UTC para ordenação/janelas + data local emitida para agrupamento contábil. Recomendação: separar os dois usos — muda o modelo de `Transaction` (Fase 0) e refina o fechamento do achado #6, por isso não é edição mecânica.
5. **#76** — parcelamento: mínimo (campo `installment` + normalização de payee) vs. produto completo (visão "compras parceladas em andamento"). Recomendação: os dois — o mínimo é obrigatório pra `anomaly`/`group_by_payee` não degradarem; a visão é a segunda pergunta mais comum da persona.

As edições abaixo assumem as recomendações. Se o autor decidir diferente, os blocos marcados mudam de texto, não de local.

---

## Sumário executivo

**Achado #60.** Na frase "compila nativamente, 24 testes unitários + 5 doctests passam", nenhuma mudança — este é o número vigente. A correção é no roadmap (ver bloco §10 abaixo). Adicionar ao parágrafo "Estado atual", após "validação técnica (achados #41–47) já aplicadas": `Terceira rodada de consistência (achados #48–64) aplicada — deriva pós-reversões de ADR-02/06, grafo de dependências do integrity, multi-moeda no escopo consolidado.`

**Achado #51.** Adicionar ao parágrafo "Decisões que mais mudam o resultado se ignoradas": `; escopo consolidado particiona por moeda, nunca converte (achado #51 — não existe fonte de câmbio offline)`.

## §4 — Visão de arquitetura

**Achado #61.** Na árvore do layout, linha de `domain`, substituir:
`└── src/{lib,money,metric,cashflow,predictive,dedupe}.rs`
por:
`└── src/{lib,money,metric,cashflow,predictive,dedupe,consolidation,diagnostic}.rs`
(`consolidation` já existe em código — ADR-04; `diagnostic` entra pelo achado #48, bloco ADR-08 abaixo).

**Achado #48 `[decisão do autor]`.** Após o parágrafo do grafo de dependências ("`domain` não depende de ninguém…"), adicionar: `Os tipos Diagnostic/DiagnosticCode moram em domain::diagnostic (achado #48) — parse é o produtor primário e os reexporta; integrity (analytics) é consumidor. É o que mantém o grafo acíclico com integrity dentro de analytics: sem isso, analytics dependeria de parse.`

## §8 — Estratégia de testes e evals

**Achado #55.** Nova subseção após o bloco do achado #5:

> **[Achado #55 fechado como decisão] Testes de paridade entre parsers.** A duplicação de mapeamento aceita na ADR-02 ganha detector de deriva: o gerador de corpus (achado #4) emite pares equivalentes do mesmo conteúdo semântico em forma 1.x (SGML) e 2.x (XML), e um teste de paridade afirma `sgml::to_document(a) == xml::to_document(b)` — igualdade do `Document`, tolerando apenas diagnostics específicos de formato (charset, aridade). Cobre obrigatoriamente as normalizações duplicadas (vírgula decimal #18, timezone #6). Roda a partir da Fase 1, quando os dois parsers coexistem.

**Achado #56.** Novo item na lista fraco→forte:

> - **Fronteira `Decimal` (round-trip)** — Fraco: verificar que a serialização não dá erro. Forte: `Document` com valores sem representação exata em `f64` (`0.10`, `0.20`, somas que divergem em ponto flutuante) → `Document → JsValue → Document` **deve** preservar igualdade exata de todo `Money`; caso gêmeo passando o mesmo valor por `f64` que **deve** falhar (prova que a rota string está de fato em uso — achado #56).

**Achado #59.** No item `credit (CU)`, adicionar segundo caso gêmeo: `; e caso gêmeo sem CREDITLIMIT no arquivo, que **deve** retornar CU indisponível com diagnostic — nunca valor derivado de LEDGERBAL/AVAILBAL apresentado como declarado (achado #59; se derivação for oferecida, é opt-in com ExternalInput marcando a inferência)`.

## ADR-02 — Dois parsers independentes

**Achado #55.** No parágrafo de trade-off ("Duplicação de mapeamento entre `sgml`/`xml` é aceita conscientemente"), adicionar ao final: `A deriva entre as duas cópias é detectada pelos testes de paridade do §8 (achado #55) — pares equivalentes 1.x/2.x do corpus devem produzir o mesmo Document.`

## ADR-04 — Modelagem de domínio / consolidação

**Achado #53.** No parágrafo de `dedupe` (achado #10) e no de `consolidate_by_fitid`, substituir a semântica first-wins incondicional por:

> Em `FITID` repetido, o payload das ocorrências é comparado: idêntico → cópia, descartada e reportada como `DuplicateFitid` (comportamento atual); **divergente** (valor, data ou tipo diferem) → conflito real, não cópia — a precedência cronológica ainda escolhe qual ocorrência fica, mas o descarte é reportado como `DiagnosticCode::FitidConflicting`, família "exigem atenção" (achado #17), nunca silenciosamente como duplicata. `integrity` traduz o conflito pro leigo ("uma transação aparece com valores diferentes em duas faturas"). *Este achado toca código existente e testado (`dedupe`/`consolidation`) — inclui atualização dos testes com caso gêmeo payload-idêntico vs. payload-divergente.*

**Achado #63.** Ao final do parágrafo de `consolidate_by_fitid`, adicionar: `O ConsolidationOutcome é exatamente o objeto que a fronteira serializa como Portfolio na ADR-13 — mesma estrutura, dois nomes por camada (Rust/JS); o relatório de consolidação (duplicates_removed, conflicts, sources) viaja dentro dele (achado #49).`

**Achado #51 `[decisão do autor]`.** Novo parágrafo após consolidação multi-fonte:

> **[Achado #51 fechado como decisão] Multi-moeda na consolidação.** `consolidate_by_fitid` particiona a saída por `CurrencyCode` — nunca soma nem converte entre moedas (não existe fonte de câmbio offline, e ADR-09 impede buscá-la; conversão automática é estruturalmente impossível neste projeto). `ConsolidationOutcome` carrega uma série por moeda; toda visão consolidada da ADR-13 renderiza um bloco por moeda ("Total: R$ X · US$ Y"); `integrity` reporta a presença de múltiplas moedas. É a metade de produto do achado #2: lá o domínio recusa a soma; aqui o produto define o que mostrar no lugar dela.

## ADR-05 — Analytics

**Achado #52.** Na linha de `ledger` do mapa de módulos, adicionar: `— agregações de gasto consomem eventos classificados/filtrados por TRNTYPE, nunca TRNAMT bruto (achado #52, ver ADR-13)`.

## ADR-06 — Fronteira WASM

**Achado #54.** Atualizar assinaturas e exemplos:
- Rust: `pub fn parse_ofx(bytes: &[u8], max_bytes: usize) -> Result<JsValue, JsValue>`, com a checagem `bytes.len() > max_bytes → erro imediato` como primeira linha do corpo (materializa o achado #9 no código de exemplo — é o que o agente da Fase 5 copia).
- JS: `const doc = parse_ofx(bytes, MAX_BYTES); // teto decidido pelo integrador (achado #9), não pela lib`.

**Achado #56.** Substituir, na "Nota verificada para a Fase 5", o trecho "decisão adiada para quando `wasm` existir" por:

> **[Achado #56 fechado como decisão]** `Decimal` atravessa a fronteira como **string**, nunca número JS: a feature `serde-float` do `rust_decimal` fica proibida no workspace (mesma classe de risco do achado #41 — compila normal, corrompe o invariante do ADR-03 em silêncio), e a feature `wasm` do `rust_decimal` segue proibida em `domain`, opcional só dentro de `wasm` se o FFI manual ficar verboso. Eval de round-trip no §8 garante a rota.

## ADR-08 — Erros e resiliência

**Achado #48 `[decisão do autor]`.** No fechamento do achado #17, substituir "O enum é versionado junto da API pública de `ofx-parse`" por: `O enum mora em ofx-domain::diagnostic e é versionado junto da API pública de domain (achado #48 — permite integrity consumi-lo sem aresta analytics → parse); ofx-parse o reexporta e é o produtor primário.`

**Achado #53.** Adicionar `FitidConflicting` à lista da família "achados que exigem atenção", com a nota `(payload divergente sob o mesmo FITID — ver ADR-04, achado #53)`.

## ADR-10 — Artefatos de repositório

**Achado #58 `[decisão do autor]`.** No parágrafo da `LICENSE`, adicionar ao final:

> Registro de conformidade (achado #58): LGPL foi desenhada pra linkagem dinâmica e Rust linka estático — consumo do `.wasm` pelo host JS é a fronteira dinâmica natural (relink = trocar o arquivo, conformidade trivial); consumo das crates via Cargo embute a lib no binário do consumidor e aciona a §6 da LGPL-2.1 (permitir relink: objetos ou fonte). A interpretação adotada por este projeto vai documentada no README antes da publicação no crates.io. Alternativa avaliada e disponível se o atrito se mostrar real: MPL-2.0 (copyleft por arquivo, sem fricção de linkagem estática, preserva "modificações voltam") — reabrir só com sinal concreto, não especulativamente.

## ADR-11 — Artefatos de Claude Code

**Achado #64.** No rascunho de CLAUDE.md, corrigir a linha do runway para: `` - `predictive::runway` é família derivada (`fn(Money, &Metric<Money>) -> Metric<Decimal>`) — não force a assinatura primária nela ``.

**Achados #51/#52.** Adicionar duas linhas às "Invariantes que não regridem":
`` - Consolidação particiona por moeda, nunca converte — não existe câmbio offline ``
`` - `ledger` de gasto consome eventos classificados/TRNTYPE filtrado, nunca `TRNAMT` bruto ``

## ADR-13 — Produto

**Achado #52.** No parágrafo de nomenclatura do `ledger`, adicionar:

> **[Achado #52 fechado como decisão]** Toda agregação de *gasto* (`total_spent`, `extremes`, `average_ticket`, `group_by_payee`) consome eventos filtrados por natureza do lançamento (`TRNTYPE`/classificação de `cashflow`): compra e estorno entram, **pagamento de fatura não** — somar `TRNAMT` bruto num statement de cartão mistura compra com o pagamento da fatura anterior e mostra número errado pra pergunta "quanto gastei". É a versão contábil do achado #13. `net_cash_flow` permanece como métrica distinta (líquido é pergunta legítima; só não é "total gasto"). O filtro aplicado entra na proveniência.

Na tabela de visões, separar a primeira linha em duas: `Total gasto no período → ledger::total_spent (compras + estornos, sem pagamento de fatura — achado #52)` e `Fluxo líquido do período → ledger::net_cash_flow (NCF — inclui pagamentos)`.

**Achado #51 `[decisão do autor]`.** No parágrafo "Escopo de análise", adicionar: `Quando as fontes carregam mais de uma moeda, o escopo consolidado é particionado por CurrencyCode — ver decisão na ADR-04 (achado #51); nenhuma visão soma entre moedas.`

**Achado #48 `[decisão do autor]`.** No parágrafo do `integrity`, ajustar: "Consome os `Diagnostic`/`DiagnosticCode` **de `domain::diagnostic`** que o `parse` emite (ADR-08, achado #48)…".

**Achado #49.** No Grupo 2, remover a frase `Expõe também getConsolidationReport(): {duplicatesRemoved, sources}…` e substituir por: `O relatório de consolidação (duplicatesRemoved, conflicts — achado #53, sources) viaja dentro do próprio Portfolio retornado — não existe função separada de consulta, porque exigiria estado residente na fronteira, exatamente o que a ADR-06 eliminou (achado #49).`

**Achados #54/#57.** No Grupo 1, atualizar assinaturas:
- `loadFile(bytes: Uint8Array, opts: {maxBytes: number}): Document` — `maxBytes` obrigatório (achado #9/#54).
- `loadFiles(files: Array<Uint8Array>, opts: {maxBytes: number}): Array<{ok: Document} | {error: FatalDiagnostic}>` — resultado posicional alinhado à ordem de entrada; falha fatal de um arquivo (caso `Err` do ADR-08) ocupa a posição dele em vez de sumir do array (achado #57), preservando "erros de um não abortam os outros" de forma observável pelo JS.

## §10 — Roadmap

**Achado #62.** Remover a numeração ordinal da lista (os dois itens "0." e o deslocamento item-vs-fase); referenciar cada item pelo nome (`Fase −1`, `Ação imediata da Fase 0`, `Fase 0`…`Fase 6`) como termo em negrito de uma lista não numerada.

**Achado #60.** Na Fase −1, substituir "(20/20 testes, …)" por "(testes verdes no fechamento; contagem vigente no sumário executivo — 24 unitários + 5 doctests após os fechamentos posteriores)".

**Achados #55/#53/#51.** Na Fase 1, adicionar aos entregáveis: `testes de paridade sgml/xml sobre pares equivalentes do corpus (achado #55)`. Na Fase 3, adicionar: `ledger de gasto sobre eventos classificados (achado #52), partição por moeda no consolidado (achado #51), FitidConflicting em dedupe/consolidation com testes atualizados (achado #53 — toca código existente), verificação de disponibilidade de CREDITLIMIT por dialeto (achado #59)`.

## §11 — Questões em aberto

**Achado #50.** Substituir o trecho `e a política na ADR-13 (a lib consolida na fronteira, porque a persona-alvo não sabe consolidar OFX no JS)` por: `e a política na ADR-13 — o mecanismo mora na lib (o consumidor não reimplementa dedupe de FITID em JS), mas o acionamento é sempre explícito por quem chama, nunca automático`.

---

# Parte 2 — achados #65–#75 (continuação da rodada)

Mesma convenção e mesma ordem por posição no documento. Dois achados tocam código existente e testado (#65 junto com o #53 já registrado; #72 muda onde uma regra vive, de construtor pra diagnostic).

## §5 — Princípios de design

**Achado #66 `[decisão do autor]`.** Nenhuma edição na regra Hexagonal — mas registrar no parágrafo de convenções que `Transaction` carrega **dois campos temporais** com papéis distintos (instante UTC + data local emitida), pra Fase 0 não colapsar os dois num `NaiveDate` só. Texto detalhado no bloco ADR-08 abaixo.

## ADR-04 — Consolidação e dedupe

**Achado #65.** Substituir a semântica de chave em `dedupe_by_fitid`/`consolidate_by_fitid`:

> **[Achado #65 fechado como decisão — toca código existente]** A chave de deduplicação é **`(AccountId, Fitid)`**, nunca `Fitid` puro: no spec OFX a unicidade de `FITID` tem escopo de conta emissora — entre contas distintas, `FITID` igual são transações distintas, jamais deduplicadas. `consolidate_by_fitid` ganha extrator `account_of` (ou consolida por conta e mescla cronologicamente depois — equivalente). Casos gêmeos obrigatórios: mesmo `FITID` em contas distintas **deve** preservar ambas; mesmo `FITID` na mesma conta entre fontes **deve** deduplicar. Sem isto, o escopo consolidado da ADR-13 subconta o gasto silenciosamente com emissores de `FITID` sequencial curto.

## ADR-05 — Analytics

**Achado #67.** No fechamento do achado #16, substituir "reamostradas para grade diária" pela versão por contexto:

> A grade de reamostragem acompanha o fator de anualização, não o contrário: série de investimento (`UNITPRICE`) → grade de **dias de pregão**, sem preencher fim de semana, coerente com `√252`; série de gasto/saldo → grade de **calendário**, coerente com `√365`. Reamostrar investimento pra grade de calendário e anualizar com `√252` deflaciona σ com dias de retorno zero — o erro simétrico ao que este achado já corrigiu. Política de preenchimento explícita (forward-fill para saldo); contagem de pontos preenchidos entra na proveniência (achado #67).

**Achado #70.** Novo parágrafo após o mapa de módulos: `Parâmetros externos pendentes de declaração (mesmo tratamento de risk_free_rate/span — achado #14): VaR exige nível de confiança e método (histórico vs. paramétrico); z-score exige janela e população (histórico global vs. por estabelecimento); MDD exige janela. Cada um vira parâmetro explícito com ExternalInput e caso gêmeo no §8 (achado #70).`

**Achado #69.** Novo parágrafo no escopo de `ledger`: `ADB e toda métrica sobre BALAMT operam sobre série de saldo reconstruída — OFX entrega snapshot(s) (LEDGERBAL@DTASOF) + transações, não série. Reconstrução: âncora no snapshot, rolando transações; âncora, direção e span entram na proveniência; segundo snapshot, quando presente, vira cross-check (alimenta a invariante do §8 e o integrity) (achado #69).`

**Achado #74.** Na linha `credit` do mapa, expandir `k_d` com a definição do handoff (ou referência explícita a ele) — símbolo de métrica sem definição no documento é lacuna de handoff.

## ADR-08 — Datas

**Achado #66 `[decisão do autor]`.** No fechamento do achado #6, adicionar:

> **[Refinado pelo achado #66]** A normalização UTC serve a ordenação e janelas entre fontes — não ao agrupamento contábil. Em UTC−3, lançamentos entre 21:00 e 23:59 locais mudam de dia (e, na virada, de mês) ao converter: `period_series` agrupando pela data UTC atribui compras ao mês errado da fatura, exatamente pra persona-alvo. `Transaction` carrega os dois: instante UTC (ordenação, janelas, dedupe) e data local emitida (agrupamento de `ledger`, mês da fatura). Quando o offset está ausente, a data local assumida é a UTC e o `DateTimezoneNormalized` já existente sinaliza. Cada função registra na proveniência qual das duas usa.

**Achado #72.** No §8 (bloco abaixo) e aqui: consistência de sinal `TRNTYPE`×`TRNAMT` **não** é invariante de construtor — emissores reais violam, e rejeitar contradiz o "maximiza dado recuperável". É diagnostic da taxonomia do #17 (normalização rastreável ou achado de atenção, a decidir por dialeto no corpus).

## ADR-09 / `web/` — Privacidade

**Achado #71.** Adicionar à ADR-09: `A demo (web/) declara Content-Security-Policy com connect-src 'none' (meta tag na página estática) — exfiltração impossível por construção no runtime do browser, mesmo se uma dependência futura introduzir fetch. Política→estrutura, o mesmo critério da ADR-11 aplicado à garantia central do projeto (achado #71).`

## ADR-13 — API de consumo

**Achado #68.** No Grupo 3, substituir "escopo definido pelo tipo do objeto recebido" por:

> Escopo definido por dois eixos explícitos: o objeto (`Document` | `Portfolio`, discriminados por campo `kind` no envelope serializado — desserialização por tentativa entre tipos parecidos falha tarde) **e** um seletor de conta (`accountId | "all"`), porque `Document` ≠ produto — um arquivo OFX pode carregar N contas (lista plana da ADR-04). O exemplo da ADR-06 já passava `account_id`; a assinatura vale para todo o Grupo 3 (achado #68).

## §8 — Testes

**Achado #72.** Mover a consistência de sinal da lista de invariantes de proptest para o padrão diagnostic: o proptest testa o **normalizador** (input com sinal violado → transação preservada + diagnostic emitido), não a rejeição.

**Achados #65/#67/#70.** Casos gêmeos novos: chave `(AccountId, Fitid)` (par preserva/deduplica do bloco ADR-04); grade por contexto (mesma série anualizada com o fator errado **deve** divergir do golden); um caso gêmeo por parâmetro novo de VaR/z-score/MDD.

## §10 — Roadmap

**Achado #73.** Parser XML 2.x aparece como entregável da Fase 0 **e** da Fase 1 — decidir a fase dona (recomendação: manter na Fase 0, que é o "caminho fácil antes do difícil" declarado) e, em consequência, mover `SECURITY.md` para a Fase 0: a superfície de input não confiável abre quando o **primeiro** parser existe, não na Fase 1.

**Achados #65/#66/#69.** Fase 0: modelo de `Transaction` com dois campos temporais (achado #66). Fase 3: reconstrução de série de saldo para ADB (achado #69). Código existente: chave de dedupe `(AccountId, Fitid)` com testes gêmeos (achado #65 — pode ser aplicado já, antes da Fase 0, por tocar `ofx-domain`).

## ADR-07 — Build

**Achado #75.** Adicionar item de verificação (não afirmação) à Fase 5: confirmar que `strip = true` do perfil não remove as custom sections que o `wasm-bindgen` consome no pipeline `cargo build → wasm-bindgen → wasm-opt`; havendo conflito, o strip migra pro `wasm-opt` no pós-build com resultado final equivalente.

---

# Parte 3 — achados #76–#85 (realismo de dialeto e produto)

Mesma convenção. Os achados #53/#65/#77 formam um **pacote único** de mudança em `dedupe`/`consolidation` (código existente) — aplicar juntos, uma passagem de testes.

## §3 — Requisitos não-funcionais

**Achado #82.** Na frase de determinismo, adicionar: `— inclusive na serialização: agregações agrupadas (por payee, por moeda) saem em coleções ordenadas (BTreeMap ou sort estável), nunca HashMap serializado, para que golden tests da saída byte a byte sejam possíveis (achado #82)`.

## ADR-02 — Parsing

**Achado #79.** No diagrama e no texto, substituir o decode fixo do caminho 2.x:

> **[Achado #79 fechado como decisão]** O caminho 2.x não presume UTF-8: honra o encoding da declaração XML via `encoding_rs` (dependência já presente no caminho 1.x), emitindo diagnostic da família "correção aplicada" quando divergir do spec (que manda UTF-8); BOM é consumido antes do sniff. Emissores reais — inclusive brasileiros — declaram ISO-8859-1 em arquivos 2.x; decode fixo produziria mojibake em payee acentuado ou falha de parse, contradizendo o ADR-08.

## ADR-04 — Domínio, dedupe e consolidação

**Achado #77.** Novo parágrafo no bloco de dedupe/consolidação (junto de #53/#65):

> **[Achado #77 fechado como decisão — toca código existente, pacote com #53/#65]** Antes de usar `FITID` como chave, cada fonte passa por avaliação de qualidade da chave: distribuição degenerada (cardinalidade muito menor que a contagem de transações, valores vazios ou constantes — "0" para tudo é defeito notório de emissor) → `DiagnosticCode::FitidUnreliable` e fallback de chave por identidade (hash de data+valor+payee) ou dedupe desativado para aquela fonte, registrado na proveniência da consolidação. Sem isto, um extrato de emissor ruim colapsa para uma transação e o `integrity` reporta o desastre como "duplicatas removidas".

**Achado #80.** Nas estruturas tipadas, adicionar: `janela declarada do extrato (DTSTART/DTEND da transaction list) — campo do modelo desde a Fase 0; é o que permite ao integrity distinguir "mês sem gasto" de "arquivo faltando" (achado #80), e entra na proveniência de qualquer métrica de período`.

**Achado #76 `[decisão do autor]`.** Nas estruturas tipadas: `Transaction` ganha campo opcional `installment: {n, of}`, preenchido pelo mapeamento de cada parser quando marcador de parcela é reconhecido em `NAME`/`MEMO` (diagnostic da família "correção aplicada"); o texto cru permanece intacto.

## ADR-05 — Analytics

**Achado #78.** Novo parágrafo após a família derivada:

> **[Achado #78 fechado como decisão]** Toda métrica declara precondições de entrada; violação retorna diagnostic tipado (`InsufficientData` | `UndefinedMetric`), nunca `NaN`, `Infinity` ou panic. Instâncias: CU com limite zero e IRR não-convergente (já fechados — viram casos da regra, não exceções); Sharpe com σ=0; z-score com n<2; CAGR com valor inicial ≤0; `runway` com burn rate ≤0; VaR/MDD com série menor que a janela. §8 ganha um caso gêmeo por precondição.

**Achado #76.** No mapa, linha `anomaly`: adicionar `— z_score opera sobre compras (parcelas da mesma compra agregadas via installment, achado #76), não sobre a distribuição de parcelas`.

## ADR-06 — Fronteira

**Achado #83.** Adicionar ao contrato: `Err da fronteira é objeto estruturado {code, message} — code do mesmo enum estável do achado #17, estendendo a proibição de string matching até a borda (achado #83); nunca string solta.`

## ADR-13 — Produto

**Achado #81.** No parágrafo de `group_by_payee`: `agrupamento opera sobre payee normalizado (caixa, prefixos de adquirente/gateway conhecidos — PAG*, MP*, PAYPAL* —, marcador de parcela do achado #76), com a estratégia de normalização na proveniência; o cru permanece na Transaction (achado #81)`.

**Achado #76 `[decisão do autor]`.** Nova linha na tabela de visões: `Compras parceladas em andamento → produto + consolidado → ledger::open_installments (agrega por compra via installment: total, pago, restante)`.

**Achado #84.** Grupo 1/2: `loadFile(bytes, {maxBytes, label?})` — `label` opcional fornecido pelo JS (a lib recebe bytes, não nomes de arquivo); é o que alimenta `SourceLabel` no relatório de consolidação e no `ExternalInput::ConsolidatedFrom`. Sem label, a fronteira gera ordinal (`arquivo-1`, …).

## §7 — Fluxo de dados

**Achado #85.** No diagrama de sequência, `participant B as bindings (wasm)` → `participant B as ofx-wasm` — alinhar ao rename do §4.

## §8 — Testes

**Achados #77/#78.** Casos gêmeos novos: fonte com `FITID` constante **deve** produzir `FitidUnreliable` + preservar todas as transações via chave de identidade (nunca colapsar para uma); um caso gêmeo por precondição do achado #78 (ex.: série constante → Sharpe **deve** retornar `UndefinedMetric`, não `Infinity`).

---

## DOD desta rodada

- [ ] Cinco decisões `[decisão do autor]` confirmadas (#48, #51, #58, #66, #76) — ou blocos correspondentes reescritos conforme a escolha divergente
- [ ] Pacote `dedupe`/`consolidation` aplicado de uma vez (#53 payload divergente + #65 chave `(AccountId, Fitid)` + #77 qualidade de FITID) — testes gêmeos dos três (idêntico vs. divergente; preserva entre contas vs. deduplica na mesma conta; FITID constante → `FitidUnreliable` sem colapso), `cargo test` e `clippy -D warnings` verdes
- [ ] Nenhuma menção remanescente a `getConsolidationReport()` como função separada
- [ ] Todas as assinaturas de `parse_ofx`/`loadFile`/`loadFiles` (Rust e JS) carregam o teto de bytes
- [ ] `grep` por "consolida na fronteira" no §11 não retorna o enquadramento antigo
- [ ] Árvore do §4 lista `consolidation` (e `diagnostic`, se #48 confirmado)
- [ ] Achado #16 reescrito com grade por contexto (pregão/`√252` vs. calendário/`√365`) e política de preenchimento na proveniência
- [ ] Consistência de sinal removida da lista de invariantes de construtor do §8 e reclassificada como diagnostic
- [ ] Parser XML 2.x com fase única dona no roadmap; `SECURITY.md` alinhado a ela
- [ ] Grupo 3 da ADR-13 com seletor de conta e discriminante `kind` no envelope
- [ ] Regra geral de precondição de métrica registrada (#78) com CU/IRR reclassificados como instâncias
- [ ] Caminho 2.x honrando encoding declarado com diagnostic (#79); modelo com janela declarada (#80) e `installment` (#76)
- [ ] Nenhuma agregação agrupada serializando `HashMap` (#82); erro da fronteira estruturado (#83); diagrama do §7 renomeado (#85)
- [ ] Sumário executivo do Discovery registra a terceira rodada e o intervalo #48–#85
