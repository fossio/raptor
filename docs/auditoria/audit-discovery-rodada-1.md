# Auditoria de Arquitetura — Discovery OFX Rust/WASM

**Data:** 2026-07-18
**Score de Maturidade:** 6.1 / 10
**Artefato auditado:** `discovery-ofx-rust-wasm.md`
**Fonte de verdade para escopo:** handoff técnico anexo (§1–§3)

---

## Resumo Executivo

O documento é tecnicamente sólido nas decisões de engenharia de fronteira — a estratégia de AST neutro (ADR-02), a separação `Decimal`/`f64` (ADR-03) e o design do handle opaco em WASM (ADR-06) estão corretos e bem justificados, sem erro factual identificado sobre SGML/XML/FDX/toolchain Rust-wasm. O risco não está na tecnologia escolhida, está na **rastreabilidade entre o handoff e o documento**: duas lacunas de escopo passam despercebidas porque nada no texto sinaliza que ficaram de fora, o que é pior do que um não-objetivo explícito.

Os dois achados críticos apontam para o mesmo padrão de falha — omissão silenciosa em vez de decisão registrada. Faltam 2 das 20 métricas do handoff (Burn Rate e Runway) em qualquer módulo, tabela ou fase de roadmap. E o tipo `Money` não carrega moeda, o que permite `analytics::aggregation` somar `TRNAMT` de contas em moedas diferentes sem erro nem diagnostic — corrompendo o próprio invariante de exatidão financeira que o ADR-03 declara como motivação central.

Prioridade de ação: fechar as duas lacunas críticas antes de iniciar a Fase 0 do roadmap, já que ambas afetam a assinatura de tipos (`Money`) e a categorização de módulos (`analytics`) — corrigir depois do código escrito custa muito mais que corrigir agora no documento.

---

## Score por Categoria

| Categoria | Score |
|---|---|
| Completude de escopo (rastreabilidade handoff → documento) | 4.5 |
| Modelagem de domínio e invariantes financeiras | 5.0 |
| Consistência interna do documento | 6.5 |
| Corretude técnica (afirmações Rust/OFX/WASM) | 9.0 |
| Testabilidade / evals discriminantes | 5.5 |
| Segurança, privacidade e compliance | 5.5 |
| Estrutura e rastreabilidade como artefato ADR | 7.0 |

---

## Inventário de Problemas

### Críticos

**1. Duas das 20 métricas do handoff não aparecem em nenhum módulo, tabela ou fase do roadmap.**
Evidência: a tabela de mapeamento do ADR-05 cobre `aggregation` (NCF, ADB, AT), `credit` (CU, k_d), `anomaly` (Z-Score), `returns` (HPR, TWR, IRR, CAGR, DY), `risk` (σ, Sharpe, MDD, VaR, w_i) e `trend` (SMA, EMA) — 18 métricas. A categoria "Preditiva" do handoff (§3), com Taxa de Queima (BR) e Pista de Sobrevivência (Runway), não tem módulo correspondente nem menção em nenhuma outra seção do documento.
Recomendação: adicionar módulo `predictive` (ou subsumir em `trend`) para BR e Runway, e explicitar que Runway consome o *output* de BR — não os dados brutos de transação. Isso quebra a premissa declarada em ADR-05 de que toda métrica é `fn(&[T]) -> Metric<V>` sem estado; Runway é uma função de outra métrica, não do domínio diretamente.
Esforço: baixo — ainda é decisão de documento, nenhum código escrito.

**2. `Money(Decimal)` não carrega código de moeda — agregações cross-currency produzem resultado numericamente exato porém financeiramente incorreto, sem diagnostic.**
Evidência: ADR-03 define a invariante "nenhuma soma monetária em ponto flutuante" mas não menciona `CURDEF`/`CURSYM`. O OFX permite contas e transações em moedas distintas dentro do mesmo documento. `NCF`, `ADB` e `AT` somam `TRNAMT` sem checagem de moeda.
Recomendação: `Money { amount: Decimal, currency: CurrencyCode }`; agregação entre moedas diferentes deve falhar explicitamente ou exigir conversão, nunca somar silenciosamente.
Esforço: médio — decisão precisa ser fechada antes de `analytics::aggregation` ser implementado, porque muda a assinatura de tipo em toda a cadeia `domain → analytics → bindings`.

### Altos

**3. Bill Payment e Taxes citados como Message Sets suportados (ADR-04, handoff §5) mas ausentes do layout de crates e do roadmap.**
Evidência: ADR-04 lista os cinco Message Sets do handoff; o layout do workspace (§4) só tem `banking.rs`, `creditcard.rs`, `investment.rs`; o roadmap (§9) não tem fase para os outros dois.
Recomendação: decidir explicitamente — dentro do MVP (adiciona módulos + fase) ou fora (move para §2, Não-objetivos). Hoje é ambiguidade não assumida, não uma escolha.
Esforço: baixo.

**4. Corpus de golden files planeja usar dialetos reais de banco sem decisão de anonimização ou sintetização (§7).**
Evidência: "corpus de golden files versionado (...) um por combinação versão × dialeto de banco" não especifica origem sintética. Dado bancário real versionado em repositório — mesmo privado — é extrato financeiro de pessoa física identificável.
Recomendação: registrar decisão — corpus gerado programaticamente por dialeto (preferível) ou anonimizado (scrub de `FITID`, nome do titular, número de conta) antes do primeiro commit.
Esforço: baixo se decidido agora; alto se descoberto depois de dado real já estar no histórico do git.

**5. Evals discriminantes (§7) só têm exemplo para `aggregation` e `returns`/IRR — `risk`, `trend` e `credit` não têm caso-armadilha definido.**
Evidência: os dois exemplos fraco/forte cobrem NCF e IRR; Sharpe, VaR, MDD, SMA/EMA, CU e k_d não têm par fraco/forte equivalente.
Recomendação: replicar o padrão fraco→forte para pelo menos um caso por módulo restante antes da Fase 3.
Esforço: médio.

### Médios

**6. Timezone/offset embutido em `DTPOSTED` não é normatizado em ADR-02 nem ADR-04.**
Evidência: o formato OFX inclui sufixo de offset (ex.: `[-3:BRT]`); métricas por janela temporal (ADB, SMA/EMA, Z-Score) ficam sensíveis a isso sem uma regra de normalização.
Recomendação: normalizar toda data para UTC no mapeamento AST→domínio, e registrar a decisão explicitamente em ADR-02 ou ADR-04.
Esforço: baixo.

**7. Assinatura declarada para analytics ("função pura `fn(&[T]) -> Metric<V>`", ADR-05) não acomoda métricas compostas como Runway.**
Evidência: ver achado #1 — Runway depende do resultado de BR, não apenas dos dados brutos de transação/saldo.
Recomendação: ou declarar uma segunda categoria de assinatura (`fn(Metric<V>) -> Metric<V>` para métricas derivadas) ou tratar BR como etapa intermediária dentro do cálculo de Runway, sem expô-la como métrica pública separada — decisão de modelagem que falta ser tomada.
Esforço: baixo.

**8. Status único "Proposto" no cabeçalho trata as 9 decisões como bloco monolítico, sem diferenciar as de alta incerteza (ADR-02, parsing; ADR-05, analytics) das quase-triviais (ADR-07, perfil de build; ADR-09, garantia estrutural de privacidade).**
Evidência: tabela de metadados no topo do documento (§0) tem um único campo `Status`.
Recomendação: status por ADR individual, ou ao menos uma nota de confiança relativa entre as decisões mais e menos sujeitas a revisão.
Esforço: baixo.

### Baixos

**9. Nenhuma decisão sobre limite de tamanho de input ou proteção de memória linear contra arquivo malicioso/corrompido no WASM.**
Evidência: ADR-09 cobre privacidade estrutural (sem rede) mas não cobre DoS client-side por arquivo anormalmente grande.
Recomendação: definir um limite de bytes de entrada e comportamento ao excedê-lo (erro explícito antes do parsing).
Esforço: baixo.

**10. Nenhuma menção a `wasm-bindgen-test` ou estratégia de teste para a superfície `bindings` (serialização, ciclo de vida do handle).**
Evidência: §7 cobre golden files e proptest, ambos nativos — a camada que efetivamente cruza a fronteira JS/WASM fica sem cobertura de teste declarada.
Recomendação: adicionar suíte `wasm-bindgen-test` rodando em headless browser ou Node antes da Fase 5.
Esforço: médio.

**11. Deduplicação de `FITID` entre reimportações de extratos sobrepostos não é endereçada.**
Evidência: ausente de ADR-04 e ADR-08.
Recomendação: não bloqueia parsing de arquivo único; registrar como decisão pendente para quando o consumidor importar múltiplos arquivos do mesmo período.
Esforço: baixo.

**12. Decisões individuais não têm identificador estável em front-matter para referência cruzada futura (só headings de texto).**
Evidência: `ADR-01` a `ADR-09` são headings markdown, sem campo estruturado (ex.: `id`, `status`, `data`) por decisão.
Recomendação: se o documento for referenciado em commits/PRs futuros, vale um bloco de metadados por ADR — não bloqueante para a fase atual.
Esforço: baixo.

---

## Recomendações Priorizadas

Mapear BR e Runway em um módulo explícito, documentando a dependência funcional entre as duas, antes de qualquer linha de `analytics` ser escrita — é o achado crítico de menor esforço e o que mais barato fica para corrigir agora.

Adicionar `currency` ao tipo `Money` e decidir a regra de agregação cross-currency antes de `analytics::aggregation` existir — crítico e o que mais caro fica se descoberto depois, porque muda assinatura de tipo em toda a cadeia de crates.

Decidir o destino de Bill Payment e Taxes — dentro do MVP ou explicitamente fora — antes de fechar o roadmap da Fase 2.

Fechar a política de corpus (sintético vs. anonimizado) antes do primeiro arquivo de teste ser commitado — o custo de correção sobe de baixo para alto assim que dado real entra no histórico do git.

Completar os evals discriminantes de `risk`, `trend` e `credit` seguindo o padrão já estabelecido para `aggregation`/`returns`, antes de considerar a Fase 3 como pronta para implementação.

---

## Metodologia

Auditoria baseada exclusivamente no texto do artefato gerado nesta conversa, cruzado contra o handoff técnico original (§1–§3) como fonte de verdade para escopo — em particular a tabela de 20 métricas e os 5 Message Sets citados. Não há código ou toolchain Rust/wasm-pack disponível neste ambiente para compilação real; a categoria "Corretude técnica" foi avaliada por conhecimento de domínio sobre compatibilidade `wasm32-unknown-unknown` das crates citadas (`quick-xml`, `encoding_rs`, `rust_decimal`, `wasm-bindgen`, `serde-wasm-bindgen`), não por build efetivo. Categorias como cobertura de teste real, complexidade ciclomática ou métricas de runtime não se aplicam — o artefato é um documento de decisão pré-implementação, não um sistema em execução.
