export type Organization = 'AnonymousAlcoholics';

export interface Meeting {
	longitude: number;
	latitude: number;
	org: Organization;

	country: string | null;
	region: string | null;
	distance: number | null;
	conferenceUrl: string | null;
	online: boolean;
	name: string;

}
