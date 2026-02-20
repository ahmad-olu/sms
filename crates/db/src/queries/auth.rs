use crate::error::DbResult;

pub struct AuthQ;

impl AuthQ {
    /// Check if email is already taken in a school
    pub async fn email_exists(school_id: String, email: String) -> DbResult<bool> {
        todo!()
    }

    /// Check if phone is already taken in a school
    pub async fn phone_exists(school_id: String, phone: String) -> DbResult<bool> {
        todo!()
    }

    /// Check if admission number is already taken in a school
    pub async fn admission_number_exists(
        school_id: String,
        admission_number: String,
    ) -> DbResult<bool> {
        todo!()
    }

    /// Verify user password hash (fetch hash for comparison)
    pub async fn get_password_hash(user_id: String) -> DbResult<Option<String>> {
        todo!()
    }

    /// Update password hash
    pub async fn update_password(user_id: String, new_hash: String) -> DbResult<()> {
        todo!()
    }

    /// Mark email as verified
    pub async fn verify_email(user_id: String) -> DbResult<()> {
        todo!()
    }

    /// Mark phone as verified
    pub async fn verify_phone(user_id: String) -> DbResult<()> {
        todo!()
    }
}
