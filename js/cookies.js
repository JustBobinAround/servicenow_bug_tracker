export function cookie_exists(name) {
    const decoded_cookie = decodeURIComponent(document.cookie);
    const cookie_array = decoded_cookie.split(';');

    for (let i = 0; i < cookie_array.length; i++) {
        let cookie = cookie_array[i];
        while (cookie.charAt(0) === ' ') {
            cookie = cookie.substring(1);
        }
        if (cookie.indexOf(name + "=") === 0) {
            return true; 
        }
    }

    return false; 
}

export function set_cookie(name, value, daysToExpire) {
    const expiration_date = new Date();
    expiration_date.setTime(expiration_date.getTime() + (daysToExpire * 24 * 60 * 60 * 1000));
    const expires = "expires=" + expiration_date.toUTCString();
    document.cookie = name + "=" + value + ";" + expires + ";path=/";
}

export function get_cookie(name) {
    const decoded_cookie = decodeURIComponent(document.cookie);
    const cookie_array = decoded_cookie.split(';');
    
    for (let i = 0; i < cookie_array.length; i++) {
        let cookie = cookie_array[i];
        while (cookie.charAt(0) === ' ') {
            cookie = cookie.substring(1);
        }
        if (cookie.indexOf(name + "=") === 0) {
            return cookie.substring(name.length + 1, cookie.length);
        }
    }
    
    return "";
}


export function delete_cookie(name) {
    document.cookie = name + '=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
}

