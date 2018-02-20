CREATE TABLE public.authn_credentials (
    email TEXT UNIQUE NOT NULL,
    login_email TEXT NOT NULL PRIMARY KEY,
    password TEXT
);

CREATE INDEX authn_credentials_login_email_index ON public.authn_credentials (login_email);