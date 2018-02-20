CREATE TABLE public.student_comments (
    student INT NOT NULL,
    session INT NOT NULL,
    comment TEXT,
    PRIMARY KEY (student, session),
    CONSTRAINT student_comments_students_id_fk FOREIGN KEY (student) REFERENCES students (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT student_comments_sessions_id_fk FOREIGN KEY (session) REFERENCES sessions (id) ON DELETE CASCADE ON UPDATE CASCADE
);