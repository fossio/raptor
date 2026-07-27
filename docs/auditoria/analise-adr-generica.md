# Análise conceitual: DDD, SOLID, Hexagonal e correlatos aplicados ao Discovery OFX

Correção de duas premissas da versão anterior desta análise. Primeiro fato: **há I/O local** — o WASM+JS pode falar com `localhost` (leitura de arquivo via File API, `fetch` para serviço local, `IndexedDB`), o que é diferente de "sem I/O". A restrição real é *nenhum upload para servidor remoto*, não *nenhum I/O*. Isso muda a avaliação dos Secondary Ports: eles têm propósito. Segundo, e mais importante: os anexos não são um menu de itens para adotar ou rejeitar. São um conjunto de **lentes conceituais** — DDD, SOLID, Hexagonal, ROP, CQS, os padrões GoF — e o trabalho útil é usar cada lente para avaliar o que já construímos: onde o Discovery está aderente, onde está implícito e deveria ser explícito, e onde viola o princípio sem ter percebido. A análise abaixo é por conceito, não por parágrafo do anexo.

---

## 1. Hexagonal (Ports & Adapters)

A lente certa, e o I/O local é o que a torna aplicável de verdade. O Discovery já é hexagonal na estrutura sem usar o vocabulário: `bindings` é Primary Adapter, `domain`/`parse`/`analytics` é o core, a fronteira `&[u8]`/`serde-wasm-bindgen` é o contrato de borda. O que a lente revela e o Discovery ainda não nomeia:

O core hoje recebe `&[u8]` já lido — a leitura acontece no JS (`file.arrayBuffer()`) antes de cruzar a fronteira. Isso funciona para "abrir um arquivo", mas o momento em que I/O local deixa de ser trivial é quando aparecer: cache de resultados em `IndexedDB`, leitura de múltiplos arquivos para a consolidação multi-arquivo (a questão em aberto §10 do Discovery — `w_i`/Sharpe de portfólio precisam de série consolidada), ou um relógio para métricas com janela "últimos 12 meses" (DY usa `INVTRAN` dos últimos 12 meses — de onde vem "agora"?). Cada um desses é um **Secondary Port** que o domínio deveria declarar como trait e a borda implementar: `TimeProvider` (o WASM lê `Date.now()`, um teste injeta relógio fixo), `PriceSeriesProvider` para a consolidação, eventualmente um `ResultCache`.

O valor de nomear isso agora não é escrever os ports hoje — é registrar no ADR-01 que a arquitetura é hexagonal e que **capacidade externa do core entra como trait, não como chamada direta**. Sem essa regra explícita, a implementação de DY na Fase 4 provavelmente pega o tempo atual via chamada direta a uma API de plataforma dentro de `analytics`, contaminando o core com I/O — exatamente o que a auditoria vem prevenindo em outras dimensões. A lente hexagonal transforma "não faça I/O no core" (regra negativa, fácil de violar sem perceber) em "capacidade externa = Port injetado" (regra positiva, verificável).

Ponto onde o Discovery já acerta e a lente confirma: a decisão de `bindings` receber `&[u8]` em vez de o core abrir o arquivo É a inversão hexagonal correta — a borda faz o I/O, o core recebe o dado. Só falta generalizar de "bytes de entrada" para "toda capacidade externa".

## 2. DDD (Domain-Driven Design)

Lente que os anexos citam de leve (Aggregates, Value Objects, Domain Errors) mas que se aplica com força, porque OFX É um domínio com invariantes reais. Onde o Discovery já está aderente:

**Value Objects** — `Money`, `CurrencyCode`, `Fitid` são VOs canônicos: imutáveis, validados na construção, identidade por valor. `Money` recusar `Add` e exigir `try_add` é a defesa de invariante de VO no ponto certo.

**Domain Errors tipados** — `CurrencyMismatch`, `RunwayError`, o `DiagnosticCode` planejado — são erros de domínio, não de infraestrutura. Aderente.

Onde a lente DDD revela vocabulário faltando:

**Aggregate e Aggregate Root** não estão nomeados. O que o Discovery chama de `Document` é, em DDD, o Aggregate Root — a raiz de consistência que garante que contas, transações e posições formam um todo coerente (a invariante `saldo_inicial + Σ transações == saldo_final` do §7 é invariante de agregado, não de transação isolada). Nomear `Document` como Aggregate Root e as invariantes de balanço como invariantes de agregado dá um lugar conceitual para elas viverem — hoje a do §7 está solta na estratégia de testes, quando é uma regra de domínio.

**Classificação de cashflow (achado #13) é um Domain Service.** Não é dado (não pertence a uma transação isolada) nem infraestrutura. É conhecimento de negócio que opera sobre VOs — a definição de Domain Service. Está implementado como funções livres em `cashflow.rs`, o que é fine em Rust, mas reconhecê-lo como Domain Service explica *por que* mora no domínio e não em analytics: é regra sobre o significado de um lançamento, não cálculo estatístico.

**Ubiquitous Language** — o Discovery já usa os termos do spec OFX (`TRNAMT`, `INVPOS`, Message Set) em vez de inventar sinônimos. Aderente, e vale registrar como decisão consciente: os nomes vêm do spec, não de abstrações próprias.

## 3. SOLID

Lente por lente, contra o código real de `ofx-domain`:

**SRP** — aderente. `money`/`metric`/`cashflow`/`predictive`/`dedupe` têm cada um uma razão de mudar. A separação `parse`/`analytics`/`domain` é SRP em escala de crate.

**OCP** — parcialmente explícito. O AST neutro (ADR-02) é OCP puro: adicionar front-end de formato (FDX futuro) não altera o mapeador nem o domínio. Mas o Discovery não nomeia isso como OCP — e deveria, porque é o argumento mais forte a favor da decisão (C) do ADR-02 sobre as alternativas (A) e (B).

**LSP** — relevante num ponto não óbvio: as duas famílias de assinatura de métrica (primária `fn(&[T]) -> Metric<V>` e derivada `fn(Metric) -> Metric`, achado #7). LSP diz para não forçar um subtipo num contrato que ele não satisfaz — é exatamente o argumento de por que Runway não podia ser espremida na assinatura primária. O achado #7 é, na prática, uma violação de LSP que a auditoria pegou sem usar o nome. Registrar o nome fortalece a decisão.

**ISP** — é o argumento que falta para os Secondary Ports da seção 1. Quando `TimeProvider`/`PriceSeriesProvider` entrarem, ISP manda que sejam traits pequenas e separadas, não uma `Environment` monolítica. Vale deixar a regra pronta antes de a primeira trait de port nascer, senão a tentação é uma trait `Platform` com tudo dentro.

**DIP** — o cerne. O core depende de abstrações (as traits de Port), as bordas dependem do core. O Discovery já faz isso com o dado de entrada; a seção 1 generaliza para toda capacidade externa. A regra dos anexos que vale adotar literalmente: **proibido `#[cfg(target_arch)]` no core para trocar comportamento**. Hoje o Discovery não diz isso, e é armadilha real — a tentação de `#[cfg(wasm)]` dentro de `analytics` para o relógio quebraria DIP e a testabilidade. A injeção tem que ser na borda (`bindings`/CLI monta o serviço com o adapter concreto), nunca via cfg no core.

## 4. Railway Oriented Programming (ROP) e CQS

**ROP** — já é o estilo do código (`Result` + `?` + combinadores em `sum_homogeneous`, `try_add`, `runway`). Sem seção nova; vale uma linha registrando que o encadeamento de `Result` é o padrão de fluxo de erro do domínio, já materializado.

**CQS** — a lente revela algo sobre `analytics`. Toda métrica é query pura (sem efeito colateral, retorna `Metric`), CQS bem-comportado. O ponto de atenção é quando cache/persistência local entrar (seção 1): surgem comandos (escrever no cache) que precisam ficar separados das queries (computar métrica). Registrar CQS agora previne o anti-padrão de uma função que computa E persiste no mesmo passo. Ainda não é problema, é barato deixar a regra pronta.

## 5. Padrões GoF que se aplicam de verdade

Filtrando os que os anexos listam por reflexo contra os que têm uso real:

**Newtype** — já é o padrão (`Money`, `CurrencyCode`, `Fitid`), validação no construtor. A regra `From` infalível / `TryFrom` falível vale como convenção — é a única coisa concreta da análise anterior que sobrevive à releitura.

**Strategy** — aplicável real: os `Formatter` de saída (JSON para JS, futura tabela para CLI) e, mais interessante, a política de reamostragem do achado #16 (grade diária vs mensal) é uma Strategy — o algoritmo de anualização troca conforme a grade. Nomear ajuda a Fase 4.

**Facade** — `bindings::OfxDocument` já é Facade sobre `parse`+`analytics`+`domain`. Aderente e implícito; o handle opaco É o padrão Facade aplicado.

**Decorator** — o `CachedReader` do anexo vira, no nosso caso, decorator sobre o futuro `PriceSeriesProvider` ou sobre parsing repetido. Especulativo demais para registrar agora; anotar como possibilidade quando cache existir.

Os demais (Abstract Factory por `#[cfg]`, Observer assíncrono) caem junto com a maquinaria que não se aplica — Observer só faria sentido para progresso de operação longa, e nenhuma métrica sobre um extrato é longa o bastante para justificar callback de progresso.

## 6. Async — a correção sobre a versão anterior

I/O local existe e no browser é assíncrono por natureza (File API, `fetch`, `IndexedDB` são todos async no JS). Então quando um Secondary Port de I/O local nascer, sua **implementação WASM será async**. A correção sobre a análise anterior: async não é proibido nem cerimônia — ele pertence ao *adapter*. O core define o Port podendo ser `async` (via `async-trait` ou AFIT quando estável) sem se acoplar a nenhuma runtime; a runtime é escolha da borda (`wasm-bindgen-futures` no WASM, o que a CLI quiser). Isso é exatamente o tratamento de async da versão revisada do anexo, e esse se aplica.

O que muda em relação ao que eu disse antes: o core continua **síncrono para computação pura** (parsing e as 20 métricas não esperam nada — são CPU), mas os Ports de I/O local que a seção 1 identifica são async, e o núcleo não deve assumir `Send + Sync` nas Futures desses ports, porque o executor WASM é single-thread (`spawn_local`). Assumir `Send` seria a falsa expectativa de concorrência que a versão revisada do anexo alerta corretamente.

## 7. O que muda no Discovery

Quase nada dos anexos entra como *mecanismo novo*. O que entra é **vocabulário conceitual que nomeia decisões já tomadas**, mais regras positivas que fecham armadilhas ainda abertas:

1. Declarar a arquitetura como Hexagonal no ADR-01, com a regra "capacidade externa do core = Secondary Port injetado na borda, nunca chamada direta nem `#[cfg]` no core". Fecha a armadilha do relógio/DY na Fase 4 (DIP + ISP + Hexagonal convergem aqui). Os Ports async nascem quando a capacidade for necessária; o core define contrato, a borda escolhe runtime.
2. Nomear `Document` como Aggregate Root e as invariantes de balanço como invariantes de agregado (DDD). Dá lugar às regras do §7 que hoje flutuam na estratégia de testes.
3. Nomear o achado #7 como violação de LSP evitada, e o AST neutro (ADR-02) como OCP. O nome fortalece decisões que hoje se defendem por intuição.
4. Registrar como convenção de código: Newtype com `From`/`TryFrom` conforme falibilidade; derives com ressalva de `f64` (não deriva `Eq`); doctests na API pública do `domain`; proibição de `println!`/`#[cfg(target_arch)]` no core.

Isso é adição de uma seção curta "Princípios de design" antes das ADRs (ou expansão do §3 requisitos), mais notas em ADR-01/02/04. Nenhuma reescrita, nenhum código novo agora.

Continua rejeitado por colisão factual, independente do nível: `wee_alloc` (ADR-07 já o descartou por estar sem manutenção — fato sobre a lib, não questão conceitual).

## Decisão pendente antes de editar

Uma escolha de forma, não de conteúdo: os conceitos (DDD/SOLID/Hexagonal/ROP/CQS) entram como **uma seção "Princípios de design" nova** (§3.5, antes das ADRs, transversal a todas), ou **diluídos nas ADRs que cada um toca** (Hexagonal→ADR-01, LSP→ADR-05, OCP→ADR-02, Aggregate→ADR-04)? A seção transversal é mais legível para onboarding (o anexo §14 valoriza isso); a diluição mantém cada princípio ao lado da decisão concreta que ele justifica, sem seção que ninguém lê. Ambas defensáveis — é a tua preferência de como o documento é consumido que decide.
