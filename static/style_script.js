function isTouchPointer() {
    return matchMedia("(pointer: coarse)").matches;
}

function inViewport(element) {
    const windowRect = new DOMRect(0, 0, window.innerWidth, window.innerHeight);
    const elementRect = element.getBoundingClientRect();

    const elementVisibleX = Math.max(windowRect.left, elementRect.left);
    const elementVisibleY = Math.max(windowRect.top, elementRect.top);

    return new DOMRect(elementVisibleX, elementVisibleY, Math.min(windowRect.right, elementRect.right) - elementVisibleX, Math.min(windowRect.bottom, elementRect.bottom) - elementVisibleY);
}

var checkScrollSpeed = (function(settings){
    settings = settings || {};

    var lastPos, newPos, timer, delta,
        delay = settings.delay || 150; // in "ms" (higher means lower fidelity )

    function clear() {
        lastPos = null;
        delta = 0;
    }

    clear();

    return function(){
        newPos = window.scrollY;
        if ( lastPos != null ){ // && newPos < maxScroll
            delta = (newPos -  lastPos);
        }
        lastPos = newPos;
        clearTimeout(timer);
        timer = setTimeout(clear, delay);

        return delta / (delay / 1000.0);
    };
})();

if (isTouchPointer()) {
    let smoooth_scroll_speed = 0;
    // listen to "scroll" event
    window.onscroll = function () {

        //console.log("scroll");
        const emotes = document.getElementsByClassName('emote_container');
        const emoteContainer = document.getElementById("emotes-row");

        const containerRect = inViewport(emoteContainer);

        const center = containerRect.y + containerRect.height / 2.0;

        const scrollSpeed = checkScrollSpeed();

        smoooth_scroll_speed += 0.1 * (scrollSpeed - smoooth_scroll_speed);
        const scaleEffect = Math.abs(scrollSpeed) > 50 ? Math.abs(smoooth_scroll_speed / 3000.0) : 0;

        const baseScale = 1.0 + Math.min(scaleEffect, 0.1);

        console.log("scroll speed", scrollSpeed);

        for (let i = 0; i < emotes.length; i++) {
            let emote = emotes[i];

            let emoteCenter = emote.getBoundingClientRect().y + emote.getBoundingClientRect().height / 2.0;

            let scale = baseScale / (1.0 + Math.pow(scaleEffect * (center - emoteCenter) / 500, 2));

            emote.style.transform = `scale(${scale})`;
        }
    };
}