use serde::{Deserialize, Serialize};
use state_repository::ModelKey;
use workflow::{Flow, Transfert};

pub struct Payment {}

impl Flow for Payment {
    type Input = PaymentPay;
    type Output = PaymentPaid;
}

pub struct AskPayment {}

impl Flow for AskPayment {
    type Input = PaymentPaid;
    type Output = PaymentPay;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaymentPay {
    pub amount: u32,
    pub target: ModelKey,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PaymentPaid {
    Paid(u32, ModelKey),
    NotPaid(ModelKey),
}

impl Transfert for PaymentPay {
    fn transfert_name(&self) -> &str {
        "pay"
    }

    fn transfert_names() -> Vec<&'static str> {
        vec!["pay"]
    }

    fn target(&self) -> &ModelKey {
        &self.target
    }
}

impl Transfert for PaymentPaid {
    fn transfert_name(&self) -> &str {
        use PaymentPaid::*;
        match self {
            Paid(_, _) => "paid",
            NotPaid(_) => "not_paid",
        }
    }

    fn transfert_names() -> Vec<&'static str> {
        vec!["paid", "not_paid"]
    }

    fn target(&self) -> &ModelKey {
        use PaymentPaid::*;
        match self {
            Paid(_, t) => t,
            NotPaid(t) => t,
        }
    }
}
