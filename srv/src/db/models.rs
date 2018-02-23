use chrono::naive::NaiveDateTime;
use bigdecimal::BigDecimal;
use schema::*;

// Models for returned table rows and updates.
#[derive(Identifiable, Queryable, AsChangeset, Clone, PartialEq, Debug)]
#[table_name = "staff"]
pub struct Staff {
    pub id: i32,
    pub email: String,
    pub full_name: String,
    pub is_admin: bool,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset, Clone, PartialEq, Debug)]
#[belongs_to(Session, foreign_key = "last_session")]
#[table_name = "students"]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub full_name: String,
    pub last_session: Option<i32>,
}

#[derive(Identifiable, Queryable, AsChangeset, Clone, PartialEq, Debug)]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub supervisor_name: String,
    pub supervisor_email: String,
    pub created: NaiveDateTime,
    pub force_archive: bool,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset, Clone, PartialEq, Debug)]
#[belongs_to(Session, foreign_key = "session")]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub session: i32,
    pub supervisor_name: String,
    pub supervisor_email: String,
    pub name: String,
    pub description_md: String,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset,Clone,  PartialEq, Debug)]
#[belongs_to(Student, foreign_key = "student")]
#[belongs_to(Session, foreign_key = "session")]
#[table_name = "student_comments"]
#[primary_key(student, session)]
pub struct StudentComment {
    pub student: i32,
    pub session: i32,
    pub comment: Option<String>,
}

// This doesn't implement AsChangeset - Diesel requires it only be used on types with non-PK fields.
#[derive(Identifiable, Queryable, Associations, Clone, PartialEq, Debug)]
#[belongs_to(Student, foreign_key = "student")]
#[belongs_to(Project, foreign_key = "project")]
#[table_name = "student_marks"]
#[primary_key(student, project)]
pub struct StudentMark {
    pub student: i32,
    pub project: i32,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset, Clone, PartialEq, Debug)]
#[belongs_to(Student, foreign_key = "student")]
#[belongs_to(Project, foreign_key = "project")]
#[table_name = "student_selections"]
#[primary_key(student, project)]
pub struct StudentSelection {
    pub student: i32,
    pub project: i32,
    pub weight: BigDecimal,
}

// Models for insertions.
pub mod new {
    use chrono::naive::NaiveDateTime;
    use bigdecimal::BigDecimal;
    use schema::*;

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "staff"]
    pub struct Staff<'a> {
        pub email: &'a str,
        pub full_name: &'a str,
        pub is_admin: bool,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "students"]
    pub struct Student<'a> {
        pub email: &'a str,
        pub full_name: &'a str,
        pub last_session: i32,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "sessions"]
    pub struct Session<'a> {
        pub name: &'a str,
        pub supervisor_name: &'a str,
        pub supervisor_email: &'a str,
        pub created: NaiveDateTime,
        pub force_archive: bool,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "projects"]
    pub struct Project<'a> {
        pub session: i32,
        pub supervisor_name: &'a str,
        pub supervisor_email: &'a str,
        pub name: &'a str,
        pub description_md: &'a str,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "student_comments"]
    pub struct StudentComment<'a> {
        pub student: i32,
        pub session: i32,
        pub comment: Option<&'a str>,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "student_marks"]
    pub struct StudentMark {
        pub student: i32,
        pub project: i32,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "student_selections"]
    pub struct StudentSelection {
        pub student: i32,
        pub project: i32,
        pub weight: BigDecimal,
    }
}
