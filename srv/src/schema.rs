table! {
    authn_credentials (login_email) {
        email -> Text,
        login_email -> Text,
        password -> Nullable<Text>,
    }
}

table! {
    projects (id) {
        id -> Int4,
        session -> Int4,
        supervisor_name -> Text,
        supervisor_email -> Text,
        name -> Text,
        description_md -> Text,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        name -> Text,
        supervisor_name -> Text,
        supervisor_email -> Text,
        created -> Timestamp,
        force_archive -> Bool,
    }
}

table! {
    staff (id) {
        id -> Int4,
        email -> Text,
        full_name -> Text,
        is_admin -> Bool,
    }
}

table! {
    student_comments (student, session) {
        student -> Int4,
        session -> Int4,
        comment -> Nullable<Text>,
    }
}

table! {
    student_marks (student, project) {
        student -> Int4,
        project -> Int4,
    }
}

table! {
    students (id) {
        id -> Int4,
        email -> Text,
        full_name -> Text,
        last_session -> Nullable<Int4>,
    }
}

table! {
    student_selections (student, project) {
        student -> Int4,
        project -> Int4,
        weight -> Numeric,
    }
}

joinable!(projects -> sessions (session));
joinable!(student_comments -> sessions (session));
joinable!(student_comments -> students (student));
joinable!(student_marks -> projects (project));
joinable!(student_marks -> students (student));
joinable!(student_selections -> projects (project));
joinable!(student_selections -> students (student));
joinable!(students -> sessions (last_session));

allow_tables_to_appear_in_same_query!(
    authn_credentials,
    projects,
    sessions,
    staff,
    student_comments,
    student_marks,
    students,
    student_selections,
);
