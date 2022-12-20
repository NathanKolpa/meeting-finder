import {ResultsList} from "./resultsList";
import {MeetingMap} from "./meetingMap";
import {MeetingDetailsPopup} from "./meetingDetailsPopup";
import MicroModal from 'micromodal';

export type PageComponents = [ResultsList, MeetingMap, MeetingDetailsPopup];

export function initialize(): PageComponents {
    MicroModal.init();

    return [
        new ResultsList('result-list'),
        new MeetingMap('map'),
        new MeetingDetailsPopup('meetingModal')
    ]
}