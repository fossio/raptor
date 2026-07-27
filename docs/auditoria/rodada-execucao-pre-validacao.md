# Rodada de execução — pré-validação (achados #41–43)

**Data:** 2026-07-19
**Escopo:** validar riscos técnicos do Discovery que nunca tinham sido testados contra código ou fonte real — sem construir `parse`/`analytics`/`bindings`, que ficam para o agente executor.

---

## O que foi validado de fato

**#41 — `chrono` com feature `clock` acoplava `domain` a `wasm-bindgen`/`js-sys` no alvo wasm32, sem necessidade.** `domain` só usa `chrono::NaiveDate` (nenhum `Utc::now()`, nenhuma hora real) — mas a feature `clock`, habilitada por hábito no `Cargo.toml`, faz `chrono` depender de `wasm-bindgen`/`js-sys`/`futures-*` quando o alvo é `wasm32-unknown-unknown` (confirmado por download real da árvore de dependências: antes da correção, `cargo build --target wasm32-unknown-unknown` baixava `wasm-bindgen-macro`, `js-sys`, `futures-util` etc.; depois de trocar `clock` por `alloc`, só dependências normais de proc-macro). Isso violava a própria regra Hexagonal da §5 — capacidade de plataforma vazando pro core sem necessidade. **Corrigido**: `Cargo.toml` trocado para `features = ["alloc"]`, 20/20 testes + 4/4 doctests continuam passando nativamente, clippy limpo. Regra generalizada e registrada na §5: toda feature de dependência do core precisa ser checada contra o alvo wasm32, não só contra compilação nativa.

**#42 — Achado #19 (ADR-08) descrevia a mitigação errada para expansão de entidades XML.** A redação original presumia "teto configurável" no reader `quick-xml`. Verificado contra o changelog e issue tracker reais do crate (`tafia/quick-xml#258`, `#734`): `quick-xml` **não resolve** entidades customizadas de `DOCTYPE` por padrão — gera erro (`EscapeError::UnrecognizedSymbol`) em vez de expandir. Não existe "cap" para configurar porque não há expansão para limitar; o comportamento default já é a proteção. **Corrigido**: ADR-08 reescrito para refletir o mecanismo real — decisão passa a ser "não implementar entity resolver customizado" (a API existe mas é opt-in desde a `#615`), já que OFX 2.x não usa `DOCTYPE` com entidades próprias.

## O que não pôde ser validado — ambiental, não de código

**#43 — Este sandbox não consegue compilar para `wasm32-unknown-unknown`.** Sem `rustup` (só `rustc`/`cargo` via apt) e sem acesso aos domínios de distribuição do Rust (rede liberada só para `crates.io` e afins), não há como instalar o sysroot do alvo. `cargo build --target wasm32-unknown-unknown -p ofx-domain` falha com `can't find crate for core` — erro puramente de sysroot ausente, não de dependência ou código (a resolução de dependências para o alvo funciona normalmente, como o achado #41 demonstra). Registrado no roadmap (Fase 0) como primeira ação executável do próximo agente, num ambiente com toolchain completo.

---

## O que ficou de fora desta rodada, deliberadamente

Prototipar o tokenizer SGML (ADR-02) contra uma amostra real, medir tamanho de binário WASM real (ADR-07), ou validar `wasm-bindgen`/`serde-wasm-bindgen` além da resolução de dependência — todos exigem `parse`/`bindings` existirem ou o sysroot wasm32 disponível. Não são "mais uma rodada de validação", são a Fase 0/1/5 do roadmap — a linha que você pediu para não cruzar neste chat.

---

## Continuação (mesma sessão) — achados #44–47

**#44 — `Cargo.lock` nunca teve política decidida no Discovery, apesar de mencionado em chat numa rodada anterior.** Verificado por `grep`: zero ocorrências no documento. Decisão fechada agora em ADR-10: commitar na raiz do workspace (reprodutibilidade do orçamento de binário do ADR-07 e do CI), sem prejuízo à publicação futura de `domain`/`parse`/`analytics` no crates.io — que sempre resolve dependências frescas para quem consome, ignorando o lockfile de quem publica.

**#45 — `wee_alloc` estava correto na conclusão, mas sem fonte.** Verificado via busca real: `RUSTSEC-2022-0054` (advisory formal, "unmaintained"), repositório `rustwasm/wee_alloc` arquivado pelo dono em 25/08/2025, bug de corrupção de memória (#105/#106) nunca corrigido. ADR-07 atualizada para citar a evidência em vez de "sem manutenção" genérico.

**#46 — `rust_decimal` tem feature `wasm` opcional (dependente de `wasm-bindgen`), não documentada.** Confirmado via docs oficiais: expõe `Decimal` com conversões diretas atravessando `#[wasm_bindgen]`. Não habilitar em `domain` — reintroduziria o acoplamento que o achado #41 removeu do `chrono`. Registrado em ADR-06 como opção legítima só para `bindings`, decisão adiada para a Fase 5.

**#47 — A proibição de `println!`/`eprintln!` no core (§5) tinha só justificativa arquitetural, sem o fato concreto.** Confirmado contra a doc oficial do target: em `wasm32-unknown-unknown`, `println!` não escreve em lugar nenhum — descarta em silêncio, sem erro. A regra deixa de ser só estilo e passa a ser "a diferença entre ver o diagnóstico e perdê-lo sem aviso".

Todos os quatro fechados no documento nesta mesma sessão — nenhum ficou como pergunta em aberto, porque nenhum dependia de julgamento seu: eram fatos verificáveis (fonte externa ou grep no próprio documento), não decisões de produto.

