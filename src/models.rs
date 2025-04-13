use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = visitors)]
pub struct Visitor {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub consent_given: bool,
    pub consent_updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: Uuid,
    pub visitor_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = events)]
pub struct Event {
    pub id: Uuid,
    pub session_id: Uuid,
    pub event_type: String,
    pub path: String,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

// New visitor creation
#[derive(Debug, Insertable)]
#[diesel(table_name = visitors)]
pub struct NewVisitor {
    pub id: Uuid,
    pub consent_given: bool,
}

// New session creation
#[derive(Debug, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id: Uuid,
    pub visitor_id: Uuid,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

// New event creation
#[derive(Debug, Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub id: Uuid,
    pub session_id: Uuid,
    pub event_type: String,
    pub path: String,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub ip_address: Option<String>,
    pub metadata: serde_json::Value,
} 
