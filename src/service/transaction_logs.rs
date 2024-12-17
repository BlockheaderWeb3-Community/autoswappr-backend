use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct TransactionLog {
    pub wallet_address: String,
    pub from_token: String,
    pub to_token: String,
    pub percentage: u16,
    pub amount_from: u64,
    pub amount_to: u64,
}

impl TransactionLog {
    fn new(
        wallet_address: &str,
        from_token: &str,
        to_token: &str,
        percentage: u16,
        amount_from: u64,
        amount_to: u64,
    ) -> Self {
        Self {
            wallet_address: wallet_address.to_string(),
            from_token: from_token.to_string(),
            to_token: to_token.to_string(),
            percentage,
            amount_from,
            amount_to,
        }
    }

    pub fn validate(&mut self) -> Result<(), String> {
        self.validate_address(&self.wallet_address)?;
        self.validate_address(&self.from_token)
            .map_err(|_| "Invalid from_token")?;
        self.validate_address(&self.to_token)
            .map_err(|_| "Invalid  to_token")?;
        self.validate_percentage(self.percentage)?;
        self.validate_amount(self.amount_to)?;
        self.validate_amount(self.amount_from)?;
        Ok(())
    }

    fn validate_address(&self, address: &str) -> Result<(), String> {
        match address.starts_with("0x")
            && address.len() == 42
            && address[2..].chars().all(|c| c.is_ascii_hexdigit())
        {
            true => Ok(()),
            false => Err("Invalid Address".to_owned()),
        }
    }

    fn validate_percentage(&self, percentage: u16) -> Result<(), String> {
        match percentage == 0 && percentage > 100 {
            true => Err(String::from("Invalid percentage")),
            false => Ok(()),
        }
    }

    fn validate_amount(&self, _amount: u64) -> Result<(), String> {
        Ok(())
    }

    async fn save(&mut self, db: &PgPool) -> Result<(), String> {
        self.validate().map_err(|_| "Transaction log is invalid")?;
        let i_percentage = self.percentage as i16;
        let i_amount_from = self.amount_from as i64;
        let i_amount_to = self.amount_to as i64;
        sqlx::query(
            r#"INSERT INTO transactions_log (wallet_address, from_token, to_token, percentage, amount_from, amount_to)
        VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(&self.wallet_address)
        .bind(&self.from_token)
        .bind(&self.to_token)
        .bind(i_percentage)
        .bind(i_amount_from)
        .bind(i_amount_to)
        .execute(db)
        .await.expect("Failed to save transaction log to db");
        Ok(())
    }
}

pub async fn log_transaction(
    wallet_address: &str,
    from_token: &str,
    to_token: &str,
    percentage: u16,
    amount_from: u64,
    amount_to: u64,
    db: &PgPool,
) -> Result<TransactionLog, String> {
    let mut tx = TransactionLog::new(
        wallet_address,
        from_token,
        to_token,
        percentage,
        amount_from,
        amount_to,
    );
    tx.save(db).await?;
    Ok(tx)
}
