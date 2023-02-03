export type ID = number;
export type UUID = string;
export type Time = number;

export type newSignaturesResponse = {
	data?: {
		_id?: ID;
		_ts?: Time;
		user_identifier?: UUID;
		name?: string;
		status?: string;
		ts_created?: Time;
		ts_moderated?: Time;
		signature?: {
			penColor?: string;
			dotSize?: number;
			maxWidth?: number;
			minWidth?: number;
			points?: {
				time?: number;
				x?: number;
				y?: number;
				pressure?: number;
			};
		};
	}[];
	after?: string;
	before?: string;
};
