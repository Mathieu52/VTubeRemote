function isTouchPointer() {
    return matchMedia("(pointer: coarse)").matches;
}

if (isTouchPointer()) {
    $(document).on("click", '.animated_click', function () {
        startAnimatedClick($(this).get(0), true, true);
    });

    $(document).on("click", '.animated_click_darken_only', function () {
        startAnimatedClick($(this).get(0), false, true);
    });

    $(document).on("click", '.animated_click_scaling_only', function () {
        startAnimatedClick($(this).get(0), true, false);
    });

    $(document).on("animationend", '.animated_click', function () {
        clearAnimationClass($(this).get(0), true, true);
    });

    $(document).on("animationend", '.animated_click_darken_only', function () {
        clearAnimationClass($(this).get(0), false, true);
    });

    $(document).on("animationend", '.animated_click_scaling_only', function () {
        clearAnimationClass($(this).get(0), true, false);
    });

}

// Creates the callback function for the observer
function startAnimatedClick(element, scale, darken) {
    if (!scale && !darken) {
        return;
    }

    let className = scale && darken ? "phone_click_animation" : (scale ? "phone_click_scaling_animation" : "phone_click_brightness_animation");

    element.classList.remove(className);
    void element.offsetWidth;
    element.classList.add(className);
}

function clearAnimationClass(element, scale, darken) {
    if (!scale && !darken) {
        return;
    }

    let className = scale && darken ? "phone_click_animation" : (scale ? "phone_click_scaling_animation" : "phone_click_brightness_animation");
    element.classList.remove(className);
}