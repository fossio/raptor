# Análise crítica — Discovery/ADR ofx (terceira rodada de auditoria)

| Campo | Valor |
|---|---|
| Data | 2026-07-19 |
| Escopo | Consistência interna entre ADRs, lacunas de decisão de produto, contratos de API vs. decisões fechadas, riscos não registrados |
| Método | Leitura cruzada de todas as seções contra as decisões que elas próprias declaram vinculantes; continua a numeração das rodadas anteriores |
| Achados | #48–#85 (a rodada anterior terminou em #47; três passadas nesta rodada: #48–#64 consistência, #65–#75 semântica de cálculo, #76–#85 realismo de dialeto e produto) |
| Documento par | `alteracoes-adr.md` — plano mecânico de edição, um bloco por achado |
| **Status** | **Todos os achados #48–#85 aplicados ao Discovery** (as cinco decisões pendentes foram fechadas em entrevista posterior). Documento histórico — registro do "antes"; não editar para refletir aplicação.

## Avaliação geral

O documento está num estado raro para discovery pré-código: as decisões mais caras de errar (`Money` com moeda, cashflow classificado antes de retorno, corpus sintético, fronteira Decimal/f64) estão fechadas com justificativa e, quando possível, código testado. A disciplina de proveniência (`Metric { value, provenance }`), o padrão eval fraco→forte e o registro honesto das reversões (ADR-02, ADR-06) com o custo aceito nomeado são o ponto forte — o documento sabe o que decidiu e por quê.

O problema desta rodada não é decisão errada: é **deriva entre seções após as duas reversões da última sessão** — esse foi o diagnóstico da primeira passada (#48–#64). As duas passadas seguintes mudaram o caráter dos achados: a segunda (#65–#75) encontrou lacunas de semântica de cálculo com potencial de corromper resultado (chave de dedupe, datas, grades), e a terceira (#76–#85) encontrou ausências de realismo de dialeto e produto brasileiro. O conjunto: as decisões fechadas estão certas; o que falta é a camada fina entre elas. As reversões de ADR-02 e ADR-06 foram bem aplicadas nos próprios ADRs, mas deixaram texto obsoleto (§11), assinaturas de API que contradizem decisões fechadas (achado #9 vs. ADR-06/13) e uma função que reintroduz pela porta dos fundos o estado residente que a ADR-06 acabou de eliminar. Além disso, a ADR-13 — a mais nova — criou dois problemas estruturais que nenhuma rodada anterior podia ter visto: uma violação do grafo de dependências via `integrity` e um buraco de produto no escopo consolidado multi-moeda, que atinge exatamente a persona-alvo declarada.

---

## Achados de severidade alta

### #48 — `integrity` viola o grafo acíclico declarado no §4

**Evidência.** §4 fixa o grafo: `parse → domain`, `analytics → domain`, `wasm → {parse, analytics, domain}` — nenhuma aresta `analytics → parse`. ADR-08 fixa que `Diagnostic`/`DiagnosticCode` são "versionados junto da API pública de `ofx-parse`". ADR-13 coloca `integrity` dentro de `analytics` e declara que ele "consome os `Diagnostic`/`DiagnosticCode` que o `parse` já emite". As três afirmações são mutuamente incompatíveis: ou `analytics` passa a depender de `parse` (grafo muda), ou os tipos de diagnóstico não moram em `parse`.

**Consequência se ignorado.** O agente da Fase 3 vai adicionar `ofx-parse` como dependência de `ofx-analytics` sem perceber que está quebrando a invariante estrutural do §4 — e o acoplamento fica: qualquer mudança no parser passa a recompilar analytics, e `analytics` deixa de ser reusável sem parser (o caso FDX do §11 herda o problema).

**Correção proposta.** Mover `Diagnostic`/`DiagnosticCode` para `ofx-domain` (novo módulo `domain::diagnostic`). Justificativa de domínio, não só de grafo: diagnóstico de saúde do documento é linguagem ubíqua do problema ("esta transação foi normalizada", "este FITID duplicou") — descreve o *dado*, não o *ato de parsear*; `parse` é o produtor primário, `integrity` o consumidor, e ambos já dependem de `domain`. `parse` pode reexportar por conveniência. Alternativas registradas e rejeitadas: (b) aceitar a aresta `analytics → parse` — resolve o build, mantém o acoplamento; (c) subir `integrity` para `wasm` — mistura composição com regra de negócio no adapter, violando a própria §5. **Requer decisão do autor porque muda quem versiona o enum (ADR-08).**

### #49 — `getConsolidationReport()` reintroduz o estado residente que a ADR-06 eliminou

**Evidência.** ADR-13, Grupo 2: `getConsolidationReport(): {duplicatesRemoved, sources}` — assinatura sem parâmetro. Sem argumento de entrada, a função só pode responder consultando estado retido na fronteira desde a última chamada de `consolidate()` — exatamente o handle residente que a ADR-06 acabou de remover ("o `Document` interno vive e morre dentro de cada chamada de função").

**Consequência se ignorado.** O agente da Fase 5 implementa um `static`/`thread_local` na crate `wasm` pra guardar o último resultado de consolidação, e a fronteira volta a ter ciclo de vida implícito — pior que o handle original, porque agora é invisível (não há `free()` nem objeto pra rastrear; duas chamadas de `consolidate()` intercaladas se corrompem mutuamente).

**Correção proposta.** Eliminar a função. O relatório é campo do próprio objeto retornado: `consolidate(docs)` devolve o `ConsolidationOutcome` serializado inteiro — série consolidada **e** `{duplicates_removed, sources}` no mesmo objeto. É o que o tipo Rust já carrega (ADR-04); a API JS só estava escondendo isso atrás de uma segunda chamada stateful desnecessária.

### #50 — §11 mantém o enquadramento de consolidação que a ADR-13 declara revertido

**Evidência.** ADR-13: "Acionamento é sempre explícito, nunca automático (...) reverte o enquadramento anterior desta ADR, que descrevia a consolidação como automática na fronteira." §11: "a política na ADR-13 (a lib consolida na fronteira, porque a persona-alvo não sabe consolidar OFX no JS)" — é literalmente o enquadramento antigo, citado como se fosse a decisão vigente.

**Consequência se ignorado.** Num documento cujo consumidor primário é agente autônomo, duas seções afirmando políticas opostas sobre a mesma decisão produzem implementação aleatória — depende de qual seção o agente leu por último. É o mesmo tipo de deriva que o achado #3 (Bill Pay/Taxes) já custou uma rodada pra corrigir.

**Correção proposta.** Reescrever a frase do §11 para refletir a política real: mecanismo na lib (`consolidate_by_fitid`), acionamento sempre explícito pelo consumidor.

### #51 — Escopo consolidado é indefinido para multi-moeda — e atinge a persona-alvo

**Evidência.** ADR-13 define o escopo consolidado ("quanto gastei no total") sobre `consolidate_by_fitid` + `sum_homogeneous`. O achado #2 garante que `sum_homogeneous` retorna `Err(CurrencyMismatch)` em moedas divergentes. Nenhuma seção decide o que a visão consolidada faz nesse erro. O caso não é exótico: a persona brasileira de cartão de crédito frequentemente tem fatura com `CURDEF` distinto (cartão internacional, compra em USD lançada em moeda de origem em alguns emissores) — é o cenário-alvo declarado, não borda teórica.

**Consequência se ignorado.** A primeira vez que a persona leiga arrastar a pilha real de faturas, a visão "Total gasto" falha com um erro de moeda que ela não entende — ou pior, alguém "resolve" na Fase 3 somando sem conversão, desfazendo o achado #2 na prática.

**Correção proposta (decisão de produto — requer confirmação do autor).** Consolidação **particiona por `CurrencyCode`, nunca converte**: não existe fonte de câmbio offline, e ADR-09 proíbe buscar cotação — conversão automática é estruturalmente impossível neste projeto, o que torna a partição a única resposta coerente. Toda visão consolidada renderiza um bloco por moeda ("Total: R$ X · US$ Y"); `Portfolio` carrega as séries já particionadas; `integrity` reporta quando mais de uma moeda foi encontrada. Registrar como decisão vinculante antes da Fase 3, no mesmo espírito do achado #2 — é a metade de produto daquele achado de domínio.

### #52 — "Total gasto" de cartão somando `TRNAMT` bruto inclui o pagamento da fatura

**Evidência.** ADR-13 mapeia a visão "Total gasto no período" para `ledger::total_spent`/`ledger::net_cash_flow` sem definir filtro. No OFX de cartão de crédito, o pagamento da fatura anterior entra como transação de crédito no mesmo statement; estornos idem. `total_spent` que soma `TRNAMT` cru mostra gasto líquido do pagamento — número errado para a pergunta "quanto gastei", exatamente pra persona que não tem como perceber o erro.

**Consequência se ignorado.** É a versão contábil do achado #13: lá, retorno consumindo `TRNAMT` bruto misturava aporte com performance; aqui, gasto consumindo `TRNAMT` bruto mistura compra com pagamento. O documento já tem a peça que resolve (`cashflow::ClassifiedEvent` distingue naturezas de lançamento) e não a conectou ao caminho principal da persona.

**Correção proposta.** Decisão vinculante: `ledger::total_spent` (e `extremes`, `average_ticket`, `group_by_payee`) consome eventos classificados ou filtra por `TRNTYPE` explícito (compra/estorno/pagamento), nunca `TRNAMT` bruto; a classificação usada entra na proveniência. `net_cash_flow` permanece como métrica distinta (o líquido é uma pergunta legítima — só não é "total gasto"). A tabela de visões separa as duas em linhas próprias pra ninguém tratá-las como sinônimos.

### #53 — Dedupe assume que `FITID` repetido é cópia idêntica; colisão com payload divergente é engolida em silêncio

**Evidência.** ADR-04: `dedupe_by_fitid`/`consolidate_by_fitid` mantêm a primeira ocorrência e reportam `DuplicateFitid`. Nada compara o payload das ocorrências. Dois cenários reais divergem: banco reemite transação corrigida com o mesmo `FITID` e valor diferente; e emissores ruins geram `FITID` não-únicos entre produtos. Nos dois, first-wins descarta silenciosamente um registro **diferente**, não uma cópia.

**Consequência se ignorado.** O `integrity` — cujo trabalho é dizer "os dados são confiáveis?" — reporta "2 duplicatas removidas" quando o que aconteceu foi "2 transações conflitantes, uma escolhida arbitrariamente". A persona confia num número que pode estar errado, com o selo de saúde dizendo que está tudo bem.

**Correção proposta.** Comparar payload no dedupe: idêntico → `DuplicateFitid` como hoje; divergente → novo `DiagnosticCode::FitidConflicting` (família "exigem atenção", nomenclatura `Substantivo+Particípio/Adjetivo` do achado #17), mantendo a precedência cronológica como escolha mas reportando o conflito. `integrity` traduz pro leigo ("uma transação aparece com valores diferentes em duas faturas — verifique qual é a correta"). Muda `dedupe`/`consolidation`, que já existem em código — é o único achado desta rodada que toca código testado.

### #54 — `max_bytes` obrigatório (achado #9) não aparece em nenhuma assinatura de API

**Evidência.** Achado #9 fechou: "`max_bytes` é obrigatório na chamada, sem default fixo pela lib". ADR-06 mostra `parse_ofx(bytes: &[u8])`; ADR-13 mostra `loadFile(bytes)`, `loadFiles(files)`. Nenhuma carrega o parâmetro.

**Consequência se ignorado.** Código de exemplo em ADR é o que o agente copia. A decisão do achado #9 se perde na Fase 5 exatamente onde deveria se materializar — e ninguém nota, porque as assinaturas "batem com o documento".

**Correção proposta.** Atualizar todas as assinaturas e exemplos (Rust e JS) para `parse_ofx(bytes, max_bytes)` / `loadFile(bytes, {maxBytes})` / `loadFiles(files, {maxBytes})`. Edição mecânica; sem decisão nova.

---

## Achados de severidade média

### #55 — Duplicação de mapeamento (ADR-02) aceita sem mecanismo contra deriva

A reversão para dois parsers independentes aceitou conscientemente duplicar o mapeamento das tags comuns e as normalizações dos achados #6/#18 — mas não criou nada que detecte quando as duas cópias divergirem. Deriva silenciosa entre `sgml::to_document` e `xml::to_document` é o modo de falha natural dessa decisão: o mesmo conteúdo semântico passa a produzir `Document` diferente dependendo da versão do arquivo, e nenhum teste unitário de um parser pega isso. Mitigação barata que recupera a rede de segurança do AST neutro sem o acoplamento: **testes de paridade** — o gerador de corpus (já programático, achado #4) emite pares equivalentes do mesmo conteúdo em forma 1.x e 2.x, e um teste afirma `sgml::to_document(a) == xml::to_document(b)` (módulo diagnostics específicos de formato). Entra em §8 como categoria própria e no roadmap da Fase 1.

### #56 — Representação de `Decimal` na fronteira: promover de "verificar na Fase 5" a decisão vinculante

ADR-06 registra a ida-e-volta `Serialize`+`Deserialize` como "primeira coisa a verificar na Fase 5". O risco concreto tem nome: se `Decimal` atravessar a fronteira como número JS (feature `serde-float` do `rust_decimal`, ou conversão manual via `f64`), a perda de precisão desfaz o ADR-03 silenciosamente — o domínio é exato, mas o round-trip não. O custo de decidir agora é zero: **`Decimal` cruza como string, nunca número JS; `serde-float` proibida no workspace**; eval de round-trip `Document → JsValue → Document` com valores que não têm representação exata em `f64` (ex.: `0.1 + 0.2` clássico) que **deve** preservar igualdade exata. Mesma lógica que fechou o achado #41: feature de conveniência que compila normal e corrompe o invariante.

### #57 — `loadFiles(): Array<Document>` não tem onde reportar falha fatal por arquivo

ADR-13 promete "erros de um não abortam os outros", mas o tipo de retorno não tem slot pra falha fatal (header ilegível — o caso `Err` do ADR-08). Ou a assinatura vira `Array<{ok: Document} | {error: ...}>` (posicional, alinhado à ordem de entrada), ou a promessa de resiliência não é observável pelo JS — o arquivo que falhou some do array e o índice desalinha da lista de arquivos do usuário.

### #58 — LGPL-2.1 + linkagem estática Rust: a história de conformidade precisa ficar registrada

A escolha de copyleft é deliberada e não é o ponto desta análise. O ponto é que LGPL foi desenhada pra linkagem dinâmica, e o ecossistema Rust linka estático: quem consumir `ofx-domain` via Cargo embute a crate no próprio binário, e a §6 da LGPL-2.1 exige que esse distribuidor permita relink com versão modificada da lib (fornecer objetos ou fonte) — atrito conhecido que afasta consumidores corporativos de crates LGPL, às vezes sem o autor perceber que era esse o motivo. O caminho `.wasm` é tranquilo: o módulo é carregado dinamicamente pelo host JS, permanece arquivo separável e trocável — relink é trocar o arquivo. Duas saídas, ambas compatíveis com a prioridade de copyleft: (a) manter LGPL-2.1-or-later e registrar a interpretação no repositório (README/LICENSE-notes: o `.wasm` é a fronteira de biblioteca; consumo via Cargo aciona a §6 e o que o projeto considera conformidade suficiente); (b) MPL-2.0 — copyleft por arquivo, modificações à lib continuam obrigatoriamente abertas, sem atrito de linkagem estática, e é a escolha comum de libs Rust com exatamente esse objetivo. **Decisão permanece do autor** — o que falta no documento é o registro do raciocínio, antes de publicar no crates.io, não a mudança em si.

### #59 — CU pressupõe `CREDITLIMIT` disponível; a presença no OFX real varia por dialeto

O eval do §8 fixa `CREDITLIMIT` e `BALAMT` conhecidos. Em statements de cartão reais, a disponibilidade do limite varia por emissor — vários só entregam `LEDGERBAL`/`AVAILBAL`, e nesse caso o limite só existe por derivação (`ledger + avail`). Verificar contra os dialetos do corpus antes da Fase 3 e decidir o comportamento sem o dado: CU indisponível com diagnostic (preferível — não inventa número), ou derivação explícita com `ExternalInput` marcando que o limite foi inferido, nunca valor derivado apresentado como declarado. O eval forte atual só cobre o caminho feliz; falta o caso gêmeo "limite ausente".

---

## Achados de severidade baixa (editorial/consistência)

### #60 — Contagens de teste divergentes no mesmo documento
Sumário executivo: "24 testes unitários + 5 doctests". Roadmap, Fase −1: "20/20 testes". Se são fotografias de momentos diferentes, o roadmap deveria dizer isso; documento de handoff não pode ter dois números pro mesmo fato sem qualificação.

### #61 — Layout do §4 não lista `consolidation.rs`
ADR-04 declara `ofx-domain::consolidation` "implementado e testado"; a árvore do §4 mostra `src/{lib,money,metric,cashflow,predictive,dedupe}.rs` — sem `consolidation`. A árvore é o mapa que o próximo agente usa; mapa desatualizado sobre código que existe é pior que sobre código planejado.

### #62 — Numeração do roadmap quebrada
Dois itens "0." consecutivos e índice ordinal deslocado do nome da fase (item "1." = Fase 0, item "2." = Fase 1…). Pra consumo por agente, referenciar por nome de fase e abandonar a numeração ordinal da lista elimina a classe de erro.

### #63 — `Portfolio` (ADR-13) vs `ConsolidationOutcome` (ADR-04): relação não declarada
São o mesmo objeto? A leitura mais natural é `Portfolio` = `ConsolidationOutcome` serializado — mas isso está implícito, e implícito num contrato de API é o leitor adivinhando. Uma frase resolve (e, com o achado #49, o `Portfolio` passa a carregar o relatório de consolidação, o que torna a equivalência ainda mais importante de declarar).

### #64 — Assinatura de `runway` divergente no rascunho de CLAUDE.md
ADR-05: `fn(Money, &Metric<Money>) -> Metric<Decimal>`. Rascunho de CLAUDE.md (ADR-11): `fn(Money, &Metric<V>)`. O CLAUDE.md é precisamente o artefato que o agente lê primeiro; a assinatura genérica ali enfraquece a invariante que ele existe pra proteger.

---

## Continuação da rodada — achados #65–#75

A segunda passada foca no que a primeira não cobriu: semântica dos cálculos (chaves, datas, grades, parâmetros) em vez de consistência entre seções. Três achados são de severidade alta e dois tocam código existente e testado.

### Achados de severidade alta

#### #65 — `FITID` só é único por conta: consolidar produtos diferentes por `FITID` puro funde transações distintas

**Evidência.** No spec OFX, a unicidade de `FITID` tem escopo de **conta emissora**, não global — é um ID de detecção de duplicata dentro do mesmo produto. `consolidate_by_fitid` (ADR-04) recebe apenas extratores `fitid_of` e `date_of` — nenhuma noção de conta — e a ADR-13 constrói o escopo consolidado ("todos os cartões juntos") exatamente sobre ele. Emissores com `FITID` sequencial curto ("1", "2", "3"…) entre dois cartões do mesmo banco colidem em massa.

**Consequência se ignorado.** Transações reais de cartões diferentes são "deduplicadas" entre si: a visão consolidada subconta o gasto, e o `integrity` reporta as remoções como duplicatas legítimas — erro silencioso com selo de saúde no número principal da persona. É o pior modo de falha possível pro produto declarado.

**Correção proposta.** A chave de dedupe é `(AccountId, Fitid)`: duplicata só existe dentro da mesma conta (reimportação sobreposta, o caso do achado #10); entre contas, nada se deduplica jamais. `consolidate_by_fitid` ganha extrator `account_of` — ou, equivalente, consolida por conta e mescla cronologicamente depois. **Toca código existente e testado** (`dedupe`/`consolidation`): teste gêmeo obrigatório — mesmo `FITID` em contas distintas **deve** preservar ambas; mesmo `FITID` na mesma conta entre fontes **deve** deduplicar.

#### #66 — Normalização UTC + `NaiveDate` desloca a data de compras noturnas — agrupamento mensal errado em UTC−3

**Evidência.** Achado #6 normaliza toda data para UTC; achado #41 fixa `domain` em `NaiveDate`. `DTPOSTED` carrega hora: em UTC−3, qualquer lançamento entre 21:00 e 23:59 locais muda de dia calendário na conversão — e na virada do mês, muda de mês.

**Consequência se ignorado.** `ledger::period_series` atribui compras noturnas ao mês seguinte; a "evolução da fatura" diverge da fatura que a persona tem na mão, sem nenhum diagnostic. O produto parece errado justamente na visão principal, para o fuso do público-alvo declarado.

**Correção proposta (requer decisão do autor — refina o fechamento do achado #6 e muda o modelo de `Transaction` da Fase 0).** Separar os dois usos da data: **ordenação e janelas** entre fontes usam o instante UTC (o que o #6 quer garantir); **agrupamento contábil** (`period_series`, mês da fatura) usa a data local emitida, preservada no domínio ao lado do instante normalizado. A escolha de qual data cada função usa entra na proveniência.

#### #67 — Grade de reamostragem em conflito com o fator de anualização

**Evidência.** O achado #16 manda reamostrar `BALAMT`/`UNITPRICE` para "grade diária" antes de qualquer métrica de janela — e, no mesmo parágrafo, justifica `√252` porque a série de investimento "só tem pontos em dia útil de bolsa". Uma série reamostrada para grade diária de **calendário** tem 365 pontos/ano com fins de semana chapados: dias de retorno zero deflacionam σ, e `√252` sobre essa grade é fator sistematicamente errado — o mesmo tipo de erro que o próprio #16 corrigiu no sentido inverso.

**Correção proposta.** "Grade diária" é por contexto, como o fator já é: série de investimento → grade de **dias de pregão** (sem preencher fim de semana), coerente com `√252`; série de gasto/saldo → grade de **calendário**, coerente com `√365`. Política de preenchimento explícita (forward-fill para saldo — saldo persiste entre lançamentos), com contagem de pontos preenchidos na proveniência.

### Achados de severidade média

#### #68 — `Document` ≠ produto: um arquivo OFX pode carregar N contas
ADR-04 modela lista plana de contas por documento; ADR-13 Grupo 3 define escopo "pelo tipo do objeto recebido" (`Document` = por-produto). Um `Document` multi-conta quebra a equivalência — e o exemplo da ADR-06 já passa `account_id`, que a ADR-13 dropou. Correção: funções do Grupo 3 recebem seletor de conta explícito (`accountId | "all"`), e o envelope serializado ganha campo discriminante `kind: "document" | "portfolio"` — desserialização por tentativa entre dois tipos estruturalmente parecidos é frágil e falha tarde.

#### #69 — ADB exige reconstrução da série de saldo — decisão ausente
OFX entrega transações + snapshot(s) de saldo (`LEDGERBAL`@`DTASOF`), não série diária. ADB — e qualquer métrica sobre `BALAMT` — precisa reconstruir a série: âncora no snapshot, rolando as transações (em qual direção, sobre qual span). Nada decide isso. Quando existir segundo snapshot, a reconstrução vira cross-check natural (alimenta a invariante do §8 e o `integrity`). Âncora, direção e span reconstruído entram na proveniência.

#### #70 — VaR, z-score e MDD com parâmetros externos não declarados
O documento já exige (achado #14) parâmetro externo em `ExternalInput` e eval gêmeo provando que o parâmetro entra no cálculo (`risk_free_rate`, span da EMA). VaR não tem nível de confiança nem método (histórico vs. paramétrico) decididos; z-score não tem janela/população (histórico global vs. por estabelecimento) nem tratamento da assimetria típica de série de gasto; MDD não tem janela. Mesmo tratamento dos pares já fechados: parâmetro explícito, proveniência, caso gêmeo por parâmetro.

#### #71 — CSP como enforcement do ADR-09 na camada do browser
"Sem `fetch` no `app.js` não há caminho de exfiltração" é política, não estrutura — qualquer dependência da demo pode introduzir um `fetch` sem ninguém notar. `Content-Security-Policy: connect-src 'none'` (meta tag na demo estática) torna exfiltração impossível por construção no runtime do browser. É o mesmo movimento política→estrutura que o documento usa como critério na ADR-11 (instrução vs. garantia), aplicado à sua garantia mais importante.

#### #72 — Consistência de sinal `TRNTYPE`×`TRNAMT`: invariante de construtor conflita com a tolerância a dialetos
§8 lista a consistência de sinal como invariante de proptest do domínio; ADR-08 promete tolerância a arquivos malformados. Emissores reais violam sinal com frequência — se o construtor rejeitar, arquivos de dialetos ruins perdem transações válidas, contradizendo o "maximiza dado recuperável". Correção: é `Diagnostic` (normalização rastreável ou achado de atenção, taxonomia do #17), não invariante de construtor; o proptest testa o normalizador, não a rejeição.

### Achados de severidade baixa

#### #73 — Parser XML 2.x listado como entregável da Fase 0 **e** da Fase 1
E a premissa do `SECURITY.md` ("superfície de ataque abre na Fase 1") conflita com XML entregue na Fase 0 — input não confiável começa quando o primeiro parser existe. Decidir a fase dona do parser XML e alinhar o `SECURITY.md` a ela.

#### #74 — `k_d` sem definição em nenhuma seção
Aparece no mapa de módulos (`credit`) e em lugar nenhum mais. Para um documento de handoff, um símbolo de métrica opaco é lacuna — definir no próprio documento ou referenciar a definição do handoff das 20 operações.

#### #75 — Verificação para a Fase 5: `strip = true` × custom sections do `wasm-bindgen`
O perfil do ADR-07 liga `strip = true`; o pipeline é `cargo build → wasm-bindgen → wasm-opt`, e o `wasm-bindgen` consome custom sections do `.wasm` intermediário. Registrar como **item de verificação** (não afirmação): confirmar que o strip do perfil não remove as sections que o `wasm-bindgen` precisa; se houver conflito, o strip migra para o `wasm-opt` no pós-build, com resultado final equivalente.

---

## Terceira passada — achados #76–#85

Foco novo: realismo de dialeto e de produto — o que arquivos e hábitos de consumo brasileiros reais fazem com as decisões já fechadas. É também a passada em que declaro **saturação**: depois dela, o valor marginal de continuar auditando o documento é menor que o de aplicar as alterações acumuladas e executar a separação estrutural já sugerida ao final.

### Achados de severidade alta

#### #76 — Parcelamento não existe no documento — e é a realidade dominante do cartão brasileiro

**Evidência.** O documento reconhece o emissor brasileiro quando conveniente (vírgula decimal, achado #18) e declara persona brasileira de cartão de crédito — mas nenhuma seção menciona parcelamento. No OFX real de fatura, cada parcela é uma transação própria, tipicamente com marcador em `NAME`/`MEMO` ("PARC 02/10", "3/12").

**Consequência se ignorado.** Três visões da ADR-13 degradam: `group_by_payee` fragmenta o mesmo estabelecimento por parcela; `anomaly::z_score` opera sobre a distribuição de *parcelas*, não de *compras* — a compra à vista equivalente vira anomalia e o parcelão de verdade passa despercebido diluído; e a pergunta que a persona de fato tem — "quanto ainda vou pagar do que já parcelei" — não tem visão nem função planejada em lugar nenhum.

**Correção proposta `[decisão do autor]`.** Mínimo: detectar marcador de parcela no mapeamento de cada parser (campo estruturado `installment: {n, of}` em `Transaction` quando reconhecido, com diagnostic da família "correção aplicada") e normalizar payee removendo o marcador (achado #81). Produto: decidir se "compras parceladas em andamento" entra como visão da ADR-13 — recomendo que sim; é a segunda pergunta mais comum da persona depois de "quanto gastei". O achado **não** muda o #52: total gasto por fatura permanece correto (a parcela do mês é o que bateu na fatura) — complementa, não conflita.

#### #77 — `FITID` degenerado colapsa o extrato: a qualidade da chave precisa ser avaliada antes do dedupe

**Evidência.** Emissores reais produzem `FITID` vazio, constante ("0" para todas as transações) ou reciclado — classe de defeito notória no ecossistema OFX. O invariante do ADR-04 ("transação exige `FITID`") somado ao dedupe first-wins transforma isso em: `FITID` constante → extrato inteiro deduplicado para uma transação; `FITID` vazio → ou descarte em massa pelo construtor, ou colisão total na chave vazia.

**Consequência se ignorado.** Perda silenciosa de quase todos os dados de uma fonte, reportada pelo `integrity` como "N duplicatas removidas" — o mesmo modo de falha do achado #65, entrando por outra porta.

**Correção proposta.** Avaliação de qualidade do `FITID` por fonte **antes** de usá-lo como chave: distribuição degenerada (cardinalidade ≪ contagem de transações, vazios) → `DiagnosticCode::FitidUnreliable` + fallback de chave por identidade (data+valor+payee, hash) ou dedupe desligado para aquela fonte, registrado na proveniência da consolidação. Toca `dedupe`/`consolidation` existentes — com #53 e #65, os três formam um pacote único de mudança no código já testado.

### Achados de severidade média

#### #78 — Indefinição de métrica tratada caso a caso, sem regra geral
CU com limite zero e IRR não-convergente têm tratamento; Sharpe com σ=0, z-score com n<2, CAGR com valor inicial ≤0, `runway` com burn rate ≤0, VaR/MDD sobre série curta demais — não. A regra que falta: toda métrica declara precondições de entrada; violação retorna diagnostic tipado (`InsufficientData` | `UndefinedMetric`), nunca `NaN`, `Infinity` ou panic; §8 ganha caso gêmeo por precondição. Os dois casos já fechados viram instâncias da regra, não exceções.

#### #79 — Decode fixo "2.x → UTF-8" contradiz a tolerância a dialetos
No diagrama da ADR-02, o caminho 2.x decodifica UTF-8 incondicionalmente. O spec manda UTF-8, mas emissores reais (inclusive brasileiros) declaram ISO-8859-1 na declaração XML — decode fixo produz mojibake em payee acentuado ou falha o parse. Honrar o encoding declarado via `encoding_rs` (dependência já presente no caminho 1.x), com diagnostic quando divergir do spec; tratar BOM. Mesma postura do resto do ADR-08: corrigir e rastrear, não presumir conformidade.

#### #80 — Domínio não captura a janela declarada do extrato (`DTSTART`/`DTEND`)
`integrity` promete detectar "gaps temporais suspeitos entre faturas" — impossível distinguir "mês sem gasto" de "arquivo faltando" sem a cobertura que o próprio arquivo declara (`DTSTART`/`DTEND` da transaction list). ADR-04 lista conta/transação/saldo/posição; a janela declarada não aparece. Campo novo no modelo (Fase 0), consumido por `integrity` e pela proveniência de qualquer métrica de período.

#### #81 — `group_by_payee` sobre `NAME`/`MEMO` cru fragmenta estabelecimentos
Strings reais de fatura carregam prefixo de adquirente/gateway (`PAG*`, `MP*`, `PAYPAL*`), sufixo de parcela (achado #76) e variação de caixa. Agrupar pelo cru transforma "gasto por estabelecimento" — visão da tabela da ADR-13 — em lista de fragmentos. Normalização como etapa explícita e declarada (caixa, prefixos conhecidos, marcador de parcela), com a estratégia na proveniência do agrupamento; o dado cru permanece intacto na `Transaction`.

#### #82 — Determinismo do §3 vs. ordem de iteração de `HashMap`
"Mesmo input produz mesmo output" quebra byte a byte se qualquer agregação agrupada (`group_by_payee`, partição por moeda do achado #51) serializar um `HashMap`: a ordem de iteração muda por processo. Saída serializada usa coleção ordenada (`BTreeMap` ou sort estável na borda) — é o que viabiliza golden tests da serialização e o que o requisito do §3 já exigia sem dizer.

### Achados de severidade baixa

#### #83 — Contrato de erro da fronteira sem forma
`Err(JsValue)` nas assinaturas da ADR-06 — string? objeto? A disciplina do achado #17 (código estável, nunca string matching) para na borda: erro da fronteira vira objeto estruturado `{code, message}`, com `code` do mesmo enum estável.

#### #84 — `SourceLabel`: origem não declarada
A lib recebe bytes, não nomes de arquivo — o rótulo de fonte da consolidação só pode vir do JS, e nenhuma assinatura do Grupo 1/2 o carrega. Sem isso, o relatório de consolidação e o `ExternalInput::ConsolidatedFrom` não têm nome útil para exibir.

#### #85 — Diagrama do §7 ainda nomeia a fronteira como "bindings"
`participant B as bindings (wasm)` — a decisão do §4 renomeou a crate; o diagrama de sequência ficou com o nome antigo. Trocar para `ofx-wasm`.

---

## Observação estrutural (fora da numeração — sobre o documento, não sobre a arquitetura)

O documento acumulou quatro camadas no mesmo arquivo: registro de decisão (ADRs), log de auditoria (histórico de achados dentro de cada ADR), roadmap e spec de produto (ADR-13). A ADR-10 já prevê a separação `ARCHITECTURE.md` + `docs/adr/` "quando o projeto for público" — esta rodada sugere que o gatilho certo é antes: metade dos achados #48–#54 são deriva entre seções, e deriva cresce com o quadrado do tamanho do arquivo único. O formato de cada ADR carregar a própria narrativa de reversões ("revisado outra vez — decisão trocada de…") é excelente proveniência e péssima economia de tokens pro agente que só precisa do estado vigente — na separação, cada ADR ganha texto limpo do estado atual + seção `Histórico` compacta, e o log de achados vira arquivo próprio. Não é bloqueante; é o mesmo critério de economia de tokens da ADR-11 aplicado ao próprio Discovery.
