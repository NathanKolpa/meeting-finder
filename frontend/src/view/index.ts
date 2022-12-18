import {ResultsList} from "./resultsList";
import {MeetingMap} from "./meetingMap";
import {MeetingDetailsPopup} from "./meetingDetailsPopup";

export type PageComponents = [ResultsList, MeetingMap, MeetingDetailsPopup];

export function initialize(): PageComponents {
    return [
        new ResultsList('results', 'loading'),
        new MeetingMap('map'),
        new MeetingDetailsPopup('popup')
    ]
}