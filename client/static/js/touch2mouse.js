function touch2mouse(el) 
{
    function touchHandler(event) {
        var touches = event.changedTouches,
            first = touches[0],
            type = "";
        console.log(event.type + ": " + first.screenX + "," + first.screenY + " / " + first.clientX + "," + first.clientY);
        switch(event.type)
        {
            case "touchstart": type = "mousedown"; break;
            case "touchmove":  type = "mousemove"; break;        
            case "touchend":   type = "mouseup";   break;
            case "touchcancel": type = "mouseup";  break;
            default:           return;
        }

        // initMouseEvent(type, canBubble, cancelable, view, clickCount, 
        //                screenX, screenY, clientX, clientY, ctrlKey, 
        //                altKey, shiftKey, metaKey, button, relatedTarget);

        var simulatedEvent = document.createEvent("MouseEvent");
        simulatedEvent.initMouseEvent(type, true, true, window, 1, 
            first.screenX, first.screenY, 
            first.clientX, first.clientY, false, 
            false, false, false, 0/*left*/, null);

        el.dispatchEvent(simulatedEvent);
        event.preventDefault();
        event.stopPropagation();
    }
    el.addEventListener("touchstart", touchHandler, true);
    el.addEventListener("touchmove", touchHandler, true);
    el.addEventListener("touchend", touchHandler, true);
    el.addEventListener("touchcancel", touchHandler, true);    
}
