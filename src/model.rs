use crate::generate_random_string;
use bcrypt::Version;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use rusoto_dynamodb::{AttributeValue, GetItemInput, GetItemOutput, PutItemInput};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const TABLE_CODE: &str = "AtCoderAuthCode";
const TABLE_TOKEN: &str = "AtCoderAuthToken";
const EXPIRATION_SECONDS: u64 = 300;

pub struct VerificationCode {
    user_id: String,
    verification_code: String,
    expired_at: u64,
}

impl VerificationCode {
    pub fn new(user_id: &str, verification_code: &str) -> Self {
        let unix_second = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            user_id: user_id.to_string(),
            verification_code: verification_code.to_string(),
            expired_at: unix_second + EXPIRATION_SECONDS,
        }
    }
    pub fn get_item_input(user_id: &str) -> GetItemInput {
        let mut query = HashMap::new();
        query.insert(
            String::from("user_id"),
            AttributeValue {
                s: Some(user_id.to_string()),
                ..Default::default()
            },
        );
        GetItemInput {
            table_name: TABLE_CODE.to_string(),
            key: query,
            ..Default::default()
        }
    }
    pub fn get_verification_code(get_item_output: &GetItemOutput) -> Option<String> {
        let item = get_item_output.item.as_ref()?;
        let verification_code = item.get("verification_code")?.s.as_ref()?;
        Some(verification_code.to_string())
    }

    pub fn to_put_item_input(&self) -> PutItemInput {
        let mut item = HashMap::new();
        item.insert(
            String::from("user_id"),
            AttributeValue {
                s: Some(self.user_id.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("verification_code"),
            AttributeValue {
                s: Some(self.verification_code.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("expired_at"),
            AttributeValue {
                n: Some(self.expired_at.to_string()),
                ..Default::default()
            },
        );
        PutItemInput {
            item,
            table_name: String::from(TABLE_CODE),
            ..Default::default()
        }
    }
}

pub struct AuthToken {
    user_id: String,
    hash: String,
}

impl AuthToken {
    pub fn new(user_id: &str, token: &str) -> Self {
        let cost = get_random_cost();
        let salt = generate_random_string(16);
        let parts = bcrypt::hash_with_salt(token, cost, salt.as_bytes()).unwrap();

        Self {
            user_id: user_id.to_string(),
            hash: parts.format_for_version(Version::TwoB),
        }
    }
    pub fn get_item_input(user_id: &str) -> GetItemInput {
        let mut query = HashMap::new();
        query.insert(
            String::from("user_id"),
            AttributeValue {
                s: Some(user_id.to_string()),
                ..Default::default()
            },
        );
        GetItemInput {
            table_name: TABLE_TOKEN.to_string(),
            key: query,
            ..Default::default()
        }
    }
    pub fn get_hash(get_item_output: &GetItemOutput) -> Option<String> {
        let item = get_item_output.item.as_ref()?;
        let hash = item.get("hash")?.s.as_ref()?;
        Some(hash.to_string())
    }
    pub fn to_put_item_input(&self) -> PutItemInput {
        let mut item = HashMap::new();
        item.insert(
            String::from("user_id"),
            AttributeValue {
                s: Some(self.user_id.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("hash"),
            AttributeValue {
                s: Some(self.hash.clone()),
                ..Default::default()
            },
        );
        PutItemInput {
            item,
            table_name: String::from(TABLE_TOKEN),
            ..Default::default()
        }
    }
}

fn get_random_cost() -> u32 {
    thread_rng().sample(Uniform::from(10..14))
}
