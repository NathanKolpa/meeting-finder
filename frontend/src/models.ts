export type Organization =
    'AnonymousAlcoholics'
    | 'DebtorsAnonymous'
    | 'CrystalMethAnonymous'
    | 'CodependentsAnonymous';

export interface MeetingPosition {
    longitude: number;
    latitude: number;
}

export interface Meeting {
    /// A unique id generated by the frontend.
    id: number;

    name: string;
    notes: null | string;
    org: Organization;
    source: string;

    position: MeetingPosition | null;
    country: string | null;
    region: string | null;
    distance: number | null;
    locationName: null | string;
    locationNotes: null | string;
    address: string;

    email: string | null;
    phone: string | null;

    onlineUrl: string | null;
    onlineNotes: string | null;
    online: boolean;

    formattedTime: string;
    recurring: boolean;
    day: string;
    time: string;
    durationInSecs: number;
}
