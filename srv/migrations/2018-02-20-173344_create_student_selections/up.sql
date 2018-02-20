CREATE TABLE public.student_selections (
    student INT NOT NULL,
    project INT NOT NULL,
    weight NUMERIC(4,2) NOT NULL,
    PRIMARY KEY (student, project),
    CONSTRAINT student_selections_students_id_fk FOREIGN KEY (student) REFERENCES students (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT student_selections_projects_id_fk FOREIGN KEY (project) REFERENCES projects (id) ON DELETE CASCADE ON UPDATE CASCADE
);