import {Meeting} from "../models";
import {getLogoImgUrlByOrg} from "./logo";
import {MeetingCallback} from "./callback";

interface ResultListItemActions {
}

export class ResultsList {
    private resultsElement: HTMLElement;
    private loadingText: HTMLElement;
    private actions: { [id: number]: ResultListItemActions } = {};
    private isLoading = true;
    private viewOnMapCallback: MeetingCallback = null;
    private showInfoCallback: MeetingCallback = null;

    public constructor(resultsId: string, loadingId: string) {
        this.resultsElement = this.findElementOrThrow(resultsId);
        this.loadingText = this.findElementOrThrow(loadingId);
    }

    private findElementOrThrow(id: string) {
        let element = document.getElementById(id);

        if (!element) {
            throw new Error(`Cannot find element #${id}`);
        }
        return element;
    }

    private clear() {
        this.actions = {};
        this.resultsElement.replaceChildren();
    }

    public addMeeting(meeting: Meeting) {
        if (this.isLoading) {
            throw new Error('Cannot add meetings while loading, call setLoading(false) first')
        }

        function setTextOrRemoveIfNull(value: any, element: Element) {
            if (value) {
                element.textContent = value;
            } else {
                element.remove();
            }
        }

        let li = document.createElement("li");

        li.className = 'result';
        li.innerHTML =
            `
            <img class="logo" alt="Organization logo">
            <div class="container">
                <div class="inner">
                    <div class="title">
                        <span class="name">[Title]</span>
                        <span class="time">[time]</span>
                    </div>
                    
                    <ul class="subtext">
                        <li class="country">[Country]</li>
                        <li class="region">[Region]</li>
                        <li class="distance">[Distance] <span>km</span></li>
                        <li class="online">[online]</li>
                    </ul>
                    
                    <ul class="actions">
                        <li class="link info">Info</li>
                        <li class="separator">|</li>
                        <li class="link focus">View on map</li>
                    </ul>
                </div>
            </div>
            `;

        let logo = li.getElementsByClassName('logo')[0]! as HTMLImageElement;
        let name = li.getElementsByClassName('name')[0]!;
        let country = li.getElementsByClassName('country')[0]!;
        let region = li.getElementsByClassName('region')[0]!;
        let distance = li.getElementsByClassName('distance')[0]!;
        let time = li.getElementsByClassName('time')[0]!;
        let online = li.getElementsByClassName('online')[0]!;

        let focus = li.getElementsByClassName('focus')[0]! as HTMLElement;
        let info = li.getElementsByClassName('info')[0]! as HTMLElement;

        setTextOrRemoveIfNull(meeting.name, name);
        setTextOrRemoveIfNull(meeting.country, country);
        setTextOrRemoveIfNull(meeting.region, region);
        setTextOrRemoveIfNull(meeting.distance ? meeting.distance + ' km' : null, distance);
        setTextOrRemoveIfNull(meeting.formattedTime, time);
        setTextOrRemoveIfNull(meeting.online ? 'online' : null, online);

        if (!meeting.position) {
            focus.classList.add('disabled');
            focus.title = 'No location available'
        } else {
            focus.onclick = () => {
                if (this.viewOnMapCallback) {
                    this.viewOnMapCallback(meeting);
                }
            };
        }

        info.onclick = () => {
            if (this.showInfoCallback) {
                this.showInfoCallback(meeting);
            }
        }


        logo.src = getLogoImgUrlByOrg(meeting.org);

        this.actions[meeting.id] = {};

        this.resultsElement.appendChild(li);
    }

    public addMeetings(meetings: Meeting[]) {
        for (const meeting of meetings) {
            this.addMeeting(meeting);
        }
    }

    public setLoading(isLoading: boolean) {
        this.loadingText.hidden = !isLoading;

        if (isLoading) {
            this.clear();
        }

        this.isLoading = isLoading;
    }

    public setViewOnMapCallback(callback: MeetingCallback) {
        this.viewOnMapCallback = callback;
    }

    public setShowInfoCallback(callback: MeetingCallback) {
        this.showInfoCallback = callback;
    }
}