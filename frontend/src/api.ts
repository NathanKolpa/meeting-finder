import {Meeting} from "./models";

function pad(a: any, b: number) {
    return ([1e15] + a).slice(-b)
}

export async function fetchMeetings(): Promise<Meeting[]> {
    const request = await fetch(import.meta.env.VITE_API_URL);
    const response = await request.json() as ApiMeeting[];

    let id = 0;

    return response.map(apiMeeting => {

        let isRecurring = !!apiMeeting.time.recurring;
        let formattedTime = '';

        if (isRecurring) {
            let timeStr = apiMeeting.time.recurring.time as string;
            let [hours, minutes, _seconds] = timeStr.split(':')

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
            distance: 0,
            source: apiMeeting.source,
            time: apiMeeting.time.recurring.time,
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
    time: string;
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
