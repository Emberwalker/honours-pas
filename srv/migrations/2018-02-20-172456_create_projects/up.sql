CREATE TABLE public.projects (
    id SERIAL PRIMARY KEY NOT NULL,
    session INT NOT NULL,
    supervisor_name TEXT NOT NULL,
    supervisor_email TEXT NOT NULL,
    name TEXT NOT NULL,
    description_md TEXT NOT NULL,
    CONSTRAINT projects_sessions_id_fk FOREIGN KEY (session) REFERENCES sessions (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX projects_session_index ON public.projects (session);