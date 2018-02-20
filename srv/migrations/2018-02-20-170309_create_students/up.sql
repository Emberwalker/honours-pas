CREATE TABLE public.students (
    id SERIAL PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL,
    full_name TEXT NOT NULL,
    last_session INT,
    CONSTRAINT students_sessions_id_fk FOREIGN KEY (last_session) REFERENCES sessions (id) ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE INDEX students_email_index ON public.students (email);