import {enableScroll, disableScroll} from './scroll_script.js'
import {EmoteElement, EmoteStatus} from "./emote_script.js";

let statusDiv = document.getElementById('status');
let editButton = document.getElementById('edit_button');
let emoteRowDiv = document.getElementById('emotes-row');

function isTouchPointer() {
    return matchMedia("(pointer: coarse)").matches;
}

document.addEventListener('DOMContentLoaded', () => {
    // Fetch the template HTML file
    fetch('emote_template.html')
        .then(response => response.text())
        .then(html => {
            /*
            // Create a temporary element to hold the template content
            const temp = document.createElement('div');
            temp.innerHTML = html.trim();

            // Get the template content
            const templateContent = temp.firstElementChild;

            // Clone the template content
            const clone = document.importNode(templateContent, true);
            document.body.appendChild(clone);

             */
            init();
        });
});

function getMouseX(e) {
    if (e.type === 'touchstart' || e.type === 'touchmove' || e.type === 'touchend' || e.type === 'touchcancel') {
        var touch = e.originalEvent.touches[0] || e.originalEvent.changedTouches[0];
        return touch.pageX;
    } else if (e.type === 'mousedown' || e.type === 'mouseup' || e.type === 'mousemove' || e.type === 'mouseover' || e.type === 'mouseout' || e.type == 'mouseenter' || e.type == 'mouseleave') {
        return e.clientX;
    }
}

function getMouseY(e) {
    if (e.type === 'touchstart' || e.type === 'touchmove' || e.type === 'touchend' || e.type === 'touchcancel') {
        var touch = e.originalEvent.touches[0] || e.originalEvent.changedTouches[0];
        return touch.pageY;
    } else if (e.type === 'mousedown' || e.type === 'mouseup' || e.type === 'mousemove' || e.type === 'mouseover' || e.type === 'mouseout' || e.type == 'mouseenter' || e.type == 'mouseleave') {
        return e.clientY;
    }
}

function moveElementOnMouse(mouseEvent, element) {
    element.style.left = `${getMouseX(mouseEvent) - element.getBoundingClientRect().width / 2.0}px`;
    element.style.top = `${getMouseY(mouseEvent) - element.getBoundingClientRect().height / 2.0}px`;
}

function createTemplate() {
    let temp = document.getElementsByTagName("template")[0];
    let item = temp.content.querySelector("div");

    let node = document.importNode(item, true);

    emoteRowDiv.appendChild(node);

    return node;
}

let STATE = {
    connected: false,
    editing: false,
}

function set_status_light_state(emote, active) {
    let status_light = emote.getElementsByClassName("status_light")[0];

    if (active == null) {
        status_light.classList.replace("active", "no_state");
        status_light.classList.replace("inactive", "no_state");
    } else {
        if (active) {
            status_light.classList.replace("no_state", 'active');
            status_light.classList.replace("inactive", 'active');
        } else {
            status_light.classList.replace("no_state", 'inactive');
            status_light.classList.replace("active", 'inactive');
        }
    }
}

function get_emote_state(emote) {
    let status_light = emote.getElementsByClassName("status_light")[0];
    if (status_light.classList.contains("active")) {
        return true;
    } else if (status_light.classList.contains("inactive")) {
        return false;
    } else {
        return null;
    }
}

function insertEmoteBefore(emote, location) {
    emoteRowDiv.insertBefore(emote, location);
}

function insertEmoteAfter(emote, location) {
    emoteRowDiv.insertBefore(emote, location);
    emoteRowDiv.insertBefore(location, emote);
}

function setEmoteListeners(emote) {
    emote.icon.addEventListener("click", () => {
        if (STATE.connected) {
            emote.toggle();
        }
    })

    // Get all images within elements with class "blur_on_hover"
    const emotes = document.getElementsByClassName('emote_container');

    emote.icon.addEventListener('mouseenter', () => {
        if (STATE.editing) return;
        // Apply blur to all images except the one being hovered over
        for (let i = 0; i < emotes.length; i++) {
            let emote_element = emotes[i];
            if (emote !== emote_element) {
                emote_element.classList.add("out_of_focus");
            }
        }
    });

    emote.icon.addEventListener('mouseleave', () => {
        if (STATE.editing) return;
        // Remove blur from all images when mouse leaves the image
        for (let i = 0; i < emotes.length; i++) {
            let emote_element = emotes[i];
            emote_element.classList.remove("out_of_focus");
        }
    });

    let listArrow = emote.getElementsByTagName("list-arrow")[0];
    let upArrow = emote.getElementsByClassName("move_arrow_up")[0];
    let downArrow = emote.getElementsByClassName("move_arrow_down")[0];

    let dragged = false;

    // Not enabled on the phone

    //emote.addEventListener("touchstart", (event) => {if  (STATE.editing) { grabEmote(event); }})
    emote.addEventListener("mousedown", (event) => {
        if (STATE.editing && event.target === emote) {
            grabEmote(event);
        }
    });
    emote.addEventListener("mousemove", (event) => {
        if (dragged && event.target === emote) {
            dragEmote(event);
        }
    });
    //emote.addEventListener("touchmove", (event) => { if (dragged) { dragEmote(event); }});

    emote.addEventListener("mouseup", (event) => {
        if (event.target === emote) {
            dropEmote(event);
        }
    });
    emote.addEventListener("mouseleave", (event) => {
        if (event.target === emote) {
            cancelDrag(event);
        }
    });
    emote.addEventListener("mouseout", (event) => {
        if (event.target === emote) {
            cancelDrag(event);
        }
    });

    //emote.addEventListener("touchend", dropEmote);

    function grabEmote(event) {
        dragged = true;
        emote.classList.add("dragged");
        moveElementOnMouse(event, emote)

        if (isTouchPointer()) {
            disableScroll();
        }
    }

    function dragEmote(event) {
        moveElementOnMouse(event, emote);
    }

    function cancelDrag(event) {
        dragged = false;
        emote.style.left = `0`;
        emote.style.top = '0';
        emote.classList.remove("dragged");
    }

    function dropEmote(event) {
        if (dragged) {
            console.log("Drag end");

            if (STATE.editing) {
                const emotes = document.getElementsByClassName('emote_container');

                let mouseY = getMouseY(event);

                let placed = false;
                for (let i = 0; i < emotes.length; i++) {
                    let emote_element = emotes[i];

                    if (emote_element[i] === emote) {
                        continue;
                    }

                    let emote_element_rect = emote_element.getBoundingClientRect();
                    let emote_element_center = emote_element_rect.y + emote_element_rect.height / 2.0;

                    if (mouseY < emote_element_center) {
                        console.log("Selected", emote_element.getElementsByClassName("emote_title")[0].innerText);
                        insertEmoteBefore(emote, emote_element);
                        placed = true;
                        break;
                    }
                }

                if (!placed) {
                    insertEmoteAfter(emote, emotes[emotes.length - 1]);
                }
            }

            cancelDrag();

            if (isTouchPointer()) {
                enableScroll();
            }
        }
    }
}

function subscribe(uri) {
    let retryTime = 1;

    function connect(uri) {
        const events = new EventSource(uri);

        events.addEventListener("message", async (ev) => {
            console.log("raw data", JSON.stringify(ev.data));
            console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
            const info = JSON.parse(ev.data);

            if (!("type_" in info) || !("id" in info) || !("name" in info) || !("active" in info) || !("time_left" in info)) return;
            let div = document.getElementById(info.id);

            if (div !== null) {
                if (info.type_ === "removed") {
                    emoteRowDiv.removeChild(div);
                    return;
                }
            } else {
                //div = document.createElement("emote-element")
                div = new EmoteElement(info.id);
                emoteRowDiv.appendChild(div);

                setEmoteListeners(div);
            }

            div.status = info.active == null ? EmoteStatus.NONE : (info.active ? EmoteStatus.ACTIVE : EmoteStatus.INACTIVE);

            let status_text = div.getElementsByClassName("status_text")[0];
            if (info.time_left !== null) {
                status_text.innerText = info.time_left / 1000000.0 + 'ms left';
            } else {
                status_text.innerText = '';
            }

            div.getElementsByClassName("emote_title")[0].innerText = info.name;
        });

        events.addEventListener("open", () => {
            setConnectedStatus(true);
            console.log(`connected to event stream at ${uri}`);
            retryTime = 1;
        });

        events.addEventListener("error", () => {
            setConnectedStatus(false);
            events.close();

            let timeout = retryTime;
            retryTime = Math.min(16, retryTime * 2);
            console.log(`connection lost. attempting to reconnect in ${timeout}s`);
            setTimeout(() => connect(uri), (() => timeout * 1000)());
        });
    }

    connect(uri);
}

// Set the connection status: `true` for connected, `false` for disconnected.
function setConnectedStatus(status) {
    STATE.connected = status;
    statusDiv.className = (status) ? "connected" : "reconnecting";

    if (status) {
        enableScroll();
    } else {
        disableScroll()
    }
}

function setEditingStatus(status) {
    STATE.editing = status;

    console.log(editButton.className);

    if (status) {
        $(".emote_container").addClass("animated_click_darken_only");
        editButton.classList.replace("not_editing", "editing");
        emoteRowDiv.classList.replace("not_editing", "editing");
    } else {
        $(".emote_container").removeClass("animated_click_darken_only");
        editButton.classList.replace("editing", "not_editing");
        emoteRowDiv.classList.replace("editing", "not_editing");
    }
}

function toggleEditingStatus() {
    setEditingStatus(!STATE.editing);
}


function init() {
    editButton.addEventListener("click", toggleEditingStatus);
    // Subscribe to server-sent events.
    subscribe("/events");
}