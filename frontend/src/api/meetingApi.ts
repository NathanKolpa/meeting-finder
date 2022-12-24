import { Meeting, MeetingPosition } from "../models";

function pad(a: any, b: number) {
    return ([1e15] + a).slice(-b)
}

export interface MeetingFetchDistanceOptions {
    position: MeetingPosition,
    distance: number
}

export interface MeetingFetchOptions {
    distance?: MeetingFetchDistanceOptions
}

export async function fetchMeetings(opts?: MeetingFetchOptions): Promise<Meeting[]> {
    let url = import.meta.env.VITE_API_URL + '/meetings';

    if (opts) {
        url += '?';

        if (opts.distance) {
            url += `longitude=${encodeURIComponent(opts.distance.position.longitude)}&latitude=${encodeURIComponent(opts.distance.position.latitude)}&distance=${encodeURIComponent(opts.distance.distance)}`;
        }
    }

    const request = await fetch(url);
    const response = await request.json() as ApiSearchMeeting[];

    let id = 0;

    return response.map(searchMeeting => {
        let apiMeeting = searchMeeting.meeting;

        let isRecurring = !!apiMeeting.time.recurring;
        let formattedTime = '';

        if (isRecurring) {
            let hours = apiMeeting.time.recurring.hour;
            let minutes = apiMeeting.time.recurring.minute;

            formattedTime = `Every ${apiMeeting.time.recurring.day} at ${hours}:${minutes}`;

            if (apiMeeting.duration) {
                let seconds = apiMeeting.duration.secs;

                let endHours = Math.floor(seconds / 3600);
                seconds %= endHours;

                let endMinutes = Math.floor(seconds / 60);

                formattedTime += ` - ${pad(endHours + (+hours), 2)}:${pad(endMinutes + (+minutes), 2)}`;
            }
        }

        return {
            name: apiMeeting.name,
            id: ++id,
            notes: apiMeeting.notes,
            day: apiMeeting.time.recurring.day,
            country: apiMeeting.location.country,
            position: apiMeeting.location.position,
            online: apiMeeting.online_options.is_online,
            org: apiMeeting.org,
            address: apiMeeting.location.address,
            region: apiMeeting.location.address,
            onlineUrl: apiMeeting.online_options.online_url,
            distance: searchMeeting.distance,
            source: apiMeeting.source,
            durationInSecs: apiMeeting.duration?.secs,
            locationName: apiMeeting.location.location_name,
            onlineNotes: apiMeeting.online_options.notes,
            locationNotes: apiMeeting.location.location_notes,
            email: apiMeeting.contact.email,
            phone: apiMeeting.contact.phone,
            recurring: isRecurring,
            formattedTime
        };
    })
}

interface ApiSearchMeeting {
    meeting: ApiMeeting;
    distance: number | null;
}

interface ApiMeeting {
    name: string;
    org: ApiOrg;
    notes: null | string;
    source: string;
    contact: ApiContact;
    location: ApiLocation;
    online_options: ApiOnlineOptions;
    time: ApiTime;
    duration: ApiDuration;
}

interface ApiContact {
    email: null | string;
    phone: null | string;
}

interface ApiDuration {
    secs: number;
    nanos: number;
}

interface ApiLocation {
    position: ApiPosition | null;
    location_name: null | string;
    location_notes: null | string;
    country: null | string;
    region: null | string;
    address: string;
}

interface ApiPosition {
    latitude: number;
    longitude: number;
}

interface ApiOnlineOptions {
    online_url: null | string;
    notes: null | string;
    is_online: boolean;
}

enum ApiOrg {
    AnonymousAlcoholics = "AnonymousAlcoholics",
    DebtorsAnonymous = "DebtorsAnonymous",
    CrystalMethAnonymous = "CrystalMethAnonymous",
    CodependentsAnonymous = "CodependentsAnonymous",
}

interface ApiTime {
    recurring: ApiRecurring;
}

interface ApiRecurring {
    day: ApiDay;
    hour: number;
    minute: number;
}

enum ApiDay {
    Friday = "Friday",
    Monday = "Monday",
    Saturday = "Saturday",
    Sunday = "Sunday",
    Thursday = "Thursday",
    Tuesday = "Tuesday",
    Wednesday = "Wednesday",
}
