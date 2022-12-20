export interface SearchQuery {
    location: string;
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

            this.submit({
                location: data.get('location') as string
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