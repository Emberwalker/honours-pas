use chrono::naive::NaiveDateTime;
use bigdecimal::BigDecimal;
use schema::*;

// Models for returned table rows and updates.
#[derive(Serialize, Identifiable, Queryable, AsChangeset, Clone, PartialEq, Debug)]
#[table_name = "staff"]
pub struct Staff {
    pub id: i32,
    pub email: String,
    pub full_name: String,
    pub is_admin: bool,
}

#[derive(Serialize, Identifiable, Queryable, Associations, AsChangeset, Clone, PartialEq, Debug)]
#[belongs_to(Session, foreign_key = "last_session")]
#[table_name = "students"]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub full_name: String,
    pub last_session: Option<i32>,
}

#[derive(Serialize, Identifiable, Queryable, AsChangeset, Clone, PartialEq, Debug)]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub supervisor_name: String,
    pub supervisor_email: String,
    pub created: NaiveDateTime,
    pub force_archive: bool,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, AsChangeset, Clone,
         PartialEq, Debug)]
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

// This is ugly, but it's the only way to do this cleanly until/if Rust adds delegation properly.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectWithStaff {
    pub id: i32,
    pub session: i32,
    pub supervisor_name: String,
    pub supervisor_email: String,
    pub name: String,
    pub description_md: String,
    pub additional_staff: Vec<String>,
}

// This doesn't implement AsChangeset - Diesel requires it only be used on types with non-PK fields.
#[derive(Serialize, Identifiable, Queryable, Associations, Clone, PartialEq, Debug)]
#[belongs_to(Project, foreign_key = "project")]
#[table_name = "project_staff"]
#[primary_key(project, staff)]
pub struct ProjectStaff {
    pub project: i32,
    pub staff: String,
}

#[derive(Serialize, Identifiable, Queryable, Associations, AsChangeset, Clone, PartialEq, Debug)]
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
#[derive(Serialize, Identifiable, Queryable, Associations, Clone, PartialEq, Debug)]
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

    #[derive(Deserialize, Insertable, PartialEq, Debug)]
    #[table_name = "staff"]
    pub struct Staff {
        pub email: String,
        pub full_name: String,
        pub is_admin: bool,
    }

    #[derive(Deserialize, Insertable, PartialEq, Debug)]
    #[table_name = "students"]
    pub struct Student {
        pub email: String,
        pub full_name: String,
        pub last_session: i32,
    }

    #[derive(Deserialize, Insertable, PartialEq, Debug)]
    #[table_name = "sessions"]
    pub struct Session {
        pub name: String,
        pub supervisor_name: String,
        pub supervisor_email: String,
        pub created: Option<NaiveDateTime>,
        pub force_archive: Option<bool>,
    }

    #[derive(Deserialize, Insertable, PartialEq, Debug)]
    #[table_name = "projects"]
    pub struct Project {
        pub session: i32,
        pub supervisor_name: String,
        pub supervisor_email: String,
        pub name: String,
        pub description_md: String,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct ProjectWithStaff {
        pub supervisor_name: String,
        pub supervisor_email: String,
        pub name: String,
        pub description_md: String,
        pub additional_staff: Vec<String>,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "project_staff"]
    pub struct ProjectStaff {
        pub project: i32,
        pub staff: String,
    }

    #[derive(Insertable, PartialEq, Debug)]
    #[table_name = "student_comments"]
    pub struct StudentComment {
        pub student: i32,
        pub session: i32,
        pub comment: Option<String>,
    }

    #[derive(Deserialize, Insertable, PartialEq, Debug)]
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

impl ProjectWithStaff {
    pub fn from_project(p: Project, s: Vec<ProjectStaff>) -> ProjectWithStaff {
        ProjectWithStaff {
            id: p.id,
            session: p.session,
            supervisor_name: p.supervisor_name,
            supervisor_email: p.supervisor_email,
            name: p.name,
            description_md: p.description_md,
            additional_staff: s.into_iter().map(|it| it.staff).collect(),
        }
    }
}

impl new::Project {
    pub fn from_with_staff(p: new::ProjectWithStaff, session: i32) -> Self {
        new::Project {
            session: session,
            supervisor_name: p.supervisor_name,
            supervisor_email: p.supervisor_email,
            name: p.name,
            description_md: p.description_md,
        }
    }
}
