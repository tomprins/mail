// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.core.invoke;

document.addEventListener("DOMContentLoaded", initialize);

function initialize() {
  document.getElementById("mail-searchbar").addEventListener("click", function (event) {
    document.getElementById("mail-searchbar-input").focus()
  })

  document.getElementById("mail-searchbar-input").addEventListener("input", searchMail);
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

    const maillistEntry = createMaillistEntry(mail);
    mailList.appendChild(maillistEntry);
  }
}

function maillistMailClick(event, mail) {
  const detailView = document.getElementById("detail-view");
  detailView.replaceChildren();

  const mailBody = DOMPurify.sanitize(mail.document.raw_body);
  detailView.innerHTML = mailBody;
}

function createMaillistEntry(mail) {
  const maillistMail = document.createElement("div");
  maillistMail.className = "maillist-mail card";

  const mailBody = document.createElement("div");
  mailBody.className = "maillist-mail-body card-body";

  const mailTitle = document.createElement("h5");
  mailTitle.className = "card-title";
  mailTitle.textContent = mail.document.from;

  const mailText = document.createElement("p");
  mailText.className = "card-text";
  mailText.textContent = mail.document.subject;

  mailBody.appendChild(mailTitle);
  mailBody.appendChild(mailText);
  maillistMail.appendChild(mailBody);

  maillistMail.addEventListener("mousedown", (event) => {
    maillistMailClick(event, mail);
  });

  maillistMail.addEventListener("mouseenter", (event) => {
    maillistMail.style.backgroundColor = "#303030"
    maillistMail.style.borderColor = "#575757"
  })

  maillistMail.addEventListener("mouseleave", (event) => {
    maillistMail.style.backgroundColor = "#262626"
    maillistMail.style.borderColor = "#474747"
  })

  return maillistMail;
}

setMaillist();
