//! `raptor-domain` — núcleo estável do workspace (ADR-01, ADR-04).
//!
//! Não depende de nenhuma outra crate do workspace. Os módulos abaixo
//! fecharam os achados #1/#2/#7/#10/#13/#14/#15 (ver
//! `docs/auditoria/fechamento-auditoria.md` e
//! `docs/auditoria/alteracoes-adr.md`) antes de o modelo de domínio
//! completo (`Document`/`Account`/`Transaction`, issue #40) existir — por
//! isso operam sobre tipos independentes (`Money`, extratores genéricos)
//! em vez do modelo de domínio inteiro.

pub mod cashflow;
pub mod consolidation;
pub mod dedupe;
pub mod metric;
pub mod money;
pub mod predictive;
