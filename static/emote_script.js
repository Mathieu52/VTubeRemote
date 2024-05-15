let emoteDefaultInnerHTML = "";

document.addEventListener('DOMContentLoaded', () => {
    // Fetch the template HTML file
    fetch('emote_template.html')
        .then(response => response.text())
        .then(html => {
            const temp = document.createElement('div');
            temp.innerHTML = html.trim();
            // Get the template content
            const templateContent = temp.firstElementChild;

            // Clone the template content
            emoteDefaultInnerHTML = templateContent.innerHTML;
        });
});

export const EmoteStatus = {
    ACTIVE: 'active',
    INACTIVE: 'inactive',
    NONE: 'no-status',
};

export class EmoteElement extends HTMLElement {
    static #XML_HTTP_REQUEST = new XMLHttpRequest();
    constructor(id) {
        super();

        this.innerHTML = emoteDefaultInnerHTML;

        this.icon = this.getElementsByClassName("emote_icon")[0];
        this.statusLight = this.getElementsByClassName("status_light")[0];

        this.status = EmoteStatus.NONE;
        this.id = id;
    }

    get id() {
        return this.getAttribute('id');
    }

    set id(id) {
        this.setAttribute('id', id);
        this.#fetchIconImage(id);
    }

    get #imagePath() {
        return `resources/${this.id}.png`;
    }

    #fetchIconImage(id) {
        let iconPath = this.#imagePath;

        fetch(iconPath)
            .then((response) => {
                if (response.ok) {
                    this.icon.style.backgroundImage = `url("${iconPath}")`;
                } else {
                    this.icon.style.backgroundImage = 'url("resources/no_image_found.png")'
                }
            });
    }

    get status() {
        for (const possible_status of Object.values(EmoteStatus)) {
            if (this.statusLight.classList.contains(possible_status)) {
                return EmoteStatus[possible_status];
            }
        }

        console.error(`Emote element id(${this.id})'s status light is missing it's status class, returning null`);
        return null;
    }

    set status(status) {
        if (!Object.values(EmoteStatus).includes(status)) {
            console.log(`Program attempted to set invalid status (${status}) to Emote element id(${this.id})`);
            return;
        }

        for (const possible_status of Object.values(EmoteStatus)) {
            this.statusLight.classList.remove(possible_status);
        }

        this.statusLight.classList.add(status);
    }

    get #togglePath() {
        return `trigger/${this.id}`;
    }

    toggle() {
        switch (this.status) {
            case EmoteStatus.ACTIVE:
                this.status = EmoteStatus.INACTIVE
                break;
            case EmoteStatus.INACTIVE:
                this.status = EmoteStatus.ACTIVE
                break;
        }

        EmoteElement.#XML_HTTP_REQUEST.open("GET", this.#togglePath, true);
        EmoteElement.#XML_HTTP_REQUEST.send();
    }
}

customElements.define("emote-element", EmoteElement);