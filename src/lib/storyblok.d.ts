export type DATETIME = string;

type File = {
	filename: string;
	alt: string;
}

export type Content<T> = {
	id: number;
	content: T;
}

export type BioContent = {
	short_bio: string;
}

export type CvContent = {
	companies: Company[];
}

export type MemoriesSettingsContent = {
	columns: number;
	rows: number
}

export type MemoryContent = {
	title: string;
	img: File;
	flag: File;
	col_span: number;
	row_span: number;
}

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
