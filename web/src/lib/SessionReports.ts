import _ from "lodash";

export interface ISessionReportRaw {
    session: ISession;
    by_student: IRawByStudent[];
    by_project: IRawByProject[];
    students: IStudent[];
    projects: IProject[];
    comments: string[number];
}

export interface ISession {
    name: string;
    supervisor_name: string;
    supervisor_email: string;
    id: number;
}

export interface IRawByStudent {
    student: number;
    choices: number[];
    is_eq: boolean[];
}

export interface IRawByProject {
    project: number;
    selections: number[];
    is_eq: boolean[];
}

export interface IRawStudent {
    id: number;
    email: string;
    full_name: string;
}

export interface IStudent extends IRawStudent {
    comment?: string;
}

export interface IStudentProjectChoice extends IStudent {
    isEq: boolean;
}

export interface IStudentMap {
    [key: number]: IStudent;
}

export interface IProject {
    id: number;
    name: string;
    supervisor_name: string;
    supervisor_email: string;
}

export interface IProjectMap {
    [key: number]: IProject;
}

export interface IProjectRow extends IProject {
    choices: IStudentProjectChoice[][];
}

export interface IStudentRow extends IStudent {
    choices: Array<{ project: number, eq_last: boolean }>;
}

export class SessionReport {
    public readonly students: IStudent[];
    public readonly projects: IProject[];
    public readonly studentMap: IStudentMap;
    public readonly projectMap: IProjectMap;
    public readonly studentRows: IStudentRow[];
    public readonly projectRows: IProjectRow[];
    public readonly maxChoices: number;

    protected raw: ISessionReportRaw | undefined;

    constructor(report: ISessionReportRaw | undefined) {
        this.raw = report;
        this.students = allStudents(this.raw);
        this.projects = allProjects(this.raw);
        this.studentMap = studentMap(this.students);
        this.projectMap = projectMap(this.projects);
        this.studentRows = studentRows(this.raw, this.studentMap);
        this.projectRows = projectRows(this.raw, this.studentMap, this.projectMap);
        this.maxChoices = maxChoices(this.projectRows);
    }

    public choicesRange(): number[] {
        return _.range(0, this.maxChoices);
    }

    public session(): ISession {
        if (!this.raw) {
            return {
                id: -1,
                name: "...",
                supervisor_email: "",
                supervisor_name: "",
            };
        }
        return this.raw.session;
    }
}

function allStudents(report: ISessionReportRaw | undefined): IStudent[] {
    if (!report) { return []; }
    const all = _.map(report.students, (it: IRawStudent) => {
        let comment: string | undefined = report.comments[it.id];
        if (comment === "") { comment = undefined; }
        const out = {
            ...it,
            comment,
        } as IStudent;
        return out;
    }) as any as IStudent[];
    return _.sortBy(all, ["id"]);
}

function allProjects(report: ISessionReportRaw | undefined): IProject[] {
    if (!report) { return []; }
    return report.projects;
}

function studentMap(students: IStudent[]): IStudentMap {
    return _.fromPairs(_.map(students, (it) => [it.id, it]));
}

function projectMap(projects: IProject[]): IProjectMap {
    return _.fromPairs(_.map(projects, (it: IProject) => [it.id, it]));
}

function studentRows(report: ISessionReportRaw | undefined, students: IStudentMap): IStudentRow[] {
    if (!report) { return []; }

    const out = _.map(report.by_student, (it: IRawByStudent) => {
        const clonedEq = it.is_eq.slice(0);
        clonedEq.unshift(false); // Make the arrays equal length for convenience
        const choices = _.map(_.zip(it.choices, clonedEq), (inner: [number, boolean]) => {
            return {
                eq_last: inner[1],
                project: inner[0],
            };
        });
        return {
            choices,
            ...students[it.student],
        };
    }) as any as IStudentRow[]; // Completely overrule Typescript here - for some reason it gets this totally wrong.

    return _.sortBy(out, ["id"]);
}

function projectRows(report: ISessionReportRaw | undefined, students: IStudentMap,
                     projects: IProjectMap): IProjectRow[] {
    if (!report) { return []; }

    const out = _.map(report.by_project, (it: IRawByProject) => {
        return {
            choices: _.map(_.zip(it.selections, it.is_eq), (outerEntry: [number[], boolean[]]) => {
                return _.map(_.zip(outerEntry[0], outerEntry[1]), (entry: [number, boolean]) => {
                    const res: IStudentProjectChoice = {
                        isEq: entry[1],
                        ...students[entry[0]],
                    };
                    return res;
                });
            }),
            ...projects[it.project],
        };
    }) as any as IProjectRow[]; // Completely overrule Typescript here - for some reason it gets this totally wrong.

    return _.sortBy(out, ["supervisor", "name", "id"]);
}

function maxChoices(pRows: IProjectRow[]): number {
    return _.reduce(pRows, (curr: number, it: IProjectRow) => {
        if (it.choices.length > curr) { return it.choices.length; }
        return curr;
    }, 0);
}
