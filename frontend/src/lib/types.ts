export interface SignatureMeta {
	id: string;
	approved?: boolean;
}

export interface Signature extends SignatureMeta {
	name: string;
	ts_created: string;
	ts_modified: string;
	approved: boolean;
	signature: string;
}

export type Metadata = {
	title: string;
	subtitle?: string;
	date: string;
	published: boolean;
	preview: string;
	previewImageUrl?: string;
	slug?: string;
	tags?: string[];
	page?: boolean;
};
