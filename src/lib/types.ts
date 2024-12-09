export interface SignatureMeta {
	id: string;
	name: string;
}

export interface Signature extends SignatureMeta {
	ts_created: string;
	ts_modified: string;
	approved: boolean;
	signature: string;
}
