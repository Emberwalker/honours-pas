ALTER TABLE public.students DROP CONSTRAINT students_sessions_id_fk;
ALTER TABLE public.students
ADD CONSTRAINT students_sessions_id_fk
FOREIGN KEY (last_session) REFERENCES sessions (id) ON DELETE SET NULL ON UPDATE CASCADE;