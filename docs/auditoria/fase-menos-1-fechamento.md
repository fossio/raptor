# Fase −1 — Fechamento de Decisões Bloqueantes

> **⚠️ Superado.** Este documento reflete o estado logo após a Fase −1, antes do
> fechamento completo da auditoria. A seção "O que NÃO foi resolvido nesta fase"
> abaixo lista achados que **foram fechados depois** (ver `fechamento-auditoria.md`,
> que é a fonte de verdade atual para status de achado). Mantido aqui só como
> registro histórico do que a Fase −1 especificamente cobriu — não leia a seção
> de pendências como estado atual do projeto.

Pré-condição da Fase 0 do roadmap original, conforme recomendado na segunda passada da auditoria. Diferente do resto do Discovery, esta fase não ficou só no papel — os quatro achados abaixo foram materializados como código Rust real, compilado com `cargo test` e `cargo clippy -- -D warnings` limpos (17/17 testes, 0 warnings — hoje são 20/20, ver `fechamento-auditoria.md`). Crate: `ofx-domain`, único módulo tocado nesta fase.

## Achados fechados

**#2 — Moeda em `Money`** (`money.rs`). `CurrencyCode` obrigatório no tipo; `Money` não implementa `std::ops::Add` de propósito — soma passa por `try_add`/`sum_homogeneous`, que retornam `Err(CurrencyMismatch)` em vez de misturar moedas silenciosamente. `sum_homogeneous` é o ponto de entrada que `analytics::aggregation` (NCF/ADB/AT) vai usar.

**#13 — Cashflow classification** (`cashflow.rs`). `CashflowKind::{External, Performance, Neutral}` mapeado a partir de `TRNTYPE` (Banking) e dos subtipos de `INVTRAN`/`INCOME` (Investments). O caso que mais importa está no teste `compra_de_acao_e_neutral_nao_performance`: comprar uma ação não é rendimento, é realocação de capital dentro da carteira — é exatamente a distinção que faltava para TWR/IRR/HPR/CAGR pararem de confundir aporte com performance. `Confidence::Low` em `Credit`/`Debit`/`Other` sinaliza a jusante que a classificação ali é heurística fraca, não leitura direta do spec.

**#1 e #7 — Módulo `predictive`** (`predictive.rs`). Duas famílias de assinatura declaradas explicitamente: `burn_rate` é primária (`fn(&[Money], ...) -> Metric<Money>`), `runway` é derivada (`fn(Money, &Metric<Money>) -> Metric<Decimal>`) — consome o resultado de `burn_rate`, não dados brutos. Isso resolve o achado #7: a premissa de "toda métrica é função pura independente" do ADR-05 original agora tem uma segunda família reconhecida, em vez de forçar Runway num molde que não serve.

**#14 e #15 — Provenance com inputs externos + diagnostic de amostra** (`metric.rs`). `Provenance::external_inputs: Vec<ExternalInput>` — quando `RiskParams` (taxa livre de risco, horizonte) entrar em `analytics::risk` na Fase 4, o valor fica registrado na proveniência, não só usado e descartado. `Provenance::new` já emite `MetricDiagnostic::LowSampleSignificance` automaticamente abaixo de 30 observações — testado em `amostra_pequena_gera_diagnostic_automatico`.

## O que NÃO foi resolvido nesta fase

Fora de escopo deliberado — a Fase −1 fechou só os quatro achados críticos/altos que travavam a assinatura de tipos (`domain → analytics → bindings`). Continuam abertos para as fases seguintes:

- **#3** — decisão sobre Bill Payment/Taxes dentro ou fora do MVP (é decisão de roadmap, não de tipo — não bloqueava código).
- **#4** — política de anonimização do corpus de golden files (decisão de processo, entra na Fase 1 antes do primeiro commit de arquivo real).
- **#5** — evals discriminantes para `risk`/`trend`/`credit` (só existe código de domínio ainda, não há métrica para testar).
- **#6, #16** — normalização de timezone e política de reamostragem/anualização — tocam `parse` (Fase 1) e `analytics::risk`/`trend` (Fase 4), não `domain`.
- **#17** — taxonomia de `Diagnostic` do parsing (distinta do `MetricDiagnostic` de analytics criado aqui) — pertence a `ofx-parse`, Fase 1.
- **#18–#25** — todos pertencem a `parse`, `bindings` ou processo de projeto; nenhum bloqueava a Fase 0.

## Ajuste no Discovery original

O ADR-05 do documento original precisa de uma nota: a assinatura "toda métrica é `fn(&[T]) -> Metric<V>` sem estado" deixa de valer como regra única — `predictive::runway` é o primeiro caso documentado da família derivada, e `analytics::returns`/`risk` provavelmente terão mais (TWR, por exemplo, também não é pura sobre dados crus depois que a classificação de cashflow entra no meio).
