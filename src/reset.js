// reset subtitle after message
export function reset(prev) {
    setTimeout(function(){
        document.getElementById('subtitle').innerHTML = prev;
        document.getElementById('subtitle').style = "";
    }, 1500);
}
