# Auditoria de Arquitetura — Discovery OFX Rust/WASM (Segunda Passada)

**Data:** 2026-07-18
**Score de Maturidade revisado:** 5.4 / 10 (era 6.1 na primeira passada)
**Continuação de:** `audit-discovery-ofx-rust-wasm.md` — numeração de achados prossegue a partir de #13
**Motivo do rebaixamento:** um achado transversal (#13) invalida a aplicabilidade de ~6 métricas sobre dados OFX crus e não estava visível na primeira passada, que auditou completude de escopo antes de corretude de aplicação.

---

## Resumo Executivo

A primeira passada auditou *o que falta* (métricas ausentes, Message Sets ambíguos, moeda no tipo). Esta segunda passada audita *o que está presente mas não funciona como afirmado* — a corretude de aplicação das fórmulas do handoff sobre a estrutura de dados real do OFX. O resultado é pior, e por um motivo estrutural único: **o OFX não distingue fluxo de capital de performance**, e quase todas as métricas de retorno e risco do handoff assumem essa distinção como dada. Um aporte de R$10.000 numa conta e um rendimento de R$10.000 produzem a mesma variação de `BALAMT`. Qualquer métrica que derive retorno de saldo sem separar cashflows externos está numericamente exata e financeiramente errada — o mesmo modo de falha do achado crítico #2 (moeda), agora generalizado para retorno/risco.

O documento trata as 20 fórmulas como se fossem implementáveis diretamente sobre `TRNAMT`/`BALAMT`/`UNITPRICE`. Cerca de um terço delas exige input externo não modelado (taxa livre de risco, série consolidada de cotações, valorização do portfólio em cada data de fluxo) ou pré-processamento que separa aporte de rendimento. Nada disso aparece nas assinaturas de `analytics` nem na proveniência declarada em ADR-05.

---

## Inventário de Problemas (continuação)

### Críticos

**13. [Transversal] OFX não separa fluxo de capital de performance; métricas de retorno derivadas de saldo/posição estão estruturalmente incorretas sem cashflow tagging.**
Afeta: HPR, TWR, CAGR, MDD (quando calculado sobre `BALAMT`), e por consequência σ, Sharpe e VaR derivados dessa série.
Evidência: o handoff (§3) define, por exemplo, `CAGR = (V_f/V_i)^(1/t) − 1` mapeado para `BALAMT`/`INVPOS`. Se houve aporte entre `V_i` e `V_f`, o crescimento medido é contribuição + rendimento, não retorno. `MDD sobre BALAMT` numa conta que recebe depósitos mede recomposição de saldo, não drawdown de performance. `HPR = (P_f − P_i + D)/P_i` só é válido para uma posição sem compra/venda intermediária — que o OFX registra em `INVTRAN` e que a fórmula ignora.
Recomendação: introduzir uma etapa de *cashflow classification* no domínio (ou em analytics) que separe movimentações externas (aporte/saque/transferência) de eventos de performance, antes de qualquer métrica de retorno. TWR e IRR/XIRR já pressupõem isso — TWR quebra a série nos fluxos, XIRR usa os fluxos como termos; ambas só funcionam se os fluxos estiverem identificados. As demais métricas de retorno precisam da mesma base.
Esforço: alto — é decisão de modelagem que precede a Fase 3 e altera o contrato de `analytics::returns` e `analytics::risk`.

### Altos

**14. Métricas que exigem input externo não têm parâmetro na assinatura nem entrada na proveniência.**
Evidência: Sharpe exige taxa livre de risco (`R_f`) — não existe no OFX. VaR e volatilidade anualizada exigem definição de frequência/horizonte. `w_i` e Sharpe de portfólio exigem série consolidada de cotações que um único arquivo raramente contém (conecta com a questão em aberto §10 do Discovery, mas ali como dúvida, não como impacto de assinatura). A proveniência declarada em ADR-05 (janela temporal, contagem, origem) não prevê registrar inputs externos — logo um Sharpe fica não-auditável, contradizendo o objetivo de auditabilidade do §3.
Recomendação: assinaturas dessas métricas recebem um struct de parâmetros externos explícito (`RiskFreeRate`, `Horizon`, `PriceSeries`), e a proveniência passa a incluí-los. Sem isso, o valor retornado não é reproduzível a partir só do documento OFX.
Esforço: médio.

**15. Validade estatística sobre amostras pequenas e não-normais não é reconhecida.**
Evidência: extrato mensal tem dezenas de observações, não milhares. `VaR = q_{1−α}(R)` como quantil histórico sobre ~30 pontos é ruído. `Z-Score` de despesas assume normalidade; gasto pessoal tem cauda pesada e sazonalidade — a taxa de falso positivo/negativo em detecção de anomalia será alta. σ amostral (N−1) sobre poucos pontos é instável.
Recomendação: registrar como limitação explícita de cada métrica (a proveniência já carrega a contagem de inputs — usá-la para emitir diagnostic de baixa significância abaixo de um N mínimo). Não é bug de código, é honestidade estatística que o documento deve declarar antes de expor a métrica.
Esforço: baixo (documentar + threshold de diagnostic).

**16. Anualização e frequência de retornos indefinidas.**
Evidência: σ, Sharpe, EMA (fator α), SMA (janela k) dependem da frequência da série, que no OFX é irregular (transações não são equiespaçadas). Anualizar σ diário e σ mensal usa fatores diferentes; o documento não define de onde vem a frequência nem se há reamostragem.
Recomendação: decidir política de reamostragem (ex.: resample para grade diária/mensal antes das métricas de janela) e parametrizar α da EMA por span, não como constante mágica.
Esforço: médio.

**17. `Vec<Diagnostic>` sem taxonomia de códigos estáveis (refinamento de ADR-08).**
Evidência: ADR-08 define parsing parcial com diagnostics, mas trata `Diagnostic` como texto + severidade. O consumidor não consegue distinguir programaticamente "transação descartada por FITID ausente" de "tag proprietária ignorada" sem string matching.
Recomendação: enum de códigos estável (`DiagnosticCode`) versionado junto da API pública. É o que torna o parsing parcial realmente consumível em vez de só observável.
Esforço: baixo.

### Médios

**18. `TRNAMT` com vírgula decimal (violação real do spec por emissores locais) quebra `rust_decimal::from_str`.**
Evidência: o spec OFX manda ponto decimal, mas há emissores br/europeus que produzem `<TRNAMT>-123,45`. `Decimal::from_str` rejeita vírgula. Conecta com resiliência (ADR-08): hoje isso seria erro fatal de mapeamento numa transação, não um diagnostic recuperável.
Recomendação: normalização de separador decimal no mapeamento AST→domínio, com diagnostic quando aplicada (rastreabilidade da correção).
Esforço: baixo.

**19. XML 2.x — expansão de entidades (billion laughs) como vetor de DoS de memória, distinto do limite de tamanho de arquivo (#9).**
Evidência: #9 (primeira passada) trata tamanho de bytes do input. Entity expansion é ortogonal: um arquivo pequeno pode expandir para gigabytes em linear memory. `quick-xml` não faz fetch externo (XXE é inócuo, ainda mais em WASM sem I/O), mas expansão interna precisa de limite. Não mencionado.
Recomendação: cap de expansão de entidades no reader XML, com erro explícito.
Esforço: baixo.

**20. Contrato de ciclo de vida do handle opaco depende de `FinalizationRegistry`, que não é determinístico.**
Evidência: ADR-06 menciona FinalizationRegistry como mitigação de ciclo de vida. Ele não garante execução — numa SPA de longa duração processando muitos arquivos, `OfxDocument` vaza linear memory até o registry (talvez) coletar. `free()` explícito precisa ser parte documentada do contrato JS, não fallback.
Recomendação: documentar `doc.free()` como obrigatório no contrato de uso e no exemplo do `app.js`; tratar FinalizationRegistry como rede de segurança, não como mecanismo primário.
Esforço: baixo.

### Baixos

**21. Afirmação imprecisa: "evita cópia dupla" (ADR-06).**
Evidência: `wasm-bindgen` copia o `Uint8Array` de JS para a linear memory do WASM na entrada — há uma cópia JS→wasm inevitável. O handle opaco evita a cópia *de volta* (serializar o documento inteiro para JS), o que é real, mas a redação sugere zero cópia. Ajustar a afirmação para não superverder.
Esforço: baixo.

**22. `panic = "abort"` sem `console_error_panic_hook` compromete diagnóstico de dialetos de banco.**
Evidência: ADR-07 escolhe `panic = "abort"` por tamanho. Sem o hook, um panic durante parse de um dialeto novo aborta sem mensagem útil no console — exatamente o cenário de debug mais frequente (arquivo real de banco desconhecido). Trade-off tamanho × diagnosticabilidade não discutido.
Recomendação: hook habilitável em build de debug via feature flag, mantendo o release enxuto.
Esforço: baixo.

**23. Roadmap sem gate de decisões bloqueantes antes da Fase 0.**
Evidência: §9 é incremental mas assume que as decisões críticas (#2 moeda, #13 cashflow, #1 métricas faltantes) serão resolvidas "durante". Elas alteram assinaturas de tipo e precisam preceder o código, não acompanhá-lo.
Recomendação: uma "Fase −1" explícita de fechamento de decisões bloqueantes como pré-condição da Fase 0.
Esforço: baixo.

**24. Superfície pública da lib e política de SemVer não declaradas.**
Evidência: `domain` é chamado de "núcleo estável" (ADR-01) sem compromisso de estabilidade de API, nem definição de o que é `pub` vs `pub(crate)`. Para uma lib consumida por Rust e por JS, a fronteira pública é contrato.
Recomendação: declarar a superfície pública mínima e política de versionamento antes da Fase 5 (quando `bindings` congela o contrato JS).
Esforço: baixo.

**25. Tabela de aridade SGML não cobre extensões proprietárias de banco.**
Evidência: ADR-02 deriva a aridade das DTDs 1.x. Emissores adicionam tags fora do DTD (`<INTU.BID>`, campos proprietários); o tokenizer não sabe se são folha ou agregação. O "fallback heurístico" foi citado sem especificação.
Recomendação: especificar a heurística de fallback (ex.: presença de tag de fechamento correspondente no stream define aridade em runtime) e cobri-la no corpus com um arquivo de dialeto extenso.
Esforço: médio.

---

## Score por Categoria (revisado)

| Categoria | 1ª passada | 2ª passada | Δ |
|---|---|---|---|
| Completude de escopo | 4.5 | 4.5 | — |
| Modelagem de domínio e invariantes financeiras | 5.0 | 3.5 | ↓ #13, #14 |
| Consistência interna do documento | 6.5 | 6.0 | ↓ #21 |
| Corretude técnica (afirmações e aplicação) | 9.0 | 7.0 | ↓ #15, #16, #19 |
| Testabilidade / evals discriminantes | 5.5 | 5.0 | ↓ #15 |
| Segurança, privacidade e compliance | 5.5 | 5.0 | ↓ #19, #20 |
| Estrutura e rastreabilidade como artefato ADR | 7.0 | 6.5 | ↓ #23, #24 |

Score geral: média aritmética simples = **5.4 / 10**.

---

## Recomendações Priorizadas (consolidadas com a 1ª passada)

O achado #13 sobe ao topo de tudo. Antes de moeda, antes das métricas faltantes: definir a etapa de *cashflow classification* que separa aporte/saque de performance. Sem ela, metade da categoria `returns` e parte de `risk` entregam números plausíveis e errados — o pior tipo de defeito num sistema financeiro, porque não falha, mente. É o gate real da Fase 3/4.

Fechar simultaneamente #2 (moeda em `Money`) e #14 (parâmetros externos + proveniência): os três achados de assinatura (#2, #13, #14) tocam a mesma cadeia de tipos `domain → analytics → bindings` e devem ser resolvidos num único movimento de design, não em três rodadas.

Adicionar a "Fase −1" do #23 ao roadmap tornando #1, #2, #13, #14 pré-condições explícitas — é o mecanismo que impede esses achados de virarem retrabalho depois do código escrito.

Os demais (#15 a #25) são refinamentos de baixo/médio esforço que não bloqueiam, mas #17 (taxonomia de diagnostics) e #18 (vírgula decimal) valem entrar já na Fase 1, porque ambos moldam o contrato de `ParseOutcome` que o resto consome.

---

## Metodologia

Segunda passada focada em corretude de *aplicação* das fórmulas do handoff sobre a estrutura de dados OFX real, dimensão não coberta na primeira passada (que priorizou completude de escopo). Fonte de verdade mantida: handoff §3 (tabela de 20 métricas com tags OFX mapeadas). Avaliação por conhecimento de domínio financeiro e de compatibilidade `wasm32`, sem build efetivo — não há toolchain Rust/wasm-pack neste ambiente. As categorias rebaixadas refletem defeitos que só se manifestam quando as fórmulas encontram dados com aportes, moedas mistas, amostras pequenas ou dialetos proprietários — todos ausentes de um teste ingênuo de caminho feliz, o que explica por que a primeira passada, olhando o documento como especificação, não os expôs.

Este relatório é uma auditoria de documento pré-implementação. Nenhum achado indica erro de código porque não há código; todos indicam decisões de design que, se não fechadas antes da implementação, se tornam defeitos difíceis de reverter.
