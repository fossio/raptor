//! Raptor domain layer. Scaffold — os módulos reais (money, diagnostic,
//! dedupe, consolidation, cashflow, predictive, metric) entram numa issue
//! dedicada, seguindo `docs/discovery-ofx-rust-wasm.md`.

/// ```
/// assert_eq!(raptor_domain::PLACEHOLDER, "raptor-domain");
/// ```
pub const PLACEHOLDER: &str = "raptor-domain";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_is_stable() {
        assert_eq!(PLACEHOLDER, "raptor-domain");
    }
}
