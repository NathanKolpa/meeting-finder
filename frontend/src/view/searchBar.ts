export interface SearchQuery {
    location: string | null;
    distance: number
}

export type SearchCallback = ((query: SearchQuery) => void) | null;

export class SearchBar {
    private readonly form: HTMLFormElement;
    private readonly locationFeedback: HTMLElement;
    private searchCallback: SearchCallback = null;

    public constructor(formId: string) {
        this.form = document.getElementById(formId) as HTMLFormElement;
        this.locationFeedback = this.form.getElementsByClassName('location-feedback')[0] as HTMLElement;

        this.form.onsubmit = (e) => {
            e.preventDefault();
            const data = new FormData(this.form);

            let location = data.get('location') as string;
            let distance = parseFloat(data.get('distance') as string);

            // don't search on location when distance is set to "all"
            if (isNaN(distance)) {
                distance = 0;
                location = "";
            }

            this.submit({
                location: location == "" ? null : location,
                distance: distance
            });
        }
    }

    private submit(query: SearchQuery) {
        this.clearErrors();

        if (this.searchCallback) {
            this.searchCallback(query);
        }
    }

    public setOnSearchCallback(callback: SearchCallback) {
        this.searchCallback = callback;
    }

    public setLocationError(message: string) {
        this.locationFeedback.innerText = message;
        this.locationFeedback.hidden = false;
    }

    public clearErrors() {
        this.locationFeedback.hidden = true;
    }
}
