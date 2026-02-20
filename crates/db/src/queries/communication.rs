use crate::error::DbResult;
use shared::models::{
    communication::{Announcement, Event, EventRsvp, Message},
    system_and_audit::{Notification, SmsLog},
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::{SurrealValue, Value};

const ANNOUNCEMENT_TABLE: &str = "announcements";
const MESSAGE_TABLE: &str = "messages";
const EVENT_TABLE: &str = "events";
const EVENT_RSVP_TABLE: &str = "event_rsvps";
const NOTIFICATION_TABLE: &str = "notifications";
const SMS_LOG_TABLE: &str = "sms_log";

#[derive(Debug, SurrealValue)]
struct CountTotal {
    total: Option<u64>,
}

#[derive(Debug, SurrealValue)]
struct MonthlyCost {
    total_cost: Option<f64>,
}

pub struct AnnouncementQ;

impl AnnouncementQ {
    /// Create an announcement
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: Announcement,
    ) -> DbResult<Option<Announcement>> {
        let announcement: Option<Announcement> =
            sdb.create(ANNOUNCEMENT_TABLE).content(data).await?;
        Ok(announcement)
    }

    /// Get published announcements for a school
    pub async fn get_published(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        limit: Option<u32>,
    ) -> DbResult<Vec<Announcement>> {
        let limit = limit.unwrap_or(20);
        let announcements: Vec<Announcement> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND published = true
                ORDER BY created_at DESC
                LIMIT $limit
                "#,
            )
            .bind(("table", ANNOUNCEMENT_TABLE))
            .bind(("school_id", school_id))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(announcements)
    }

    /// Publish an announcement
    pub async fn publish(
        &self,
        sdb: &Surreal<Client>,
        announcement_id: String,
    ) -> DbResult<Option<Announcement>> {
        let announcement: Option<Announcement> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    published = true,
                    published_at = time::now(),
                    updated_at = time::now()
                "#,
            )
            .bind(("table", ANNOUNCEMENT_TABLE))
            .bind(("id", announcement_id))
            .await?
            .take(0)?;
        Ok(announcement)
    }
}

pub struct MessageQ;

impl MessageQ {
    /// Send a message
    pub async fn send(&self, sdb: &Surreal<Client>, data: Message) -> DbResult<Option<Message>> {
        let message: Option<Message> = sdb.create(MESSAGE_TABLE).content(data).await?;
        Ok(message)
    }

    /// Mark a message as read
    pub async fn mark_read(
        &self,
        sdb: &Surreal<Client>,
        message_id: String,
    ) -> DbResult<Option<Message>> {
        let message: Option<Message> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    read = true,
                    read_at = time::now()
                "#,
            )
            .bind(("table", MESSAGE_TABLE))
            .bind(("id", message_id))
            .await?
            .take(0)?;
        Ok(message)
    }

    /// Get unread messages for a user
    pub async fn get_unread(
        &self,
        sdb: &Surreal<Client>,
        user_id: String,
    ) -> DbResult<Vec<Message>> {
        let messages: Vec<Message> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE recipient_id = type::thing('users', $user_id)
                AND read = false
                ORDER BY sent_at DESC
                "#,
            )
            .bind(("table", MESSAGE_TABLE))
            .bind(("user_id", user_id))
            .await?
            .take(0)?;
        Ok(messages)
    }

    /// Get conversation thread between two users
    pub async fn get_thread(
        &self,
        sdb: &Surreal<Client>,
        user1_id: String,
        user2_id: String,
    ) -> DbResult<Vec<Message>> {
        let messages: Vec<Message> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE (
                    sender_id = type::thing('users', $user1_id)
                    AND recipient_id = type::thing('users', $user2_id)
                ) OR (
                    sender_id = type::thing('users', $user2_id)
                    AND recipient_id = type::thing('users', $user1_id)
                )
                ORDER BY sent_at ASC
                "#,
            )
            .bind(("table", MESSAGE_TABLE))
            .bind(("user1_id", user1_id))
            .bind(("user2_id", user2_id))
            .await?
            .take(0)?;
        Ok(messages)
    }

    /// Count unread messages for a user
    pub async fn count_unread(&self, sdb: &Surreal<Client>, user_id: String) -> DbResult<u64> {
        let mut response = sdb
            .query(
                r#"
                SELECT count() AS total
                FROM type::table($table)
                WHERE recipient_id = type::thing('users', $user_id)
                AND read = false
                "#,
            )
            .bind(("table", MESSAGE_TABLE))
            .bind(("user_id", user_id))
            .await?;

        let result: Option<CountTotal> = response.take(0)?;
        Ok(result.and_then(|r| r.total).unwrap_or(0))
    }
}

pub struct EventQ;

impl EventQ {
    /// Create an event
    pub async fn create(&self, sdb: &Surreal<Client>, data: Event) -> DbResult<Option<Event>> {
        let event: Option<Event> = sdb.create(EVENT_TABLE).content(data).await?;
        Ok(event)
    }

    /// Get upcoming events for a school
    pub async fn get_upcoming(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<Event>> {
        let events: Vec<Event> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND event_date >= time::today()
                ORDER BY event_date ASC
                "#,
            )
            .bind(("table", EVENT_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(events)
    }

    /// RSVP to an event
    pub async fn rsvp(
        &self,
        sdb: &Surreal<Client>,
        data: EventRsvp,
    ) -> DbResult<Option<EventRsvp>> {
        let rsvp: Option<EventRsvp> = sdb.create(EVENT_RSVP_TABLE).content(data).await?;
        Ok(rsvp)
    }

    /// Get RSVP summary for an event
    pub async fn get_rsvp_summary(
        &self,
        sdb: &Surreal<Client>,
        event_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT response, count() AS count
                FROM type::table($table)
                WHERE event_id = type::thing('events', $event_id)
                GROUP BY response
                "#,
            )
            .bind(("table", EVENT_RSVP_TABLE))
            .bind(("event_id", event_id))
            .await?
            .take(0)?;
        Ok(result)
    }
}

pub struct NotificationQ;

impl NotificationQ {
    /// Create a notification
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: Notification,
    ) -> DbResult<Option<Notification>> {
        let notification: Option<Notification> =
            sdb.create(NOTIFICATION_TABLE).content(data).await?;
        Ok(notification)
    }

    /// Get unread notifications for a user
    pub async fn get_unread(
        &self,
        sdb: &Surreal<Client>,
        user_id: String,
    ) -> DbResult<Vec<Notification>> {
        let notifications: Vec<Notification> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE user_id = type::thing('users', $user_id)
                AND read = false
                ORDER BY sent_at DESC
                "#,
            )
            .bind(("table", NOTIFICATION_TABLE))
            .bind(("user_id", user_id))
            .await?
            .take(0)?;
        Ok(notifications)
    }

    /// Mark a notification as read
    pub async fn mark_read(
        &self,
        sdb: &Surreal<Client>,
        notification_id: String,
    ) -> DbResult<Option<Notification>> {
        let notification: Option<Notification> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    read = true,
                    read_at = time::now()
                "#,
            )
            .bind(("table", NOTIFICATION_TABLE))
            .bind(("id", notification_id))
            .await?
            .take(0)?;
        Ok(notification)
    }

    /// Mark all notifications as read for a user
    pub async fn mark_all_read(
        &self,
        sdb: &Surreal<Client>,
        user_id: String,
    ) -> DbResult<Vec<Notification>> {
        let notifications: Vec<Notification> = sdb
            .query(
                r#"
                UPDATE type::table($table) SET
                    read = true,
                    read_at = time::now()
                WHERE user_id = type::thing('users', $user_id)
                AND read = false
                "#,
            )
            .bind(("table", NOTIFICATION_TABLE))
            .bind(("user_id", user_id))
            .await?
            .take(0)?;
        Ok(notifications)
    }
}

pub struct SmsLogQ;

impl SmsLogQ {
    /// Log an SMS
    pub async fn log(&self, sdb: &Surreal<Client>, data: SmsLog) -> DbResult<Option<SmsLog>> {
        let log: Option<SmsLog> = sdb.create(SMS_LOG_TABLE).content(data).await?;
        Ok(log)
    }

    /// Get total SMS cost for a school in the current month
    pub async fn get_monthly_cost(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Option<f64>> {
        let mut response = sdb
            .query(
                r#"
                SELECT math::sum(cost) AS total_cost
                FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND sent_at >= time::floor(time::now(), 1M)
                AND sent_at < time::now()
                "#,
            )
            .bind(("table", SMS_LOG_TABLE))
            .bind(("school_id", school_id))
            .await?;

        let result: Option<MonthlyCost> = response.take(0)?;
        Ok(result.and_then(|r| r.total_cost))
    }

    /// Get SMS logs for a school
    pub async fn get_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        limit: Option<u32>,
    ) -> DbResult<Vec<SmsLog>> {
        let limit = limit.unwrap_or(50);
        let logs: Vec<SmsLog> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                ORDER BY created_at DESC
                LIMIT $limit
                "#,
            )
            .bind(("table", SMS_LOG_TABLE))
            .bind(("school_id", school_id))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(logs)
    }
}
