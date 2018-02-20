CREATE TABLE public.staff (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    full_name TEXT NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE INDEX staff_email_index ON public.staff (email);