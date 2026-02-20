pub mod academic;
pub mod assessment;
pub mod attendance;
pub mod communication;
pub mod fee;
pub mod school;
pub mod student;
pub mod system;
pub mod user;

pub use academic::{AcademicSessionQ, ClassQ, ClassSubjectQ, SubjectQ, TermQ};
pub use assessment::{AssessmentQ, GradingSchemeQ, ReportCardQ, StudentScoreQ};
pub use attendance::{AttendanceQ, AttendanceSummaryQ};
pub use communication::{AnnouncementQ, EventQ, MessageQ, NotificationQ, SmsLogQ};
pub use fee::{FeeStructureQ, InvoiceQ, PaymentQ, PaymentReminderQ};
pub use school::SchoolQ;
pub use student::{ParentQ, StudentQ};
pub use system::{ActivityLogQ, AnalyticsQ, ReportCardTemplateQ, SchoolSettingQ};
pub use user::UserQ;
