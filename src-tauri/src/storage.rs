#[derive(Debug)]
struct Credential {
    url: String,
    username: String,
    password: String,
    note: String,
}

#[derive(Debug)]
struct Secret {
    key: String,
    value: String,
    note: String,
}

#[tauri::command]
pub fn save_credential(url: String, username: String, password: String, note: String) {
    let credentials = Credential {
        url,
        username,
        password,
        note,
    };

    println!("Storing credentials: {:?}", credentials);
}

#[tauri::command]
pub fn save_secret(key: String, value: String, note: String) {
    let secret = Secret {
        key,
        value,
        note,
    };

    println!("Storing secret: {:?}", secret);
}
