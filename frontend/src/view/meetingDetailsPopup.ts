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
    private emailRow: HTMLElement;
    private email: HTMLAnchorElement;
    private phoneRow: HTMLElement;
    private phone: HTMLAnchorElement;
    private locationInfoTable: HTMLElement;
    private locationNotes: HTMLElement;
    private time: HTMLElement;
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
        this.email = root.getElementsByClassName('email').item(0) as HTMLAnchorElement;
        this.emailRow = root.getElementsByClassName('emailRow').item(0) as HTMLElement;
        this.phone = root.getElementsByClassName('phone').item(0) as HTMLAnchorElement;
        this.phoneRow = root.getElementsByClassName('phoneRow').item(0) as HTMLElement;
        this.time = root.getElementsByClassName('time').item(0) as HTMLElement;
    }

    public showMeeting(meeting: Meeting) {
        this.title.innerText = meeting.name;

        if (meeting.notes) {
            this.notes.hidden = false;
            this.setTextWithLinks(this.notes, meeting.notes);
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

        if (meeting.email) {
            this.emailRow.hidden = false;
            this.email.innerText = meeting.email;
            this.email.href = `mailto:${meeting.email}`
        }
        else {
            this.emailRow.hidden = true;
        }

        if (meeting.phone) {
            this.phoneRow.hidden = false;
            this.phone.innerText = meeting.phone;
            this.phone.href = `tel:${meeting.phone}`
        }
        else {
            this.phoneRow.hidden = true;
        }

        this.time.innerText = meeting.formattedTime;
        this.source.href = meeting.source;
        this.source.title = meeting.source;

        this.logo.src = getLogoImgUrlByOrg(meeting.org);

        MicroModal.show(this.id, {
            awaitCloseAnimation: true,
        })
    }

    private setTextWithLinks(element: HTMLElement, text: string) {
        element.replaceChildren();

        const emailRegex = /([a-z0-9._-]+@[a-z0-9.-]+\.[a-z]{2,4})/;
        const urlRegex = /(https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|www\.[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9]+\.[^\s]{2,}|www\.[a-zA-Z0-9]+\.[^\s]{2,})/;

        const regexList = [emailRegex, urlRegex];

        const combinedRegex = new RegExp(regexList.map(r => r.source).join('|'));

        let parts = text.split(combinedRegex).filter(x => x !== undefined);

        for (const part of parts) {
            let textElement: HTMLElement;

            if (part.match(combinedRegex)) {
                let anchorElement = document.createElement('a');
                anchorElement.innerText = part;

                if (part.match(emailRegex)) {
                    anchorElement.href = `mailto:${part}`;
                }
                else if (part.match(urlRegex)) {
                    anchorElement.href = part;
                }

                textElement = anchorElement;
            }
            else {
                textElement = document.createElement('span');
                textElement.innerText = part;
            }

            element.appendChild(textElement);
        }
    }

}