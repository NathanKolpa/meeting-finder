import {ResultsList} from "./resultsList";
import {MeetingMap} from "./meetingMap";
import {MeetingDetailsPopup} from "./meetingDetailsPopup";
import MicroModal from 'micromodal';
import {SearchBar} from "./searchBar";

export type PageComponents = [ResultsList, MeetingMap, MeetingDetailsPopup, SearchBar];

export function initialize(): PageComponents {
    MicroModal.init();

    return [
        new ResultsList('result-list'),
        new MeetingMap('map'),
        new MeetingDetailsPopup('meetingModal'),
        new SearchBar('search')
    ]
}