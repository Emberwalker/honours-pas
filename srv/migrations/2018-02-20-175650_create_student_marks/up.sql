CREATE TABLE public.student_marks (
    student INT NOT NULL,
    project INT NOT NULL,
    PRIMARY KEY (student, project),
    CONSTRAINT student_marks_students_id_fk FOREIGN KEY (student) REFERENCES students (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT student_marks_projects_id_fk FOREIGN KEY (project) REFERENCES projects (id) ON DELETE CASCADE ON UPDATE CASCADE
);