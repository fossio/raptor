# Documentação do `raptor`

Duas categorias, e a distinção importa: **documento vivo** é fonte de verdade e se edita quando
a decisão muda; **documento histórico** é o registro de um momento e não se edita para refletir o
presente — se o conteúdo dele foi superado, quem supera é um documento vivo.

As issues do repositório referenciam achados por número (`achado #76`, `A#22`, `RA-11/25`). O
índice abaixo é o que faz essas referências resolverem.

## Documentos vivos

| Documento | O que é |
|---|---|
| [`discovery-ofx-rust-wasm.md`](discovery-ofx-rust-wasm.md) | Decisões de arquitetura (ADR-01…13), princípios, estratégia de testes, roadmap tático por fases. **A fonte de verdade do projeto.** |
| [`roadmap-github.md`](roadmap-github.md) | Tradução do Discovery para o vocabulário do GitHub: labels, milestones M0–M7, e o texto de cada issue |
| [`evolucao-ofx.md`](evolucao-ofx.md) | Visão de longo prazo pós-Fase 6 (horizontes H0–H3), benchmark de mercado e o **anti-roadmap** — o que deliberadamente nunca entra. Visão proposta, não decisão: nada aqui é vinculante até virar ADR |
| [`handoff-ux-design-frontend.md`](handoff-ux-design-frontend.md) | Contexto de produto/UX para quem desenha a interface consumidora |
| [`alinhamento-issues.md`](alinhamento-issues.md) | Diff entre as issues abertas e os objetivos declarados nos documentos acima, issue a issue |

## Documentos históricos (`auditoria/`)

Registro das rodadas de crítica que produziram as decisões. Numeração de achados quase-contínua
(`#1`–`#85` para o Discovery, `A#1`–`A#33` para a auditoria de solução) — **com um buraco real,
não um documento perdido:** `meta-auditoria-III.md` diz continuar de "meta-auditoria II (achados
`#26`–`#32`)", mas nenhum documento com esse nome existe entre os arquivos originais desta
auditoria. `achado #26`–`#32` **não resolve para nenhum documento aqui** — não é lacuna de
indexação, é lacuna na própria fonte. Se algum desses números aparecer citado em algum lugar, a
resposta é "não documentado", não um link que falhou.

| Documento | Cobre |
|---|---|
| [`audit-discovery-rodada-1.md`](auditoria/audit-discovery-rodada-1.md) | Achados `#1`–`#12` |
| [`audit-discovery-rodada-2.md`](auditoria/audit-discovery-rodada-2.md) | Achados `#13`–`#25` |
| [`fechamento-auditoria.md`](auditoria/fechamento-auditoria.md) | Fechamento dos `#1`–`#25` — 7 em código, 18 como decisão vinculante (o header do próprio documento diz "6 e 19"; as duas tabelas somam 7+18=25 — contagem do header está errada, não corrigida aqui por ser registro histórico) |
| [`fase-menos-1-fechamento.md`](auditoria/fase-menos-1-fechamento.md) | Fase −1: os 4 achados bloqueantes materializados em código (`money`, `cashflow`, `predictive`, `metric`) |
| [`meta-auditoria-III.md`](auditoria/meta-auditoria-III.md) | Achados `#33`–`#40` — coerência do documento após as incorporações de processo. Cita "meta-auditoria II" (`#26`–`#32`) como predecessora; esse documento não está nos originais (ver nota acima) |
| [`analise-adr-generica.md`](auditoria/analise-adr-generica.md) | Análise conceitual (DDD/SOLID/Hexagonal) que virou a §5 do Discovery |
| [`analise-critica-adr.md`](auditoria/analise-critica-adr.md) | Terceira rodada — achados `#48`–`#85` |
| [`alteracoes-adr.md`](auditoria/alteracoes-adr.md) | Plano de aplicação dos `#48`–`#85` ao Discovery. **Aplicado integralmente.** (O título do próprio documento diz só "`#48`–`#64`" — desatualizado desde a Parte 2/3 que estendeu o escopo; o conteúdo é a fonte confiável, não o título) |
| [`auditoria-solucao.md`](auditoria/auditoria-solucao.md) | Auditoria de produto/viabilidade sob ótica CPO+CTO — achados `A#1`–`A#33` |
| [`resolucao-auditoria.md`](auditoria/resolucao-auditoria.md) | Respostas aos `A#1`–`A#33` (`RA-N`). **33/33 resolvidos** |
| [`rodada-execucao-pre-validacao.md`](auditoria/rodada-execucao-pre-validacao.md) | Validação anterior à execução — achados `#41`–`#47` (o título do documento diz só "`#41`–`43`"; a seção "Continuação" estende até `#47`) |
| [`plano-issues-github.md`](auditoria/plano-issues-github.md) | 15 correções à especificação das issues do roadmap. Estado de aplicação: ver [`alinhamento-issues.md`](alinhamento-issues.md) |
| [`hierarquia-artefatos-github.md`](auditoria/hierarquia-artefatos-github.md) | Esqueleto organizacional derivado da ADR-10/11/12/13. **Superado** pelo `roadmap-github.md`, que revisou a ADR-12 e trocou a taxonomia (ver §1 do alinhamento). O próprio documento se descreve como derivado de achados "`#1`–`#85`" de `fechamento-auditoria.md` — impreciso, esse arquivo cobre só `#1`–`#25`; sem efeito prático porque o documento inteiro está superado |

## Convenções de referência

- `achado #N` → depende da faixa: `#1`–`#12` em `audit-discovery-rodada-1.md`, `#13`–`#25` em
  `audit-discovery-rodada-2.md`, `#26`–`#32` **não documentado** (ver nota acima), `#33`–`#40` em
  `meta-auditoria-III.md`, `#41`–`#47` em `rodada-execucao-pre-validacao.md`, `#48`–`#85` em
  `analise-critica-adr.md`
- `A#N` → `auditoria-solucao.md`
- `RA-N` → `resolucao-auditoria.md`
- `ADR-NN` → `discovery-ofx-rust-wasm.md`
- `#N` sozinho, em issue → número de issue no GitHub
