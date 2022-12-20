import {Meeting} from "../models";
import {getLogoImgUrlByOrg} from "./logo";
import {MeetingCallback} from "./callback";

const PAGE_SIZE = 20;

interface ResultListItemActions {
    remove: () => void
}

export class ResultsList {
    private resultListContainer: HTMLElement;
    private resultsElement: HTMLElement;
    private nextPageButton: HTMLButtonElement;
    private prevPageButton: HTMLButtonElement;

    private loadingText: HTMLElement;
    private actions: { [id: number]: ResultListItemActions } = {};
    private isLoading = true;
    private viewOnMapCallback: MeetingCallback = null;
    private showInfoCallback: MeetingCallback = null;

    private meetings: Meeting[] = [];
    private currentPage = 0;

    public constructor(resultsList: string) {
        this.resultListContainer = this.findElementOrThrow(resultsList);

        this.resultsElement = this.findElementByClass(this.resultListContainer, 'results');
        this.loadingText = this.findElementByClass(this.resultListContainer, 'loading');
        this.nextPageButton = this.findElementByClass(this.resultListContainer, 'next-page') as HTMLButtonElement;
        this.prevPageButton = this.findElementByClass(this.resultListContainer, 'prev-page') as HTMLButtonElement;

        this.nextPageButton.onclick = () => this.nextPage();
        this.prevPageButton.onclick = () => this.prevPage();

        this.updatePaginationDisabledState();
    }

    private findElementOrThrow(id: string) {
        let element = document.getElementById(id);

        if (!element) {
            throw new Error(`Cannot find element #${id}`);
        }
        return element;
    }

    private findElementByClass(parent: HTMLElement, classStr: string): HTMLElement {
        let element = parent.getElementsByClassName(classStr)[0];

        if (!element) {
            throw new Error(`Cannot find element .${classStr}`);
        }
        return element as HTMLElement;
    }

    private clear() {
        this.emptyList();
        this.currentPage = 0;
        this.meetings = [];
    }

    private emptyList() {
        this.actions = {};
        this.resultsElement.replaceChildren();
    }

    private addMeetingToList(meeting: Meeting) {
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

        this.actions[meeting.id] = {
            remove: () => li.remove()
        };

        this.resultsElement.appendChild(li);
    }

    public addMeetings(meetings: Meeting[]) {
        this.meetings = this.meetings.concat(meetings);
        this.updatePaginationDisabledState();

        const { meetingsToAdd, startIndex } = this.getPaginationInfoForPage(this.currentPage);

        for (let i = startIndex; i < meetingsToAdd + startIndex; i++) {
            this.addMeetingToList(this.meetings[i]);
        }
    }

    public nextPage() {
        this.setPage(this.currentPage + 1);
    }

    public prevPage() {
        this.setPage(this.currentPage - 1);
    }

    public setPage(page: number) {
        const { meetingsToAdd, startIndex, exceedsBounds } = this.getPaginationInfoForPage(page);

        if (exceedsBounds) {
            return;
        }

        this.currentPage = page;

        this.updatePaginationDisabledState();
        this.emptyList();

        for (let i = startIndex; i < meetingsToAdd + startIndex; i++) {
            this.addMeetingToList(this.meetings[i]);
        }

        this.resultListContainer.scrollTo({ top: 0 });
    }

    private updatePaginationDisabledState() {
        const nextPageInfo = this.getPaginationInfoForPage(this.currentPage + 1);
        const prevPageInfo = this.getPaginationInfoForPage(this.currentPage - 1);

        this.nextPageButton.disabled = nextPageInfo.exceedsBounds;
        this.prevPageButton.disabled = prevPageInfo.exceedsBounds;

        this.nextPageButton.hidden = this.isLoading;
        this.prevPageButton.hidden = this.isLoading;
    }

    private getPaginationInfoForPage(page: number): { startIndex: number, meetingsToAdd: number, exceedsBounds: boolean } {
        const startIndex = PAGE_SIZE * page;
        const totalPages = Math.ceil(this.meetings.length / PAGE_SIZE);
        const meetingsToAdd = Math.min(Math.max(this.meetings.length - startIndex, 0), PAGE_SIZE);
        const exceedsBounds = (page > totalPages - 1 || page < 0) && page != 0;

        return {
            startIndex,
            meetingsToAdd,
            exceedsBounds
        };
    }

    public setMeetings(meetings: Meeting[]) {
        this.clear();
        this.addMeetings(meetings);
    }

    public setLoading(isLoading: boolean) {
        this.loadingText.hidden = !isLoading;
        this.isLoading = isLoading;

        this.updatePaginationDisabledState();
    }

    public setViewOnMapCallback(callback: MeetingCallback) {
        this.viewOnMapCallback = callback;
    }

    public setShowInfoCallback(callback: MeetingCallback) {
        this.showInfoCallback = callback;
    }
}