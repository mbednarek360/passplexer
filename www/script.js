function passGen() {

    // get them
    var service = document.getElementById("service").value;
    var username = document.getElementById("username").value;
    var password = document.getElementById("master").value;

    // verify
    if (service.length == 0 || username.length == 0 || password.length == 0) {
        setMessage("Invalid Fields");
    }

    // invalid fields
    else {

        // hash them
        var SHA1 = new jsSHA("SHA-1", "TEXT");
        SHA1.update(service);   
        service = SHA1.getHash("BYTES");
        SHA1.update(username);
        username = SHA1.getHash("BYTES");
        SHA1.update(password);
        password = SHA1.getHash("BYTES");

        // interlace
        var bytes = ""
        for (var n = 0; n < password.length; n++) {
            bytes += service[n];
            bytes += username[n];
            bytes += password[n];
        }

        // rehash
        var SHA384 = new jsSHA("SHA3-256", "BYTES");
        SHA384.update(bytes);
        outhash = SHA384.getHash("BYTES");

        // transcode
        let out = base91.encode(outhash);

        // copy to clipboard
        navigator.clipboard.writeText(out);

        // clear password
        document.getElementById("master").value = "";

        // show message
        setMessage("Copied to Clipboard");

    }
}

// delay
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

// visual feedback
function setMessage(str) {
    let element = document.getElementById("genbutton");
    let temp = element.innerText;
    element.disabled = true;
    element.innerText = str;
    sleep(2000).then(() => {
        element.innerText = temp;
        element.disabled = false;
    });
}
