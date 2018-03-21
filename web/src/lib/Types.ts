export enum UserType {
  Administrator = "admin",
  Staff = "staff",
  Student = "student",
}

export interface IProject {
  name: string;
  supervisor_name: string;
  supervisor_email: string;
  additional_staff: string[];
  description_md: string;
  id?: number;
  session?: number;
}

export interface ISession {
  id: number;
  name: string;
  is_current: boolean;
  supervisor_name: string;
  supervisor_email: string;
  projects: IProject[];
}

export interface INewSession {
  name: string;
  supervisor_name: string;
  supervisor_email: string;
}

export interface IUser {
  name: string;
  email: string;
  marked_projects: number[];
  selected_projects: IProjectSelection[];
  selection_comment: string;
  user_type: UserType;
}

export interface IProjectSelection {
  project: number;
  weight: number;
}

export interface ISupervisorCounter {
  name: string;
  email: string;
  count: number;
}
