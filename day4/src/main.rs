#[derive(Debug, Clone, Eq, PartialEq)]
struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct PassportBuilder {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl PassportBuilder {
    pub fn with_birth_year(mut self, birth_year: u32) -> Self {
        self.birth_year = Some(birth_year);
        self
    }

    pub fn with_issue_year(mut self, issue_year: u32) -> Self {
        self.issue_year = Some(issue_year);
        self
    }

    pub fn with_expiration_year(mut self, expiration_year: u32) -> Self {
        self.expiration_year = Some(expiration_year);
        self
    }

    pub fn with_height(mut self, height: String) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_hair_color(mut self, hair_color: String) -> Self {
        self.hair_color = Some(hair_color);
        self
    }

    pub fn with_eye_color(mut self, eye_color: String) -> Self {
        self.eye_color = Some(eye_color);
        self
    }

    pub fn with_passport_id(mut self, passport_id: String) -> Self {
        self.passport_id = Some(passport_id);
        self
    }

    pub fn with_country_id(mut self, country_id: String) -> Self {
        self.country_id = Some(country_id);
        self
    }

    pub fn build(self) -> Option<Passport> {
        Some(Passport {
            birth_year: self.birth_year?,
            issue_year: self.issue_year?,
            expiration_year: self.expiration_year?,
            height: self.height?,
            hair_color: self.hair_color?,
            eye_color: self.eye_color?,
            passport_id: self.passport_id?,
            country_id: self.country_id,
        })
    }
}

fn main() {
    
}
