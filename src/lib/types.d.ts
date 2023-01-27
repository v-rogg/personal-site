export type signatureData = {
	ref: { "@ref": { id: string; collection: [] } } | null;
	ts: number | null;
	data: { name: string | null; signature: [] | null }; // signature is centered
};
