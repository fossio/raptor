//! Classificação de cashflow (achado #13, ADR-05): OFX não separa fluxo de
//! capital de performance — comprar uma ação não é rendimento, é realocação
//! de capital dentro da carteira. É a distinção que `returns`/`risk`
//! precisam para TWR/IRR/HPR/CAGR pararem de confundir aporte com
//! performance.
//!
//! Este é um Domain Service (DDD, §5 do Discovery): conhecimento de negócio
//! sobre o *significado* de um lançamento, não cálculo estatístico — por
//! isso mora em `domain`, não em `analytics`.
//!
//! `Transaction`/`Account` ainda não existem em código (Milestone 0, issue
//! #40) — por isso a classificação opera sobre os subtipos `TRNTYPE`/
//! `INVTRAN` diretamente, sem depender do modelo de domínio completo.

use crate::money::Money;

/// O papel de um lançamento no fluxo de caixa de uma carteira/conta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CashflowKind {
    /// Cruza a fronteira da carteira/conta: aporte ou retirada do investidor.
    /// Marca a quebra de sub-período para TWR.
    External,
    /// Rendimento ou ganho gerado *dentro* da carteira (juros, dividendos,
    /// ganho de capital realizado).
    Performance,
    /// Realocação de capital que não entra nem sai da carteira e não é
    /// rendimento — ex.: comprar uma ação move dinheiro de caixa para
    /// posição, sem ser aporte nem performance.
    Neutral,
}

/// O quanto a classificação é uma leitura direta do spec OFX (`High`) vs.
/// uma heurística sobre um `TRNTYPE` genérico demais para decidir sem
/// ambiguidade (`Low`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    High,
    Low,
}

/// Um valor monetário já classificado quanto ao papel no fluxo de caixa —
/// o que `analytics::returns`/`risk` consomem no lugar de `TRNAMT`/`BALAMT`
/// bruto (achado #52).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClassifiedEvent {
    pub amount: Money,
    pub kind: CashflowKind,
    pub confidence: Confidence,
}

/// Subconjunto de valores `<TRNTYPE>` do Message Set Banking (spec OFX).
/// `Unknown` preserva o dado bruto para valores fora do spec (nunca perde
/// informação silenciosamente).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BankingTrnType {
    Credit,
    Debit,
    Int,
    Div,
    Fee,
    Srvchg,
    Dep,
    Atm,
    Pos,
    Xfer,
    Check,
    Payment,
    Cash,
    DirectDep,
    DirectDebit,
    RepeatPmt,
    Other,
    Unknown(String),
}

/// Classifica um lançamento Banking pelo `TRNTYPE`.
///
/// ```
/// use raptor_domain::cashflow::{classify_banking, BankingTrnType, CashflowKind, Confidence};
/// use raptor_domain::money::{Money, CurrencyCode};
/// use rust_decimal::Decimal;
///
/// let amount = Money::new(Decimal::new(-5000, 2), CurrencyCode::BRL);
/// let event = classify_banking(&BankingTrnType::Xfer, amount);
/// assert_eq!(event.kind, CashflowKind::Neutral);
/// assert_eq!(event.confidence, Confidence::High);
/// ```
pub fn classify_banking(trn_type: &BankingTrnType, amount: Money) -> ClassifiedEvent {
    let (kind, confidence) = match trn_type {
        // Transferência entre contas do próprio usuário: não entra nem sai
        // da carteira consolidada, não é rendimento.
        BankingTrnType::Xfer => (CashflowKind::Neutral, Confidence::High),
        // Rendimento sobre saldo em conta.
        BankingTrnType::Int | BankingTrnType::Div => (CashflowKind::Performance, Confidence::High),
        // Custo — não é aporte/retirada do investidor nem performance da
        // carteira, mas para o caminho Banking (sem conceito de TWR) o
        // tratamento prático é o mesmo de um fluxo externo.
        BankingTrnType::Fee | BankingTrnType::Srvchg => (CashflowKind::External, Confidence::High),
        // Movimentos inequívocos de/para fora da conta.
        BankingTrnType::Dep
        | BankingTrnType::Atm
        | BankingTrnType::Pos
        | BankingTrnType::Check
        | BankingTrnType::Payment
        | BankingTrnType::Cash
        | BankingTrnType::DirectDep
        | BankingTrnType::DirectDebit
        | BankingTrnType::RepeatPmt => (CashflowKind::External, Confidence::High),
        // Genéricos demais para decidir sem ambiguidade — heurística fraca,
        // não leitura direta do spec (achado #13).
        BankingTrnType::Credit | BankingTrnType::Debit | BankingTrnType::Other => {
            (CashflowKind::External, Confidence::Low)
        }
        BankingTrnType::Unknown(_) => (CashflowKind::External, Confidence::Low),
    };
    ClassifiedEvent {
        amount,
        kind,
        confidence,
    }
}

/// Subtipo `<INCOMETYPE>` do bloco `INCOME` (Message Set Investments).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomeType {
    CapitalGainsShort,
    CapitalGainsLong,
    Dividend,
    Interest,
    Misc,
}

/// Subconjunto de valores `<INVTRAN>`/subtipo do Message Set Investments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvestmentTranType {
    BuyStock,
    SellStock,
    BuyDebt,
    SellDebt,
    BuyMf,
    SellMf,
    BuyOpt,
    SellOpt,
    BuyOther,
    SellOther,
    Income(IncomeType),
    ReinvestIncome,
    TransferIn,
    TransferOut,
    Split,
    Other,
}

/// Classifica um lançamento de investimento pelo subtipo `INVTRAN`/`INCOME`.
///
/// O caso que mais importa: comprar uma ação é `Neutral`, não `Performance`
/// — é exatamente a distinção que faltava para TWR/IRR/HPR/CAGR pararem de
/// confundir aporte com performance (achado #13).
///
/// ```
/// use raptor_domain::cashflow::{classify_investment, InvestmentTranType, CashflowKind};
/// use raptor_domain::money::{Money, CurrencyCode};
/// use rust_decimal::Decimal;
///
/// let amount = Money::new(Decimal::new(-100000, 2), CurrencyCode::BRL);
/// let event = classify_investment(&InvestmentTranType::BuyStock, amount);
/// assert_eq!(event.kind, CashflowKind::Neutral); // compra != rendimento
/// ```
pub fn classify_investment(trn_type: &InvestmentTranType, amount: Money) -> ClassifiedEvent {
    let (kind, confidence) = match trn_type {
        // Realocação de capital dentro da carteira: não é aporte, não é
        // performance. O caso central do achado #13.
        InvestmentTranType::BuyStock
        | InvestmentTranType::SellStock
        | InvestmentTranType::BuyDebt
        | InvestmentTranType::SellDebt
        | InvestmentTranType::BuyMf
        | InvestmentTranType::SellMf
        | InvestmentTranType::BuyOpt
        | InvestmentTranType::SellOpt
        | InvestmentTranType::BuyOther
        | InvestmentTranType::SellOther
        | InvestmentTranType::Split => (CashflowKind::Neutral, Confidence::High),
        // Rendimento gerado dentro da carteira, ainda que reinvestido
        // automaticamente em mais unidades.
        InvestmentTranType::Income(_) | InvestmentTranType::ReinvestIncome => {
            (CashflowKind::Performance, Confidence::High)
        }
        // Aporte/retirada do investidor: cruza a fronteira da carteira,
        // quebra sub-período de TWR.
        InvestmentTranType::TransferIn | InvestmentTranType::TransferOut => {
            (CashflowKind::External, Confidence::High)
        }
        InvestmentTranType::Other => (CashflowKind::External, Confidence::Low),
    };
    ClassifiedEvent {
        amount,
        kind,
        confidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::money::CurrencyCode;
    use rust_decimal::Decimal;

    fn brl(amount: i64) -> Money {
        Money::new(Decimal::new(amount, 2), CurrencyCode::BRL)
    }

    #[test]
    fn compra_de_acao_e_neutral_nao_performance() {
        let event = classify_investment(&InvestmentTranType::BuyStock, brl(-10000));
        assert_eq!(event.kind, CashflowKind::Neutral);
        assert_eq!(event.confidence, Confidence::High);
    }

    #[test]
    fn venda_de_fundo_e_neutral() {
        let event = classify_investment(&InvestmentTranType::SellMf, brl(5000));
        assert_eq!(event.kind, CashflowKind::Neutral);
    }

    #[test]
    fn dividendo_e_performance() {
        let event =
            classify_investment(&InvestmentTranType::Income(IncomeType::Dividend), brl(150));
        assert_eq!(event.kind, CashflowKind::Performance);
        assert_eq!(event.confidence, Confidence::High);
    }

    #[test]
    fn transferencia_de_entrada_e_external() {
        let event = classify_investment(&InvestmentTranType::TransferIn, brl(100000));
        assert_eq!(event.kind, CashflowKind::External);
    }

    #[test]
    fn investimento_other_e_baixa_confianca() {
        let event = classify_investment(&InvestmentTranType::Other, brl(10));
        assert_eq!(event.confidence, Confidence::Low);
    }

    #[test]
    fn juros_bancarios_e_performance() {
        let event = classify_banking(&BankingTrnType::Int, brl(42));
        assert_eq!(event.kind, CashflowKind::Performance);
    }

    #[test]
    fn transferencia_bancaria_e_neutral() {
        let event = classify_banking(&BankingTrnType::Xfer, brl(-30000));
        assert_eq!(event.kind, CashflowKind::Neutral);
    }

    #[test]
    fn credit_generico_e_baixa_confianca() {
        let event = classify_banking(&BankingTrnType::Credit, brl(100));
        assert_eq!(event.kind, CashflowKind::External);
        assert_eq!(event.confidence, Confidence::Low);
    }

    #[test]
    fn debit_generico_e_baixa_confianca() {
        let event = classify_banking(&BankingTrnType::Debit, brl(-100));
        assert_eq!(event.confidence, Confidence::Low);
    }

    #[test]
    fn tarifa_bancaria_e_external_alta_confianca() {
        let event = classify_banking(&BankingTrnType::Fee, brl(-1090));
        assert_eq!(event.kind, CashflowKind::External);
        assert_eq!(event.confidence, Confidence::High);
    }

    #[test]
    fn unknown_preserva_dado_bruto_e_baixa_confianca() {
        let event = classify_banking(&BankingTrnType::Unknown("XPTO".to_string()), brl(1));
        assert_eq!(event.confidence, Confidence::Low);
    }
}
