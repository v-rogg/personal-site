export type ID = string;
export type UUID = string;
export type TS = number;

export type Signature = {
	_id?: ID;
	_ts?: TS;
	user_identifier?: UUID;
	name?: string;
	status?: string;
	ts_created?: TS;
	ts_moderated?: TS;
	signature?: SignatureData[];
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
