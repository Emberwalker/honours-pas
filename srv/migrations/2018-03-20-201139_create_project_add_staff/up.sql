CREATE TABLE public.project_staff (
    project INT NOT NULL,
    staff TEXT NOT NULL,
    PRIMARY KEY (project, staff),
    CONSTRAINT project_staff_projects_id_fk FOREIGN KEY (project) REFERENCES projects (id) ON DELETE CASCADE ON UPDATE CASCADE
);