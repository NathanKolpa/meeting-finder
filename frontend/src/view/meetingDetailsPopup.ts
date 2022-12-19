import {Meeting} from "../models";
import MicroModal from "micromodal";
import {getLogoImgUrlByOrg} from "./logo";

export class MeetingDetailsPopup {
    private logo: HTMLImageElement;
    private title: HTMLElement;
    private notes: HTMLElement;
    private country: HTMLElement;
    private countryRow: HTMLElement;
    private region: HTMLElement;
    private regionRow: HTMLElement;
    private locationInfoTable: HTMLElement;
    private locationNotes: HTMLElement;
    private source: HTMLAnchorElement;

    public constructor(private id: string) {
        const root = document.getElementById(id)!;

        this.logo = root.getElementsByClassName('logo').item(0) as HTMLImageElement;
        this.title = root.getElementsByClassName('titleText').item(0) as HTMLElement;
        this.notes = root.getElementsByClassName('notes').item(0) as HTMLElement;
        this.country = root.getElementsByClassName('country').item(0) as HTMLElement;
        this.countryRow = root.getElementsByClassName('countryRow').item(0) as HTMLElement;
        this.region = root.getElementsByClassName('region').item(0) as HTMLElement;
        this.regionRow = root.getElementsByClassName('regionRow').item(0) as HTMLElement;
        this.locationInfoTable = root.getElementsByClassName('location-info').item(0) as HTMLElement;
        this.source = root.getElementsByClassName('source').item(0) as HTMLAnchorElement;
        this.locationNotes = root.getElementsByClassName('location-notes').item(0) as HTMLElement;
    }

    public showMeeting(meeting: Meeting) {
        this.title.innerText = meeting.name;

        if (meeting.notes) {
            this.notes.hidden = false;
            this.notes.innerText = meeting.notes;
        }
        else {
            this.notes.hidden = true;
        }

        this.locationInfoTable.hidden = !meeting.country && !meeting.region;

        if (meeting.country) {
            this.countryRow.hidden = false;
            this.country.innerText = meeting.country;
        }
        else {
            this.countryRow.hidden = true;
        }

        if (meeting.region) {
            this.regionRow.hidden = false;
            this.region.innerText = meeting.region;
        }
        else {
            this.regionRow.hidden = true;
        }

        if (meeting.locationNotes) {
            this.locationNotes.hidden = false;
            this.locationNotes.innerText = meeting.locationNotes;
        }
        else {
            this.locationNotes.hidden = true;
        }

        this.source.href = meeting.source;
        this.source.title = meeting.source;

        this.logo.src = getLogoImgUrlByOrg(meeting.org);

        MicroModal.show(this.id, {
            awaitCloseAnimation: true,
        })
    }
}