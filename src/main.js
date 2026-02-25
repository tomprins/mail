// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.core.invoke;


async function get_messages() {
    let data = await invoke('get_messages');
    data = JSON.parse(data)
    for (let index = 0; index < data.length; index++) {
        const mail = data[index];

        const mailDiv = `<div class="card">
    <div class="card-body">
        <h5 class="card-title">${mail.document.from}</h5>
        <p class="card-text">${mail.document.subject}</p>
    </div>
</div>`

        let mail_list = document.getElementById("mail-list")
        mail_list.insertAdjacentHTML("beforeend", mailDiv)
    }
}

get_messages()