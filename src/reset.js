// reset subtitle after message
export function reset(prev, time) {
    setTimeout(function(){
        document.getElementById('subtitle').innerHTML = prev;
        document.getElementById('subtitle').style = "";
    }, time);
}
