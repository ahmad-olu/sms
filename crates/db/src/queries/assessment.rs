use crate::error::DbResult;
use shared::{
    helpers::create_update_dtos::EnterScoreDto,
    models::assessments_nd_grading::{
        Assessment, GradingScheme, ReportCard, ReportCardScore, StudentScore,
    },
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::{SurrealValue, Value};

const ASSESSMENT_TABLE: &str = "assessments";
const STUDENT_SCORE_TABLE: &str = "student_scores";
const GRADING_SCHEME_TABLE: &str = "grading_schemes";
const REPORT_CARD_TABLE: &str = "report_cards";
const REPORT_CARD_SCORE_TABLE: &str = "report_card_scores";

#[derive(Debug, SurrealValue)]
pub struct AvgResult {
    class_average: Option<f64>,
}

pub struct AssessmentQ;

impl AssessmentQ {
    /// Create an assessment
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: Assessment,
    ) -> DbResult<Option<Assessment>> {
        let assessment: Option<Assessment> = sdb.create(ASSESSMENT_TABLE).content(data).await?;
        Ok(assessment)
    }

    /// Get all assessments for a class + subject in a term
    pub async fn get_by_class_subject_term(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
        subject_id: String,
        term_id: String,
    ) -> DbResult<Vec<Assessment>> {
        let assessments: Vec<Assessment> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE class_id = type::thing('classes', $class_id)
                AND subject_id = type::thing('subjects', $subject_id)
                AND term_id = type::thing('terms', $term_id)
                ORDER BY assessment_type
                "#,
            )
            .bind(("table", ASSESSMENT_TABLE))
            .bind(("class_id", class_id))
            .bind(("subject_id", subject_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(assessments)
    }

    /// Get all assessments for a class in a term
    pub async fn get_by_class_term(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
        term_id: String,
    ) -> DbResult<Vec<Assessment>> {
        let assessments: Vec<Assessment> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE class_id = type::thing('classes', $class_id)
                AND term_id = type::thing('terms', $term_id)
                "#,
            )
            .bind(("table", ASSESSMENT_TABLE))
            .bind(("class_id", class_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(assessments)
    }
}

pub struct StudentScoreQ;

impl StudentScoreQ {
    /// Enter a score for a student
    pub async fn enter(
        &self,
        sdb: &Surreal<Client>,
        data: EnterScoreDto,
    ) -> DbResult<Option<StudentScore>> {
        let score: Option<StudentScore> = sdb.create(STUDENT_SCORE_TABLE).content(data).await?;
        Ok(score)
    }

    /// Update a student's score
    pub async fn update(
        &self,
        sdb: &Surreal<Client>,
        assessment_id: String,
        student_id: String,
        new_score: f64,
    ) -> DbResult<Option<StudentScore>> {
        let score: Option<StudentScore> = sdb
            .query(
                r#"
                UPDATE type::table($table) SET
                    score_obtained = $new_score,
                    updated_at = time::now()
                WHERE assessment_id = type::thing('assessments', $assessment_id)
                AND student_id = type::thing('students', $student_id)
                "#,
            )
            .bind(("table", STUDENT_SCORE_TABLE))
            .bind(("assessment_id", assessment_id))
            .bind(("student_id", student_id))
            .bind(("new_score", new_score))
            .await?
            .take(0)?;
        Ok(score)
    }

    /// Verify all scores for an assessment
    pub async fn verify_assessment_scores(
        &self,
        sdb: &Surreal<Client>,
        assessment_id: String,
        admin_id: String,
    ) -> DbResult<Vec<StudentScore>> {
        let scores: Vec<StudentScore> = sdb
            .query(
                r#"
                UPDATE type::table($table) SET
                    verified = true,
                    verified_by = type::thing('users', $admin_id),
                    verified_at = time::now()
                WHERE assessment_id = type::thing('assessments', $assessment_id)
                "#,
            )
            .bind(("table", STUDENT_SCORE_TABLE))
            .bind(("assessment_id", assessment_id))
            .bind(("admin_id", admin_id))
            .await?
            .take(0)?;
        Ok(scores)
    }

    /// Get all scores for a student in a subject (with assessment details)
    pub async fn get_by_student_subject(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        subject_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT *, assessment_id.* AS assessment
                FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                AND assessment_id.subject_id = type::thing('subjects', $subject_id)
                ORDER BY assessment_id.assessment_date
                "#,
            )
            .bind(("table", STUDENT_SCORE_TABLE))
            .bind(("student_id", student_id))
            .bind(("subject_id", subject_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get class average for an assessment
    pub async fn get_class_average(
        &self,
        sdb: &Surreal<Client>,
        assessment_id: String,
    ) -> DbResult<Option<f64>> {
        let mut response = sdb
            .query(
                r#"
                SELECT math::mean(score_obtained) AS class_average
                FROM type::table($table)
                WHERE assessment_id = type::thing('assessments', $assessment_id)
                "#,
            )
            .bind(("table", STUDENT_SCORE_TABLE))
            .bind(("assessment_id", assessment_id))
            .await?;

        let result: Option<AvgResult> = response.take(0)?;
        Ok(result.and_then(|r| r.class_average))
    }

    /// Get student total scores grouped by subject in a term
    pub async fn get_total_by_subject(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        term_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT
                    subject_id,
                    math::sum(score_obtained) AS total_score
                FROM (
                    SELECT *, assessment_id.subject_id AS subject_id
                    FROM type::table($table)
                    WHERE student_id = type::thing('students', $student_id)
                    AND assessment_id.term_id = type::thing('terms', $term_id)
                )
                GROUP BY subject_id
                "#,
            )
            .bind(("table", STUDENT_SCORE_TABLE))
            .bind(("student_id", student_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(result)
    }
}

pub struct GradingSchemeQ;

impl GradingSchemeQ {
    /// Create a grading scheme entry
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: GradingScheme,
    ) -> DbResult<Option<GradingScheme>> {
        let scheme: Option<GradingScheme> = sdb.create(GRADING_SCHEME_TABLE).content(data).await?;
        Ok(scheme)
    }

    /// Get the grade for a given score in a school
    pub async fn get_grade_for_score(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        score: f64,
    ) -> DbResult<Option<GradingScheme>> {
        let result: Option<GradingScheme> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND $score >= min_score
                AND $score <= max_score
                LIMIT 1
                "#,
            )
            .bind(("table", GRADING_SCHEME_TABLE))
            .bind(("school_id", school_id))
            .bind(("score", score))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get full grading scheme for a school
    pub async fn get_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<GradingScheme>> {
        let result: Vec<GradingScheme> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                ORDER BY min_score DESC
                "#,
            )
            .bind(("table", GRADING_SCHEME_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(result)
    }
}

pub struct ReportCardQ;

impl ReportCardQ {
    /// Create a report card
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: ReportCard,
    ) -> DbResult<Option<ReportCard>> {
        let card: Option<ReportCard> = sdb.create(REPORT_CARD_TABLE).content(data).await?;
        Ok(card)
    }

    /// Add subject scores to a report card
    pub async fn add_score(
        &self,
        sdb: &Surreal<Client>,
        data: ReportCardScore,
    ) -> DbResult<Option<ReportCardScore>> {
        let score: Option<ReportCardScore> =
            sdb.create(REPORT_CARD_SCORE_TABLE).content(data).await?;
        Ok(score)
    }

    /// Publish a report card
    pub async fn publish(
        &self,
        sdb: &Surreal<Client>,
        report_card_id: String,
    ) -> DbResult<Option<ReportCard>> {
        let card: Option<ReportCard> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    published = true,
                    published_at = time::now()
                "#,
            )
            .bind(("table", REPORT_CARD_TABLE))
            .bind(("id", report_card_id))
            .await?
            .take(0)?;
        Ok(card)
    }

    /// Get report card for a student in a term
    pub async fn get_by_student_term(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        term_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT *,
                    student_id.*,
                    class_id.*,
                    term_id.*
                FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                AND term_id = type::thing('terms', $term_id)
                LIMIT 1
                "#,
            )
            .bind(("table", REPORT_CARD_TABLE))
            .bind(("student_id", student_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get report card with all subject scores
    pub async fn get_with_scores(
        &self,
        sdb: &Surreal<Client>,
        report_card_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT *,
                    (SELECT * FROM type::table($score_table)
                     WHERE report_card_id = $parent.id) AS scores
                FROM type::thing($table, $id)
                "#,
            )
            .bind(("table", REPORT_CARD_TABLE))
            .bind(("score_table", REPORT_CARD_SCORE_TABLE))
            .bind(("id", report_card_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get all published report cards for a term
    pub async fn get_published_by_term(
        &self,
        sdb: &Surreal<Client>,
        term_id: String,
    ) -> DbResult<Vec<ReportCard>> {
        let cards: Vec<ReportCard> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE term_id = type::thing('terms', $term_id)
                AND published = true
                "#,
            )
            .bind(("table", REPORT_CARD_TABLE))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(cards)
    }

    /// Get top performing students in a term
    pub async fn get_top_performers(
        &self,
        sdb: &Surreal<Client>,
        term_id: String,
        limit: Option<u32>,
    ) -> DbResult<Vec<Value>> {
        let limit = limit.unwrap_or(10);
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT
                    student_id.*,
                    average_percentage
                FROM type::table($table)
                WHERE term_id = type::thing('terms', $term_id)
                AND published = true
                ORDER BY average_percentage DESC
                LIMIT $limit
                "#,
            )
            .bind(("table", REPORT_CARD_TABLE))
            .bind(("term_id", term_id))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Set PDF URL after generation
    pub async fn set_pdf_url(
        &self,
        sdb: &Surreal<Client>,
        report_card_id: String,
        pdf_url: String,
    ) -> DbResult<Option<ReportCard>> {
        let card: Option<ReportCard> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    pdf_url = $pdf_url
                "#,
            )
            .bind(("table", REPORT_CARD_TABLE))
            .bind(("id", report_card_id))
            .bind(("pdf_url", pdf_url))
            .await?
            .take(0)?;
        Ok(card)
    }
}
