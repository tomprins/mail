// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.core.invoke;

const searchbarMail = document.getElementById("mail-searchbar").addEventListener("input", searchMail);

function searchMail(event) {
    console.log(`searching mails for '${event.target.value}'`)
    setMailList(event.target.value)
}

async function setMailList(query = "*") {
    let mails = await invoke('get_mails', { query: query });

    const mailList = document.getElementById("mail-list")
    mailList.replaceChildren(); // Remove all previously added mails.

    let tempMail;
    for (let index = 0; index < mails.length; index++) {
        const mail = mails[index];
        const card = createMaillistEntry(mail.document.from, mail.document.subject)
        mailList.appendChild(card)

        tempMail = mail;
    }

    const detailView = document.getElementById("detail-view")
    detailView.replaceChildren()

    const mailBody = DOMPurify.sanitize(tempMail.document.raw_body);
    detailView.innerHTML = mailBody
}

function createMaillistEntry(from, subject) {
    const cardDiv = document.createElement("div")
    cardDiv.className = "card"

    const cardBody = document.createElement("div")
    cardBody.className = "card-body"

    const cardTitle = document.createElement("h5")
    cardTitle.className = "card-title"
    cardTitle.textContent = from

    const cardText = document.createElement("p")
    cardText.className = "card-text"
    cardText.textContent = subject

    cardBody.appendChild(cardTitle)
    cardBody.appendChild(cardText)
    cardDiv.appendChild(cardBody)

    return cardDiv
}


setMailList()
