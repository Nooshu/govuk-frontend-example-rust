//! Fishing licence application domain: steps, validation, answers.

use chrono::{Datelike, NaiveDate, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const LICENCE_ONE_DAY: &str = "1-day";
pub const LICENCE_EIGHT_DAYS: &str = "8-days";
pub const LICENCE_TWELVE_MONTHS: &str = "12-months";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StepId {
    LicenceLength,
    Name,
    DateOfBirth,
    WhereYouWillFish,
    Email,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub id: StepId,
    pub path: &'static str,
    pub heading: &'static str,
}

const STEPS: &[Step] = &[
    Step {
        id: StepId::LicenceLength,
        path: "/licence-length",
        heading: "How long do you need the licence for?",
    },
    Step {
        id: StepId::Name,
        path: "/name",
        heading: "What is your full name?",
    },
    Step {
        id: StepId::DateOfBirth,
        path: "/date-of-birth",
        heading: "What is your date of birth?",
    },
    Step {
        id: StepId::WhereYouWillFish,
        path: "/where-you-will-fish",
        heading: "Where will you fish?",
    },
    Step {
        id: StepId::Email,
        path: "/email",
        heading: "What is your email address?",
    },
];

#[derive(Debug, Clone)]
pub struct OptionItem {
    pub value: &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone)]
pub struct FeeOption {
    pub text: &'static str,
    pub fee: &'static str,
}

pub const COUNTRIES: &[OptionItem] = &[
    OptionItem {
        value: "England",
        text: "England",
    },
    OptionItem {
        value: "Wales",
        text: "Wales",
    },
    OptionItem {
        value: "Scotland",
        text: "Scotland",
    },
];

pub const LICENCE_LENGTHS: &[OptionItem] = &[
    OptionItem {
        value: LICENCE_ONE_DAY,
        text: "1 day",
    },
    OptionItem {
        value: LICENCE_EIGHT_DAYS,
        text: "8 days",
    },
    OptionItem {
        value: LICENCE_TWELVE_MONTHS,
        text: "12 months",
    },
];

pub const LICENCE_FEES: &[FeeOption] = &[
    FeeOption {
        text: "1 day",
        fee: "£7.10",
    },
    FeeOption {
        text: "8 days",
        fee: "£14.20",
    },
    FeeOption {
        text: "12 months",
        fee: "£36.80",
    },
];

pub fn steps() -> &'static [Step] {
    STEPS
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
    STEPS.iter().find(|s| !app.is_completed(s.id))
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    pub licence_length: String,
    pub full_name: String,
    pub day: String,
    pub month: String,
    pub year: String,
    pub country: String,
    pub email: String,
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
        STEPS.iter().all(|s| self.is_completed(s.id))
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

pub fn validate_name(full_name: &str) -> Vec<FieldError> {
    let name = clean(full_name);
    if name.chars().count() < 2 {
        return vec![FieldError::new(
            "full-name",
            "#full-name",
            "Enter your full name",
        )];
    }
    if name.chars().count() > 100 {
        return vec![FieldError::new(
            "full-name",
            "#full-name",
            "Full name must be 100 characters or fewer",
        )];
    }
    vec![]
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
        return fail("Enter your date of birth");
    }
    static D1: OnceLock<Regex> = OnceLock::new();
    static D4: OnceLock<Regex> = OnceLock::new();
    let d1 = D1.get_or_init(|| Regex::new(r"^[0-9]{1,2}$").unwrap());
    let d4 = D4.get_or_init(|| Regex::new(r"^[0-9]{4}$").unwrap());
    if !d1.is_match(&day) || !d1.is_match(&month) || !d4.is_match(&year) {
        return fail("Enter a real date of birth");
    }
    let d: u32 = day.parse().unwrap_or(0);
    let m: u32 = month.parse().unwrap_or(0);
    let y: i32 = year.parse().unwrap_or(0);
    let Some(date) = NaiveDate::from_ymd_opt(y, m, d) else {
        return fail("Enter a real date of birth");
    };
    let today = Utc::now().date_naive();
    if date > today {
        return fail("Date of birth must be in the past");
    }
    let age = age_on(date, today);
    if age < 13 {
        return fail("You must be at least 13 to use this example");
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

pub fn validate_country(value: &str) -> Vec<FieldError> {
    if COUNTRIES.iter().any(|o| o.value == value) {
        vec![]
    } else {
        vec![FieldError::new(
            "country",
            "#country",
            "Select where you will fish",
        )]
    }
}

pub fn validate_licence_length(value: &str) -> Vec<FieldError> {
    if LICENCE_LENGTHS.iter().any(|o| o.value == value) {
        vec![]
    } else {
        vec![FieldError::new(
            "licence-length",
            "#licence-length",
            "Select how long you need the licence for",
        )]
    }
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

pub fn label_for(options: &[OptionItem], value: &str) -> String {
    options
        .iter()
        .find(|o| o.value == value)
        .map(|o| o.text.to_string())
        .unwrap_or_else(|| value.to_string())
}

pub fn format_dob(day: &str, month: &str, year: &str) -> String {
    let day = day.trim();
    let month = month.trim();
    let year = year.trim();
    if day.is_empty() || month.is_empty() || year.is_empty() {
        return String::new();
    }
    format!("{day} {month} {year}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_validation() {
        assert!(!validate_name("").is_empty());
        assert!(!validate_name("A").is_empty());
        assert!(validate_name("Sam Taylor").is_empty());
        assert!(!validate_name(&"a".repeat(101)).is_empty());
    }

    #[test]
    fn email_validation() {
        assert!(!validate_email("bad").is_empty());
        assert!(validate_email("a@b.co").is_empty());
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
    fn country_and_licence_validation() {
        assert!(validate_country("England").is_empty());
        assert!(!validate_country("France").is_empty());
        assert!(validate_licence_length(LICENCE_TWELVE_MONTHS).is_empty());
        assert!(!validate_licence_length("12-month").is_empty());
        assert!(validate_licence_length(LICENCE_EIGHT_DAYS).is_empty());
        assert!(validate_cookie_choice("yes").is_empty());
        assert!(!validate_cookie_choice("maybe").is_empty());
    }

    #[test]
    fn application_completion() {
        let mut app = Application::default();
        assert!(!app.required_complete());
        assert_eq!(
            first_incomplete(&app).map(|s| s.id),
            Some(StepId::LicenceLength)
        );
        for step in steps() {
            app.mark_completed(step.id);
        }
        assert!(app.required_complete());
        assert_eq!(
            next_step(StepId::LicenceLength).map(|s| s.path),
            Some("/name")
        );
        assert_eq!(
            previous_step(StepId::Name).map(|s| s.path),
            Some("/licence-length")
        );
        assert!(step_by_path("/licence-length").is_some());
        assert_eq!(label_for(LICENCE_LENGTHS, "8-days"), "8 days");
        assert_eq!(format_dob("31", "3", "1980"), "31 3 1980");
    }
}
