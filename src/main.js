// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.core.invoke;

document.addEventListener("DOMContentLoaded", initialize);

function initialize() {
  document
    .getElementById("mail-searchbar")
    .addEventListener("input", searchMail);
}

function searchMail(event) {
  console.log(`searching mails for '${event.target.value}'`);
  setMaillist(event.target.value);
}

async function setMaillist(query = "*") {
  let mails = await invoke("get_mails", { query: query });

  const mailList = document.getElementById("maillist");
  mailList.replaceChildren(); // Remove all previously added mails.

  for (let index = 0; index < mails.length; index++) {
    const mail = mails[index];

    const maillistEntry = createMaillistEntry(
      mail.document.from,
      mail.document.subject,
    );
    maillistEntry.addEventListener("mousedown", (event) => {
      maillistMailClick(event, mail);
    });

    mailList.appendChild(maillistEntry);
  }
}

function maillistMailClick(event, mail) {
  const detailView = document.getElementById("detail-view");
  detailView.replaceChildren();

  const mailBody = DOMPurify.sanitize(mail.document.raw_body);
  detailView.innerHTML = mailBody;
}

function createMaillistEntry(from, subject) {
  const cardDiv = document.createElement("div");
  cardDiv.className = "maillist-mail card";

  const cardBody = document.createElement("div");
  cardBody.className = "card-body";

  const cardTitle = document.createElement("h5");
  cardTitle.className = "card-title";
  cardTitle.textContent = from;

  const cardText = document.createElement("p");
  cardText.className = "card-text";
  cardText.textContent = subject;

  cardBody.appendChild(cardTitle);
  cardBody.appendChild(cardText);
  cardDiv.appendChild(cardBody);

  return cardDiv;
}

setMaillist();
