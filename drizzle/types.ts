export type ID = string;
export type UUID = string;
export type TS = string;

export type Pagination<T> = {
	data: T;
	after?: string;
	before?: string;
}

export type Signature = {
	id: ID;
	ts?: TS;
	user_identifier?: UUID;
	name?: string;
	approved?: boolean | undefined;
	ts_created?: TS;
	ts_moderated?: TS;
	signature?: SignatureData[] | unknown;
};

export type SignaturesResponse = {
	data?: Signature[];
	after?: string;
	before?: string;
};

export type SignatureData = {
	penColor?: string;
	dotSize?: number;
	maxWidth?: number;
	minWidth?: number;
	velocityFilterWeight?: number;
	points?: {
		time?: number;
		x?: number;
		y?: number;
		pressure?: number;
	}[];
};
