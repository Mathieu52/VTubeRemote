let listArrowDefaultInnerHTML = "";

document.addEventListener('DOMContentLoaded', () => {
    // Fetch the template HTML file
    fetch('list_arrow_template.html')
        .then(response => response.text())
        .then(html => {
            const temp = document.createElement('div');
            temp.innerHTML = html.trim();

            // Get the template content
            const templateContent = temp.firstElementChild;

            // Clone the template content
            listArrowDefaultInnerHTML = templateContent.innerHTML;

            for (const element of document.getElementsByTagName("list-arrow")) {
                element.innerHTML = listArrowDefaultInnerHTML;
            }
        });
});

class ListArrow extends HTMLElement {
    constructor() {
        super();

        this.innerHTML = listArrowDefaultInnerHTML;

        this.arrowUp = this.getElementsByClassName("move_arrow_up")[0];
        this.arrowDown = this.getElementsByClassName("move_arrow_down")[0];

        this.arrowUp.addEventListener("click", () => {this.moveUp()});
        this.arrowDown.addEventListener("click", () => {this.moveDown()});
    }

    get movableParent() {
        let element = this.parentElement;

        while(element !== document.body && element.parentElement != null) {
            console.debug("Element", element);
            if (element.parentElement.classList.contains("arrowed-list")) {
                return element;
            }

            element = element.parentElement;
        }

        return null;
    }

    get parentList() {
        const movableParent = this.movableParent;
        return movableParent !== null ? movableParent.parentElement : null;
    }

    moveUp() {
        this.parentList.insertBefore(this.movableParent, this.movableParent.previousSibling);
    }

    moveDown() {
        this.parentList.insertBefore(this.movableParent, this.movableParent.nextSibling);
        this.parentList.insertBefore(this.movableParent.nextSibling, this.movableParent);
    }
}

customElements.define("list-arrow", ListArrow);