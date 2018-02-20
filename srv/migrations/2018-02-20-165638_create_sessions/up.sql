CREATE TABLE public.sessions (
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL,
    supervisor_name TEXT NOT NULL,
    supervisor_email TEXT NOT NULL,
    created TIMESTAMP DEFAULT NOW() NOT NULL,
    force_archive BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE INDEX sessions_created_index ON public.sessions (created DESC);