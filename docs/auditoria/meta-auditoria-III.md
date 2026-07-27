# Meta-auditoria III — Discovery OFX após princípios + inventários de processo

**Data:** 2026-07-19
**Alvo:** `discovery-ofx-rust-wasm.md` (423 linhas, 11 seções, 12 ADRs + §5 Princípios)
**Numeração de achados:** continua de #33 (meta-auditoria II fechou #26–32)
**Método:** releitura integral do documento contra o arco da conversa, mais verificação do código real em `ofx-domain` contra as afirmações novas que a §5 introduziu.

---

## Análise de intenção

O arco desde a última meta-auditoria: três inventários genéricos (OSS repo, Claude Code, GitHub) incorporados como ADR-10/11/12, seguidos de uma análise conceitual (DDD/SOLID/Hexagonal) que virou a §5 Princípios. O pedido recorrente por trás de todos foi o mesmo — "incorpore o *necessário*, não tudo" — e o filtro se manteve consistente (conjunto mínimo por fase, rejeição de _documentation theater_). A intenção desta auditoria, idêntica à da Fase 6: depois de várias emendas seguidas, verificar se o documento ainda é internamente coerente ou se as incorporações criaram contradições, referências quebradas ou promessas que o código não cumpre.

O sinal mais forte de intenção é o próprio hábito, agora estabelecido: cada bloco de incorporação é seguido de um pedido de auditoria. O usuário trata o documento como artefato que precisa passar por gate de qualidade toda vez que cresce — não confia no acréscimo incremental sem verificação. É a mesma disciplina de "verifique novamente" que apareceu na primeira auditoria.

## Levantamento de dados

Verificado contra o sandbox, não de memória:
- Código em `ofx-domain`: **zero** `println!`/`eprintln!` (aderente à regra nova), **zero** ocorrências de `f64` (regra preventiva, ainda não exercida), **zero doctests** (`grep '```'` em `src/*.rs` retorna 0).
- `rustfmt.toml`: **não existe** no workspace.
- Documento: 11 seções numeradas sem lacuna, 12 ADRs, todas as `§N` cruzadas resolvendo para a seção certa após a renumeração da última edição.

---

## Achados

### Altos

**#33 — A §5 afirma doctests como convenção, mas o código já entregue tem zero.** A §5 lista "Doctests (`cargo test --doc`) na API pública de `domain`" entre as convenções de código, no presente. O `ofx-domain` real — que foi compilado e testado nesta conversa — não tem um único doctest. Não é bug, mas é uma promessa não cumprida no único código que existe: a convenção descreve um futuro como se fosse presente. Ou a §5 marca doctests explicitamente como "a adotar na Fase 0" (alinhado com o resto dos princípios, que são forward-looking), ou o `ofx-domain` ganha doctests nos itens públicos (`Money::try_add`, `classify_banking`, `burn_rate`) para honrar a afirmação. A primeira opção é mais barata e mais honesta com o estado atual.

**#34 — O `CLAUDE.md` de exemplo (ADR-11) referencia "ADR-01 a ADR-11", congelado antes da ADR-12 existir.** Linha 304 do Discovery, dentro do bloco de código markdown do CLAUDE.md sugerido: `(ADR-01 a ADR-11)`. A ADR-12 foi adicionada no turno seguinte e o exemplo não acompanhou. Quem copiar esse `CLAUDE.md` para o repo herda uma referência que já nasce desatualizada. Trocar para "ADR-01 a ADR-12" ou, melhor, "todas as ADRs" para não precisar editar a cada nova.

### Médios

**#35 — Escopo do header não reflete que o projeto ganhou uma camada de processo/governança.** O campo Escopo (linha 7) diz "Parsing multi-versão OFX + domínio tipado + analytics + fronteira WASM offline" — descreve só o software. Mas o documento agora carrega três ADRs de processo (repositório, Claude Code, GitHub) e uma seção de princípios de engenharia. Um leitor novo lê o escopo e não espera encontrar decisão sobre labels de GitHub no mesmo documento. Isto é a versão III do achado #30 da meta-auditoria II (que você decidiu manter tudo junto) — não proponho separar de novo; proponho que o *header* admita as duas naturezas, ex.: "...fronteira WASM offline; mais princípios de design e decisões de processo/repositório (§5, ADR-10–12)".

**#36 — "20 métricas" no diagrama e no texto, mas agora são 20 em 7 módulos incluindo `predictive` — a contagem virou ambígua.** O diagrama (linha 50) rotula `analytics` como "20 métricas, funções puras", e o §1 fala em "20 operações quantitativas". Mas o achado #1 adicionou BR e Runway ao módulo `predictive`, e a §5 estabeleceu que Runway *não* é função pura (é família derivada). Então "20 métricas, funções puras" no diagrama é impreciso em dois eixos: são 20 contando as 2 do predictive (o handoff original tinha 20), mas nem todas são puras. O texto do handoff tinha 20 no total; o mapa do ADR-05 lista as mesmas 20 distribuídas. A imprecisão é pequena mas é factual — "funções puras" no rótulo do diagrama contradiz a §5. Ajustar o rótulo para "20 métricas (primárias puras + derivadas)".

**#37 — `predictive`/`cashflow`/`dedupe` aparecem como `area:` label válida (ADR-12) mas são módulos internos de `domain`, não crates.** Linha 337: "`area:` usa os nomes das crates (`domain`, `parse`, `analytics`, `bindings`, `predictive`, `dedupe`)". Mas `predictive` e `dedupe` (e `cashflow`) são *módulos dentro de* `domain`, não crates — o próprio layout do §4 mostra isso, e a §5/ADR-05 nota que `predictive` pode ser promovido a `analytics` na Fase 3/4. Listá-los como `area:` no mesmo nível de `domain`/`parse` mistura granularidades e cria labels que podem sumir quando o módulo migrar. Ou `area:` fica só nas crates reais (`domain`/`parse`/`analytics`/`bindings`), ou se assume explicitamente que `area:` mapeia módulos e não crates — mas aí `money`/`metric` também deveriam estar.

### Baixos

**#38 — `rustfmt.toml` é entregável da Fase 0 (ADR-10) mas a §5 fala em `cargo fmt` como se a config já existisse.** Menor: a §5 e o CLAUDE.md sugerido assumem `cargo fmt`/`clippy` operантes, e o ADR-10 lista `rustfmt.toml` como entregável da Fase 0. Coerente no roadmap, mas hoje o sandbox não tem o arquivo — o que é correto (Fase 0 ainda não rodou). Não é contradição, só vale confirmar que ninguém vai ler a §5 e assumir que a config de fmt já está no repo. Nenhuma ação obrigatória; anoto para completude.

**#39 — Data do header ("2026-07-18 revisão de fechamento") não avançou apesar de 4 rodadas de edição posteriores.** O header ainda diz "2026-07-18 (revisão de fechamento)", mas houve incorporação de 3 inventários + princípios depois disso, hoje (2026-07-19). A data virou fóssil de uma revisão específica. Ou vira "última atualização: 2026-07-19", ou ganha um mini-changelog de versões do próprio Discovery (o inventário GitHub, ironicamente, recomendava exatamente isso para artefatos que envelhecem).

**#40 — O documento não tem índice, e com 11 seções + 12 ADRs + 5 blocos de princípios já passou do tamanho em que um leitor navega sem um.** Puramente de navegação. Um sumário no topo (as 11 seções + lista de ADRs com uma linha cada) reduz o custo de localizar uma decisão específica num documento de 423 linhas. Barato, opcional.

---

## Sobre reestruturação

A pergunta explícita do pedido: é necessária reestruturação? **Não estrutural profunda** — a decisão de manter tudo junto (meta-auditoria II) se sustenta, e a §5 achou o lugar certo (antes das ADRs, transversal). O que há são desalinhamentos de superfície acumulados pelas incorporações rápidas: uma promessa que o código não cumpre (#33), uma referência congelada (#34), rótulos imprecisos (#36, #37) e metadados de header defasados (#35, #39).

O único que beira o estrutural é o #35 — o documento virou dois documentos num só (arquitetura de software + processo de projeto) e o header não admite isso. Mas você já decidiu conscientemente manter unido; então a correção é honestidade no header, não separação.

Nenhum destes bloqueia uso do documento. São a diferença entre um artefato que envelheceu bem e um que acumulou pequenas mentiras — e como a disciplina desta conversa inteira foi não deixar lacuna virar dívida, listo para fechar.

## Perguntas antes de editar

Duas onde a resposta muda o que faço, e não dá para inferir:
1. #33 (doctests): marcar como convenção futura na §5, ou adicionar doctests reais ao `ofx-domain` agora?
2. #39 (data/versionamento do próprio Discovery): trocar a data por "última atualização", ou adicionar um mini-changelog de versões no topo?

Os demais (#34, #35, #36, #37, #38, #40) têm correção óbvia e de baixo risco — posso aplicar todos sem decisão tua, se autorizar.
