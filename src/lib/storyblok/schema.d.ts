export type DATETIME = string;

export type Company = {
	title: string;
	url: string;
	jobs: Experience[];
	hidden: boolean;
}

export type Experience = {
	title: string;
	from: DATETIME;
	to: DATETIME | null;
	skills: string[];
	location: string;
	employment_type: string
	tags: string[];
}