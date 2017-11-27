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
}

export interface ISession {
  name: string;
  is_current: boolean;
  coordinator_name: string;
  coordinator_email: string;
  projects: IProject[];
}

export interface IUser {
  name: string;
  email: string;
  marked_projects: number[];
  selected_projects: IProjectSelection[];
  user_type: UserType;
}

export interface IProjectSelection {
  owner: IUser;
  project: number;
  weight: number;
}
