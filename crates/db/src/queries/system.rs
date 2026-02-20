use crate::error::DbResult;

use shared::{
    helpers::api_responses::DashboardMetrics,
    models::{
        SettingType,
        settings_and_configuration::{ReportCardTemplate, SchoolSetting},
        system_and_audit::ActivityLog,
    },
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::{Decimal, SurrealValue, Value};

const ACTIVITY_LOG_TABLE: &str = "activity_logs";
const SCHOOL_SETTING_TABLE: &str = "school_settings";
const REPORT_CARD_TEMPLATE_TABLE: &str = "report_card_templates";

#[derive(Debug, SurrealValue)]
struct GetSettings {
    setting_value: Option<String>,
}

pub struct ActivityLogQ;

impl ActivityLogQ {
    /// Log an activity
    pub async fn log(
        &self,
        sdb: &Surreal<Client>,
        data: ActivityLog,
    ) -> DbResult<Option<ActivityLog>> {
        let log: Option<ActivityLog> = sdb.create(ACTIVITY_LOG_TABLE).content(data).await?;
        Ok(log)
    }

    /// Get recent activity for a school
    pub async fn get_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        limit: Option<u32>,
    ) -> DbResult<Vec<ActivityLog>> {
        let limit = limit.unwrap_or(50);
        let logs: Vec<ActivityLog> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                ORDER BY created_at DESC
                LIMIT $limit
                "#,
            )
            .bind(("table", ACTIVITY_LOG_TABLE))
            .bind(("school_id", school_id))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(logs)
    }

    /// Get activity for a specific user
    pub async fn get_by_user(
        &self,
        sdb: &Surreal<Client>,
        user_id: String,
        limit: Option<u32>,
    ) -> DbResult<Vec<ActivityLog>> {
        let limit = limit.unwrap_or(50);
        let logs: Vec<ActivityLog> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE user_id = type::thing('users', $user_id)
                ORDER BY created_at DESC
                LIMIT $limit
                "#,
            )
            .bind(("table", ACTIVITY_LOG_TABLE))
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(logs)
    }

    /// Get activity for a specific entity
    pub async fn get_by_entity(
        &self,
        sdb: &Surreal<Client>,
        entity_type: String,
        entity_id: String,
    ) -> DbResult<Vec<ActivityLog>> {
        let logs: Vec<ActivityLog> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE entity_type = $entity_type
                AND entity_id = $entity_id
                ORDER BY created_at DESC
                "#,
            )
            .bind(("table", ACTIVITY_LOG_TABLE))
            .bind(("entity_type", entity_type))
            .bind(("entity_id", entity_id))
            .await?
            .take(0)?;
        Ok(logs)
    }
}

pub struct SchoolSettingQ;

impl SchoolSettingQ {
    /// Set (create) a school setting
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: SchoolSetting,
    ) -> DbResult<Option<SchoolSetting>> {
        let setting: Option<SchoolSetting> = sdb.create(SCHOOL_SETTING_TABLE).content(data).await?;
        Ok(setting)
    }

    /// Get a specific setting by key
    pub async fn get(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        key: String,
    ) -> DbResult<Option<String>> {
        let mut response = sdb
            .query(
                r#"
                SELECT setting_value FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND setting_key = $key
                LIMIT 1
                "#,
            )
            .bind(("table", SCHOOL_SETTING_TABLE))
            .bind(("school_id", school_id))
            .bind(("key", key))
            .await?;

        let result: Option<GetSettings> = response.take(0)?;
        Ok(result.and_then(|r| r.setting_value))
    }

    /// Update a setting value
    pub async fn update(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        key: String,
        value: String,
    ) -> DbResult<Option<SchoolSetting>> {
        let setting: Option<SchoolSetting> = sdb
            .query(
                r#"
                UPDATE type::table($table) SET
                    setting_value = $value,
                    updated_at = time::now()
                WHERE school_id = type::thing('schools', $school_id)
                AND setting_key = $key
                "#,
            )
            .bind(("table", SCHOOL_SETTING_TABLE))
            .bind(("school_id", school_id))
            .bind(("key", key))
            .bind(("value", value))
            .await?
            .take(0)?;
        Ok(setting)
    }

    /// Upsert a setting (create or update)
    pub async fn upsert(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        key: String,
        value: String,
        setting_type: SettingType,
    ) -> DbResult<()> {
        sdb.query(
            r#"
            IF (SELECT id FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND setting_key = $key)[0] {
                UPDATE type::table($table) SET
                    setting_value = $value,
                    updated_at = time::now()
                WHERE school_id = type::thing('schools', $school_id)
                AND setting_key = $key;
            } ELSE {
                CREATE type::table($table) CONTENT {
                    school_id: type::thing('schools', $school_id),
                    setting_key: $key,
                    setting_value: $value,
                    setting_type: $setting_type,
                    updated_at: time::now()
                };
            };
            "#,
        )
        .bind(("table", SCHOOL_SETTING_TABLE))
        .bind(("school_id", school_id))
        .bind(("key", key))
        .bind(("value", value))
        .bind(("setting_type", setting_type))
        .await?;
        Ok(())
    }

    /// Get all settings for a school
    pub async fn get_all(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<SchoolSetting>> {
        let settings: Vec<SchoolSetting> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                "#,
            )
            .bind(("table", SCHOOL_SETTING_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(settings)
    }
}

pub struct ReportCardTemplateQ;

impl ReportCardTemplateQ {
    /// Create a template
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: ReportCardTemplate,
    ) -> DbResult<Option<ReportCardTemplate>> {
        let template: Option<ReportCardTemplate> =
            sdb.create(REPORT_CARD_TEMPLATE_TABLE).content(data).await?;
        Ok(template)
    }

    /// Get default template for a school
    pub async fn get_default(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Option<ReportCardTemplate>> {
        let template: Option<ReportCardTemplate> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND is_default = true
                LIMIT 1
                "#,
            )
            .bind(("table", REPORT_CARD_TEMPLATE_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(template)
    }

    /// Set a template as default (unsets others)
    pub async fn set_default(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        template_id: String,
    ) -> DbResult<()> {
        sdb.query(
            r#"
            BEGIN TRANSACTION;
            UPDATE type::table($table) SET is_default = false
                WHERE school_id = type::thing('schools', $school_id);
            UPDATE type::thing($table, $template_id) SET
                is_default = true,
                updated_at = time::now();
            COMMIT TRANSACTION;
            "#,
        )
        .bind(("table", REPORT_CARD_TEMPLATE_TABLE))
        .bind(("school_id", school_id))
        .bind(("template_id", template_id))
        .await?;
        Ok(())
    }
}

pub struct AnalyticsQ;

impl AnalyticsQ {
    /// Get dashboard metrics for a school in a term
    pub async fn get_dashboard_metrics(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        term_id: String,
    ) -> DbResult<DashboardMetrics> {
        // SurrealDB doesn't support cross-table aggregation in a single SELECT easily,
        // so we run separate queries and combine.
        let mut response = sdb
            .query(
                r#"
                LET $sid = type::thing('schools', $school_id);
                LET $tid = type::thing('terms', $term_id);

                SELECT
                    (SELECT count() FROM students WHERE school_id = $sid AND status = 'active')[0].count AS total_students,
                    (SELECT count() FROM users WHERE school_id = $sid AND user_type = 'teacher' AND status = 'active')[0].count AS total_teachers,
                    (SELECT count() FROM classes WHERE school_id = $sid)[0].count AS total_classes,
                    (SELECT math::sum(amount_paid) FROM invoices WHERE school_id = $sid AND term_id = $tid)[0]['math::sum'] AS total_collected,
                    (SELECT math::sum(total_amount) FROM invoices WHERE school_id = $sid AND term_id = $tid)[0]['math::sum'] AS total_expected
                FROM ONLY $sid;
                "#,
            )
            .bind(("school_id", school_id))
            .bind(("term_id", term_id))
            .await?;

        let raw: Option<Value> = response.take(2)?;

        // Parse and map into DashboardMetrics
        let metrics = if let Some(v) = raw {
            if let Some(obj) = v.as_object() {
                DashboardMetrics {
                    total_students: obj
                        .get("total_students")
                        .and_then(|x| x.as_i64().copied())
                        .unwrap_or(0),
                    total_teachers: obj
                        .get("total_teachers")
                        .and_then(|x| x.as_i64().copied())
                        .unwrap_or(0),
                    total_classes: obj
                        .get("total_classes")
                        .and_then(|x| x.as_i64().copied())
                        .unwrap_or(0),
                    total_collected: obj
                        .get("total_collected")
                        .and_then(|x| x.as_decimal().copied())
                        .unwrap_or_default(),
                    total_expected: obj
                        .get("total_expected")
                        .and_then(|x| x.as_decimal().copied())
                        .unwrap_or_default(),
                    collection_rate: {
                        let collected = obj
                            .get("total_collected")
                            .and_then(|x| x.as_decimal().copied())
                            .unwrap_or_default();
                        let expected = obj
                            .get("total_expected")
                            .and_then(|x| x.as_decimal().copied())
                            .unwrap_or_default();
                        if expected.is_zero() {
                            Decimal::ZERO
                        } else {
                            (collected / expected) * Decimal::from(100)
                        }
                    },
                }
            } else {
                DashboardMetrics {
                    total_students: 0,
                    total_teachers: 0,
                    total_classes: 0,
                    total_collected: Decimal::ZERO,
                    total_expected: Decimal::ZERO,
                    collection_rate: Decimal::ZERO,
                }
            }
        } else {
            DashboardMetrics {
                total_students: 0,
                total_teachers: 0,
                total_classes: 0,
                total_collected: Decimal::ZERO,
                total_expected: Decimal::ZERO,
                collection_rate: Decimal::ZERO,
            }
        };

        Ok(metrics)
    }

    /// Fee collection by class for a term
    pub async fn get_fee_collection_by_class(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        term_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT
                    student_id.current_class_id.class_name AS class_name,
                    count() AS total_invoices,
                    math::sum(amount_paid) AS total_collected,
                    math::sum(balance) AS total_outstanding
                FROM invoices
                WHERE school_id = type::thing('schools', $school_id)
                AND term_id = type::thing('terms', $term_id)
                GROUP BY student_id.current_class_id
                "#,
            )
            .bind(("school_id", school_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(result)
    }
}
