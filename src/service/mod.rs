//! Fishing licence application domain: steps, validation, answers.

use chrono::{Datelike, NaiveDate, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const CONTACT_EMAIL: &str = "email";
pub const CONTACT_TELEPHONE: &str = "telephone";
pub const LICENCE_ONE_DAY: &str = "1-day";
pub const LICENCE_EIGHT_DAY: &str = "8-day";
pub const LICENCE_TWELVE_MTH: &str = "12-month";
pub const NOT_SURE: &str = "not-sure";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StepId {
    Name,
    DateOfBirth,
    Email,
    ContactPreference,
    WhereYouWillFish,
    LicenceLength,
    StartMonth,
    Address,
    Evidence,
    AdditionalDetails,
    CreateAPassword,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub id: StepId,
    pub path: &'static str,
    pub heading: &'static str,
}

const STEPS: &[Step] = &[
    Step {
        id: StepId::Name,
        path: "/name",
        heading: "What is your name?",
    },
    Step {
        id: StepId::DateOfBirth,
        path: "/date-of-birth",
        heading: "What is your date of birth?",
    },
    Step {
        id: StepId::Email,
        path: "/email",
        heading: "What is your email address?",
    },
    Step {
        id: StepId::ContactPreference,
        path: "/contact-preference",
        heading: "How should we contact you?",
    },
    Step {
        id: StepId::WhereYouWillFish,
        path: "/where-you-will-fish",
        heading: "Where will you fish?",
    },
    Step {
        id: StepId::LicenceLength,
        path: "/licence-length",
        heading: "How long do you need a licence for?",
    },
    Step {
        id: StepId::StartMonth,
        path: "/start-month",
        heading: "When should the licence start?",
    },
    Step {
        id: StepId::Address,
        path: "/address",
        heading: "What is your address?",
    },
    Step {
        id: StepId::Evidence,
        path: "/evidence",
        heading: "Upload evidence of a concession",
    },
    Step {
        id: StepId::AdditionalDetails,
        path: "/additional-details",
        heading: "Is there anything else we should know?",
    },
    Step {
        id: StepId::CreateAPassword,
        path: "/create-a-password",
        heading: "Create a password",
    },
];

#[derive(Debug, Clone)]
pub struct OptionItem {
    pub value: &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone)]
pub struct LicenceOption {
    pub value: &'static str,
    pub text: &'static str,
    pub fee: &'static str,
}

pub const REGIONS: &[OptionItem] = &[
    OptionItem {
        value: "north-west",
        text: "North West",
    },
    OptionItem {
        value: "north-east",
        text: "North East",
    },
    OptionItem {
        value: "midlands",
        text: "Midlands",
    },
    OptionItem {
        value: "south-west",
        text: "South West",
    },
    OptionItem {
        value: "south-east",
        text: "South East",
    },
    OptionItem {
        value: "wales",
        text: "Wales",
    },
];

pub const LICENCE_LENGTHS: &[LicenceOption] = &[
    LicenceOption {
        value: LICENCE_ONE_DAY,
        text: "1 day",
        fee: "£7.10",
    },
    LicenceOption {
        value: LICENCE_EIGHT_DAY,
        text: "8 days",
        fee: "£14.20",
    },
    LicenceOption {
        value: LICENCE_TWELVE_MTH,
        text: "12 months",
        fee: "£36.80",
    },
];

pub fn steps() -> &'static [Step] {
    STEPS
}

pub fn optional(id: StepId) -> bool {
    matches!(id, StepId::Evidence | StepId::AdditionalDetails)
}

pub fn step_by_path(path: &str) -> Option<&'static Step> {
    STEPS.iter().find(|s| s.path == path)
}

pub fn step_by_id(id: StepId) -> Option<&'static Step> {
    STEPS.iter().find(|s| s.id == id)
}

pub fn next_step(id: StepId) -> Option<&'static Step> {
    let i = STEPS.iter().position(|s| s.id == id)?;
    STEPS.get(i + 1)
}

pub fn previous_step(id: StepId) -> Option<&'static Step> {
    let i = STEPS.iter().position(|s| s.id == id)?;
    if i == 0 {
        None
    } else {
        STEPS.get(i - 1)
    }
}

pub fn first_incomplete(app: &Application) -> Option<&'static Step> {
    STEPS
        .iter()
        .find(|s| !optional(s.id) && !app.is_completed(s.id))
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    pub first_name: String,
    pub last_name: String,
    pub day: String,
    pub month: String,
    pub year: String,
    pub email: String,
    pub contact_by: String,
    pub telephone: String,
    pub regions: Vec<String>,
    pub licence_length: String,
    pub start_month: String,
    pub address_line1: String,
    pub address_line2: String,
    pub town: String,
    pub postcode: String,
    pub evidence_filename: String,
    pub additional_details: String,
    pub password_created: bool,
    pub submitted: bool,
    pub reference: String,
    pub completed: Vec<StepId>,
}

impl Application {
    pub fn is_completed(&self, id: StepId) -> bool {
        self.completed.contains(&id)
    }

    pub fn mark_completed(&mut self, id: StepId) {
        if !self.is_completed(id) {
            self.completed.push(id);
        }
    }

    pub fn unmark_completed(&mut self, id: StepId) {
        self.completed.retain(|x| *x != id);
    }

    pub fn required_complete(&self) -> bool {
        STEPS
            .iter()
            .filter(|s| !optional(s.id))
            .all(|s| self.is_completed(s.id))
    }
}

#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub href: String,
    pub text: String,
}

impl FieldError {
    pub fn new(field: &str, href: &str, text: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            href: href.into(),
            text: text.into(),
        }
    }
}

pub fn clean(value: &str) -> String {
    value.trim().to_string()
}

pub fn validate_name(first: &str, last: &str) -> Vec<FieldError> {
    let mut errors = Vec::new();
    let first = clean(first);
    let last = clean(last);
    if first.is_empty() {
        errors.push(FieldError::new(
            "first-name",
            "#first-name",
            "Enter your first name",
        ));
    } else if first.chars().count() > 100 {
        errors.push(FieldError::new(
            "first-name",
            "#first-name",
            "First name must be 100 characters or fewer",
        ));
    }
    if last.is_empty() {
        errors.push(FieldError::new(
            "last-name",
            "#last-name",
            "Enter your last name",
        ));
    } else if last.chars().count() > 100 {
        errors.push(FieldError::new(
            "last-name",
            "#last-name",
            "Last name must be 100 characters or fewer",
        ));
    }
    errors
}

pub fn validate_email(email: &str) -> Vec<FieldError> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());
    if !re.is_match(&clean(email)) {
        vec![FieldError::new(
            "email",
            "#email",
            "Enter an email address in the correct format, like name@example.com",
        )]
    } else {
        vec![]
    }
}

pub fn validate_date_of_birth(day: &str, month: &str, year: &str) -> Vec<FieldError> {
    let fail = |text: &str| vec![FieldError::new("date-of-birth", "#date-of-birth-day", text)];
    let day = clean(day);
    let month = clean(month);
    let year = clean(year);
    if day.is_empty() || month.is_empty() || year.is_empty() {
        return fail("Date of birth must include a day, month and year");
    }
    static D1: OnceLock<Regex> = OnceLock::new();
    static D4: OnceLock<Regex> = OnceLock::new();
    let d1 = D1.get_or_init(|| Regex::new(r"^[0-9]{1,2}$").unwrap());
    let d4 = D4.get_or_init(|| Regex::new(r"^[0-9]{4}$").unwrap());
    if !d1.is_match(&day) || !d1.is_match(&month) || !d4.is_match(&year) {
        return fail("Date of birth must be a real date");
    }
    let d: u32 = day.parse().unwrap_or(0);
    let m: u32 = month.parse().unwrap_or(0);
    let y: i32 = year.parse().unwrap_or(0);
    let Some(date) = NaiveDate::from_ymd_opt(y, m, d) else {
        return fail("Date of birth must be a real date");
    };
    let today = Utc::now().date_naive();
    if date > today {
        return fail("Date of birth must be in the past");
    }
    let age = age_on(date, today);
    if age < 13 {
        return fail("You must be 13 or over to apply for a rod licence");
    }
    vec![]
}

fn age_on(dob: NaiveDate, today: NaiveDate) -> i32 {
    let mut age = today.year() - dob.year();
    if today.month() < dob.month() || (today.month() == dob.month() && today.day() < dob.day()) {
        age -= 1;
    }
    age
}

pub fn validate_contact(contact_by: &str, telephone: &str) -> Vec<FieldError> {
    static PHONE: OnceLock<Regex> = OnceLock::new();
    let phone = PHONE.get_or_init(|| Regex::new(r"^[0-9+() -]{8,20}$").unwrap());
    let mut errors = Vec::new();
    if contact_by != CONTACT_EMAIL && contact_by != CONTACT_TELEPHONE {
        errors.push(FieldError::new(
            "contact-by",
            "#contact-by",
            "Select how we should contact you",
        ));
    }
    let tel = clean(telephone);
    if contact_by == CONTACT_TELEPHONE && tel.is_empty() {
        errors.push(FieldError::new(
            "telephone",
            "#telephone",
            "Enter a telephone number",
        ));
    } else if !tel.is_empty() && !phone.is_match(&tel) {
        errors.push(FieldError::new(
            "telephone",
            "#telephone",
            "Enter a telephone number, like 01632 960 001",
        ));
    }
    errors
}

pub fn validate_regions(selected: &[String]) -> Vec<FieldError> {
    let problem = |text: &str| vec![FieldError::new("regions", "#regions", text)];
    if selected.is_empty() {
        return problem("Select where you will fish");
    }
    let known: Vec<&str> = REGIONS.iter().map(|r| r.value).collect();
    let exclusive = selected.iter().any(|s| s == NOT_SURE);
    let chosen: Vec<_> = selected.iter().filter(|s| s.as_str() != NOT_SURE).collect();
    if exclusive && !chosen.is_empty() {
        return problem("Select where you will fish, or select that you have not decided yet");
    }
    for region in &chosen {
        if !known.contains(&region.as_str()) {
            return problem("Select where you will fish");
        }
    }
    vec![]
}

pub fn validate_licence_length(value: &str) -> Vec<FieldError> {
    if LICENCE_LENGTHS.iter().any(|o| o.value == value) {
        vec![]
    } else {
        vec![FieldError::new(
            "licence-length",
            "#licence-length",
            "Select how long you need a licence for",
        )]
    }
}

pub fn start_months() -> Vec<(String, String)> {
    let today = Utc::now().date_naive();
    let mut start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    let mut out = Vec::with_capacity(12);
    for _ in 0..12 {
        out.push((
            format!("{:04}-{:02}", start.year(), start.month()),
            start.format("%B %Y").to_string(),
        ));
        start = start
            .checked_add_months(chrono::Months::new(1))
            .unwrap_or(start);
    }
    out
}

pub fn validate_start_month(value: &str) -> Vec<FieldError> {
    if start_months().iter().any(|(v, _)| v == value) {
        vec![]
    } else {
        vec![FieldError::new(
            "start-month",
            "#start-month",
            "Select when the licence should start",
        )]
    }
}

pub fn normalise_postcode(value: &str) -> String {
    let compact: String = clean(value)
        .to_uppercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    if compact.len() < 5 {
        return String::new();
    }
    let split = compact.len() - 3;
    format!("{} {}", &compact[..split], &compact[split..])
}

pub fn validate_address(line1: &str, town: &str, postcode: &str) -> Vec<FieldError> {
    static PC: OnceLock<Regex> = OnceLock::new();
    let pc = PC.get_or_init(|| Regex::new(r"^[A-Z]{1,2}[0-9][A-Z0-9]? [0-9][A-Z]{2}$").unwrap());
    let mut errors = Vec::new();
    let line1 = clean(line1);
    if line1.is_empty() {
        errors.push(FieldError::new(
            "address-line-1",
            "#address-line-1",
            "Enter address line 1",
        ));
    } else if line1.chars().count() > 100 {
        errors.push(FieldError::new(
            "address-line-1",
            "#address-line-1",
            "Address line 1 must be 100 characters or fewer",
        ));
    }
    if clean(town).is_empty() {
        errors.push(FieldError::new("town", "#town", "Enter a town or city"));
    }
    let normalised = normalise_postcode(postcode);
    if normalised.is_empty() || !pc.is_match(&normalised) {
        errors.push(FieldError::new(
            "postcode",
            "#postcode",
            "Enter a full UK postcode",
        ));
    }
    errors
}

pub fn validate_evidence(filename: &str) -> Vec<FieldError> {
    if filename.is_empty() {
        return vec![];
    }
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"(?i)\.(pdf|png|jpe?g)$").unwrap());
    if !re.is_match(filename) {
        vec![FieldError::new(
            "evidence",
            "#evidence",
            "The selected file must be a PDF, PNG, or JPG",
        )]
    } else {
        vec![]
    }
}

pub fn validate_additional_details(value: &str) -> Vec<FieldError> {
    if value.chars().count() > 200 {
        vec![FieldError::new(
            "additional-details",
            "#additional-details",
            "Additional details must be 200 characters or fewer",
        )]
    } else {
        vec![]
    }
}

pub fn validate_password(password: &str, confirm: &str) -> Vec<FieldError> {
    if password.chars().count() < 8 {
        return vec![FieldError::new(
            "password",
            "#password",
            "Password must be at least 8 characters",
        )];
    }
    if password != confirm {
        return vec![FieldError::new(
            "password-confirm",
            "#password-confirm",
            "Enter the same password in both fields",
        )];
    }
    vec![]
}

pub fn validate_cookie_choice(value: &str) -> Vec<FieldError> {
    if value != "yes" && value != "no" {
        vec![FieldError::new(
            "analytics",
            "#analytics",
            "Select yes if you want to accept analytics cookies",
        )]
    } else {
        vec![]
    }
}

pub fn safe_filename(filename: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^[\w. -]+$").unwrap());
    let base = filename.rsplit(['/', '\\']).next().unwrap_or(filename);
    if base.is_empty() || base == "." || base == ".." {
        return None;
    }
    if base.chars().count() > 120 || !re.is_match(base) {
        return None;
    }
    Some(base.to_string())
}

pub fn label_for(options: &[OptionItem], value: &str) -> String {
    options
        .iter()
        .find(|o| o.value == value)
        .map(|o| o.text.to_string())
        .unwrap_or_else(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_validation() {
        assert!(!validate_name("", "x").is_empty());
        assert!(!validate_name("Sam", "").is_empty());
        assert!(validate_name("Sam", "Taylor").is_empty());
        assert!(!validate_name(&"a".repeat(101), "T").is_empty());
    }

    #[test]
    fn email_validation() {
        assert!(!validate_email("bad").is_empty());
        assert!(validate_email("a@b.co").is_empty());
    }

    #[test]
    fn postcode_normalises() {
        assert_eq!(normalise_postcode("sw1a1aa"), "SW1A 1AA");
    }

    #[test]
    fn date_of_birth_validation() {
        assert!(!validate_date_of_birth("", "1", "1990").is_empty());
        assert!(!validate_date_of_birth("32", "1", "1990").is_empty());
        assert!(validate_date_of_birth("1", "1", "1990").is_empty());
        let future = (Utc::now().year() + 1).to_string();
        assert!(!validate_date_of_birth("1", "1", &future).is_empty());
    }

    #[test]
    fn contact_and_licence_validation() {
        assert!(!validate_contact("", "").is_empty());
        assert!(validate_contact(CONTACT_EMAIL, "").is_empty());
        assert!(!validate_contact(CONTACT_TELEPHONE, "").is_empty());
        assert!(validate_contact(CONTACT_TELEPHONE, "01632 960 001").is_empty());
        assert!(validate_licence_length(LICENCE_TWELVE_MTH).is_empty());
        assert!(!validate_licence_length("nope").is_empty());
    }

    #[test]
    fn regions_and_address() {
        assert!(!validate_regions(&[]).is_empty());
        assert!(validate_regions(&["north-west".into()]).is_empty());
        assert!(!validate_regions(&["north-west".into(), NOT_SURE.into()]).is_empty());
        assert!(validate_address("1 High St", "London", "SW1A 1AA").is_empty());
        assert!(!validate_address("", "London", "SW1A 1AA").is_empty());
    }

    #[test]
    fn password_evidence_cookies() {
        assert!(!validate_password("short", "short").is_empty());
        assert!(!validate_password("longenough", "different").is_empty());
        assert!(validate_password("longenough", "longenough").is_empty());
        assert!(validate_evidence("").is_empty());
        assert!(validate_evidence("scan.pdf").is_empty());
        assert!(!validate_evidence("scan.exe").is_empty());
        assert!(validate_cookie_choice("yes").is_empty());
        assert!(!validate_cookie_choice("maybe").is_empty());
        assert_eq!(safe_filename("a/b/proof.PDF").as_deref(), Some("proof.PDF"));
        assert!(safe_filename("..").is_none());
        assert!(safe_filename("bad name!!.pdf").is_none());
    }

    #[test]
    fn application_completion() {
        let mut app = Application::default();
        assert!(!app.required_complete());
        for step in steps() {
            if !optional(step.id) {
                app.mark_completed(step.id);
            }
        }
        assert!(app.required_complete());
        assert_eq!(
            next_step(StepId::Name).map(|s| s.path),
            Some("/date-of-birth")
        );
        assert_eq!(
            previous_step(StepId::Email).map(|s| s.path),
            Some("/date-of-birth")
        );
        assert!(step_by_path("/name").is_some());
        assert!(!start_months().is_empty());
        assert_eq!(label_for(REGIONS, "wales"), "Wales");
    }
}
