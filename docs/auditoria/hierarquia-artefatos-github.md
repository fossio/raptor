# Hierarquia de artefatos GitHub — `ofx`

Simulação estrutural derivada das ADR-10/11/12/13 (`discovery-ofx-rust-wasm.md`) e dos achados de auditoria fechados (`fechamento-auditoria.md`, achados #1–#85). Não é a lista literal de issues a criar — é o esqueleto organizacional (repos → labels → milestones → buckets de issue) que o roadmap de 7 fases já decompõe. O texto de cada issue vem de `alteracoes-adr.md` no momento da criação real; aqui só o `type:`/`area:`/referência ao achado importam.

`type:`, `priority:`, `effort:`, `needs:` têm só o prefixo fechado na ADR-12 — os valores abaixo são proposta consistente com `prefixo:valor`, não decisão travada. `area:` é o único eixo com valores já fechados (nomes de crate).

```
{seu-usuário} (conta GitHub)
│
├── .github  (repo público — defaults herdados por todo repo pessoal sem override)
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml
│   │   └── feature_request.yml
│   ├── CONTRIBUTING.md
│   ├── SECURITY.md
│   └── CODE_OF_CONDUCT.md
│
└── ofx  (repo do projeto)
    │
    ├── Labels  (criadas manualmente via UI/gh — sem sync via action, ADR-12)
    │   ├── type:    bug | feature | chore | docs | refactor | decision
    │   ├── priority: p0 | p1 | p2
    │   ├── area:    domain | parse | analytics | wasm        ← único eixo fechado
    │   ├── effort:  xs | s | m | l | xl
    │   └── needs:   decision | discussion | repro
    │
    ├── ISSUE_TEMPLATE/  (override local — só o que diverge do herdado, ADR-10)
    │   └── ofx_parse_failure.yml   — form dedicado a "arquivo OFX que falha no parsing"
    │
    ├── Milestones = Fases do roadmap (§10/11, reaproveitadas — não recriadas)
    │
    │   ├── Fase 0 — Workspace + domain (Banking/CreditCard/Investments) + parser XML 2.x
    │   │   ├── [decision][needs:decision][domain]  achado #48 — localização de Diagnostic/DiagnosticCode
    │   │   ├── [decision][needs:decision][domain]  achado #58 — conformidade LGPL-2.1, linkagem estática em Rust
    │   │   ├── [decision][needs:decision][domain]  achado #76 — modelagem de parcelamento
    │   │   ├── [feature][domain]   Banking/CreditCard/Investments message sets
    │   │   ├── [feature][parse]    parser XML 2.x completo
    │   │   ├── [chore]             LICENSE, README.md, .gitignore, rustfmt.toml
    │   │   └── [chore]             CLAUDE.md inicial (ADR-11)
    │   │
    │   ├── Fase 1 — Parser SGML 1.x + normalização
    │   │   ├── [feature][parse]    tokenizer SGML + tabela de aridade
    │   │   ├── [bug][parse]        achado #6  — normalização de timezone
    │   │   ├── [bug][parse]        achado #18 — normalização de vírgula decimal
    │   │   ├── [decision][needs:decision] achado #66 — semântica dupla de datas (UTC vs. local)
    │   │   ├── [chore]             SECURITY.md
    │   │   └── [chore]             hook PreToolUse anti-commit de dado bancário real (achado #4)
    │   │
    │   ├── Fase 2 — Refinamento Credit Card / Investments message sets
    │   │   └── [refactor][domain]  achados #2/#7/#13 — invariantes de domain revisitados
    │   │
    │   ├── Fase 3 — Analytics de agregação + predictive
    │   │   ├── [feature][analytics] NCF, ADB, AT, saldos
    │   │   ├── [feature][analytics] predictive (BR, Runway)
    │   │   ├── [bug][analytics]     dedup key — (AccountId, Fitid) composto, não Fitid isolado
    │   │   └── [decision][needs:decision] achado #51 — multi-moeda em escopo consolidado
    │   │
    │   ├── Fase 4 — Analytics de retorno e risco
    │   │   ├── [feature][analytics] TWR, IRR, Sharpe, VaR, MDD
    │   │   └── [bug][analytics]     contradição grid diário vs. anualização √252
    │   │
    │   ├── Fase 5 — wasm + demo HTML offline
    │   │   ├── [feature][wasm]      bindings + contrato free() (achado #20)
    │   │   ├── [bug][wasm]          achado #9 — limite de input
    │   │   ├── [chore]              congelar contrato público (achados #12/#24)
    │   │   ├── [chore]              CHANGELOG.md a partir daqui
    │   │   └── [chore]              .claude/rules/wasm-bindings.md
    │   │
    │   └── Fase 6 — Hardening pré-lançamento
    │       ├── [feature]            corpus sintético por dialeto (achado #4)
    │       ├── [chore]              twiggy + otimização de binário
    │       ├── [feature]            PWA
    │       └── [chore]              CONTRIBUTING.md/CODE_OF_CONDUCT.md locais só se divergirem do herdado;
    │                                 RELEASING.md; Dependabot; checagem de NOTICE
    │
    └── Não criado agora (ADR-12 — mesmo filtro de ADR-10/11)
        ├── Issue Types nativos       (exige org)
        ├── GitHub Projects v2        (fluxo de time, sem contribuidor além do mantenedor)
        ├── labels.yml sincronizado   (sem volume que justifique)
        └── sub-issues/hierarquia    (7 fases já são a decomposição épico→execução)
```

## Notas de rastreabilidade

- **Fase −1** (fechamento de decisões bloqueantes de `domain`) não vira milestone — já concluída antes do repositório existir no GitHub. Fica só como referência em `fechamento-auditoria.md`.
- Issues de `[decision][needs:decision]` bloqueiam o restante da fase onde aparecem — abrir e resolver antes de abrir as issues de `[feature]`/`[bug]` dependentes da mesma fase.
- `area:` não recebe granularidade de módulo interno de `domain` (`cashflow`, `predictive`, `dedupe`, `metric`, `money`) — critério já fechado, evita que a label dependa de onde um arquivo mora hoje.
- Ao criar as issues de verdade, o corpo referencia o número do achado em `alteracoes-adr.md` — não duplica a análise no corpo da issue.
