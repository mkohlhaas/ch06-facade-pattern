// Crypto Wallet Subsystem
//
// Imagine a crypto wallet operation that requires verifying an account, updating an internal
// database, notifying a user, and appending to a secure ledger. Instead of making the client handle
// all these steps, a WalletFacade manages the interaction.

// ================================//
// 1. Complex Subsystem Components //
// ================================//

struct AccountVerification;
impl AccountVerification {
    fn verify(&self, account_id: &str) -> bool {
        println!("Verifying account: {}", account_id);
        true
    }
}

struct Database;
impl Database {
    fn update_balance(&self, account_id: &str, amount: f64) {
        println!(
            "Updating database balance for {} by ${}",
            account_id, amount
        );
    }
}

struct Ledger;
impl Ledger {
    fn record_transaction(&self, account_id: &str, amount: f64) {
        println!(
            "Logging ${} transaction to ledger for {}",
            amount, account_id
        );
    }
}

// ==============//
// 2. The Facade //
// ==============//

pub struct WalletFacade {
    verifier: AccountVerification,
    db: Database,
    ledger: Ledger,
}

impl WalletFacade {
    pub fn new() -> Self {
        Self {
            verifier: AccountVerification,
            db: Database,
            ledger: Ledger,
        }
    }

    // High-level, simplified entry point for the client
    pub fn deposit_funds(&self, account_id: &str, amount: f64) -> Result<(), &'static str> {
        if !self.verifier.verify(account_id) {
            return Err("Verification failed");
        }
        self.db.update_balance(account_id, amount);
        self.ledger.record_transaction(account_id, amount);
        Ok(())
    }
}

impl Default for WalletFacade {
    fn default() -> Self {
        Self::new()
    }
}

// ======//
// Usage //
// ======//

fn main() {
    // The client only interacts with the unified Facade API
    let wallet_system = WalletFacade::default();

    match wallet_system.deposit_funds("user_123", 250.00) {
        Ok(()) => println!("Deposit successful!"),
        Err(e) => println!("Error: {}", e),
    }
}

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_funds_success() {
        let wallet = WalletFacade::default();
        assert!(wallet.deposit_funds("user_123", 250.0).is_ok());
    }

    #[test]
    fn test_deposit_funds_small_amount() {
        let wallet = WalletFacade::default();
        assert!(wallet.deposit_funds("user_456", 10.5).is_ok());
    }
}
