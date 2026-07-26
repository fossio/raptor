# Fechamento da Auditoria — Discovery OFX Rust/WASM

**Data:** 2026-07-18
**Achados totais:** 25 (12 da primeira passada, 13 da segunda)
**Status:** 25/25 fechados — 6 em código compilado e testado, 19 como decisão vinculante registrada no Discovery

Nenhum achado ficou sem resposta. A diferença entre "fechado em código" e "fechado como decisão" não é gradação de esforço — é sobre em qual crate o achado vive. `ofx-domain` já existe e compila; `parse`, `analytics` de nível de métrica e `bindings` ainda não foram escritos (são Fases 0 a 5 do roadmap). Fechar um achado de `parse` "em código" hoje significaria escrever o parser inteiro fora de fase — então o fechamento, para esses, é tornar a decisão explícita e não-negociável no Discovery, para que quem escrever `parse` na Fase 0/1 não tenha a decisão em aberto para reinventar.

## Fechados em código (crate `ofx-domain`, 20 testes, `cargo clippy -D warnings` limpo)

| # | Achado | Onde |
|---|---|---|
| 1 | Burn Rate e Runway ausentes de qualquer módulo | `predictive.rs` |
| 2 | `Money` sem moeda permite soma cross-currency silenciosa | `money.rs` |
| 7 | Assinatura "toda métrica é pura" não cobre Runway | `predictive.rs` (família derivada) |
| 10 | Deduplicação de FITID entre reimportações | `dedupe.rs` |
| 13 | OFX não separa fluxo de capital de performance | `cashflow.rs` |
| 14 | Métricas com input externo sem parâmetro nem proveniência | `metric.rs` (`ExternalInput`, `RiskParams`) |
| 15 | Amostra pequena sem sinalização de baixa significância | `metric.rs` (`Provenance::new` emite diagnostic automático) |

## Fechados como decisão vinculante no Discovery

| # | Achado | ADR |
|---|---|---|
| 3 | Bill Payment/Taxes ambíguos entre suportados e ausentes | ADR-04 — movidos para não-objetivos, fora do MVP |
| 4 | Corpus de golden files com dado bancário real | ADR-08 — corpus sintético por dialeto, decisão vinculante |
| 5 | Evals discriminantes só para 2 de 6 módulos | §7 — padrão fraco/forte estendido a credit/risk/trend |
| 6 | Timezone de `DTPOSTED` não normatizado | ADR-08 — normalização para UTC é responsabilidade do parser |
| 8 | Status único trata 9 decisões como bloco monolítico | Header + tags `[Status: ...]` por ADR revisado |
| 9 | Sem limite de tamanho de input (DoS de memória) | ADR-09 — teto de 50 MB antes de chamar `ofx-parse` |
| 11 | ADRs sem identificador estável para referência cruzada | Tags `[Achado #N fechado]` cumprem essa função inline |
| 12 | Superfície pública e SemVer não declarados | ADR-01 — pré-1.0 até Fase 5, `pub(crate)` como padrão |
| 16 | Frequência/anualização de retornos indefinida | ADR-05 — reamostragem para grade diária, fator por horizonte |
| 17 | `Diagnostic` de parsing sem taxonomia de códigos | ADR-08 — `DiagnosticCode` enum inicial especificado |
| 18 | `TRNAMT` com vírgula decimal quebra `Decimal::from_str` | ADR-08 — normalização no mapeamento, diagnostic rastreável |
| 19 | Expansão de entidades XML como DoS de memória | ADR-08 — teto configurável no reader `quick-xml` |
| 20 | Ciclo de vida do handle dependente só de `FinalizationRegistry` | ADR-06 — `free()` explícito é contrato obrigatório |
| 21 | Afirmação "evita cópia dupla" imprecisa | ADR-06 — redação corrigida |
| 22 | `panic = "abort"` sem diagnóstico em dev | ADR-07 — `console_error_panic_hook` via feature flag |
| 23 | Roadmap sem gate de decisões bloqueantes | §9 — Fase −1 registrada como concluída |
| 24 | (duplicata conceitual de #12 — mesma resolução) | ADR-01 |
| 25 | Fallback de aridade SGML sem heurística especificada | ADR-02 — heurística de runtime especificada |

## O que isso não é

Fechar #4, #6, #16, #17, #18, #19, #25 como decisão não testa nada — não há `ofx-parse` para rodar. O valor de fechar agora é remover a decisão do caminho crítico de quem escrever a Fase 0/1: a pessoa (ou eu, numa sessão futura) não vai decidir política de corpus no meio de escrever o tokenizer SGML. Se alguma dessas decisões se mostrar errada ao encontrar dado real, o custo de mudar é o custo normal de mudar uma decisão documentada — não o custo de uma lacuna que ninguém sabia que existia, que era o problema original que a auditoria pegou.
