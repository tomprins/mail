// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.core.invoke;

document.addEventListener("DOMContentLoaded", initialize);
const scrollbarWidth = getScrollbarWidth();

function initialize() {
  const newMailPopup = document.getElementById("new-mail-popup");

  document
    .getElementById("mail-searchbar")
    .addEventListener("click", function (event) {
      document.getElementById("mail-searchbar-input").focus();
    });

  document
    .getElementById("mail-searchbar-input")
    .addEventListener("input", searchMail);

  document
    .getElementById("new-mail-button")
    .addEventListener("mousedown", function (event) {
      newMailPopup.classList.remove("hidden");
    });

  document
    .getElementById("new-mail-popup-close-icon")
    .addEventListener("mousedown", function (event) {
      newMailPopup.classList.add("hidden");
    });

  // TODO to class list
  document
    .getElementById("new-mail-popup-add-cc")
    .addEventListener("mousedown", function (event) {
      document
        .getElementById("new-mail-popup-cc-row")
        .toggleAttribute("hidden");

      document
        .getElementById("new-mail-popup-bcc")
        .setAttribute("hidden", "hidden");
    });

  document
    .getElementById("new-mail-popup-add-bcc")
    .addEventListener("mousedown", function (event) {
      document.getElementById("new-mail-popup-bcc").toggleAttribute("hidden");
    });
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
  const detailView = document.getElementById("detail-view-mail-body");
  detailView.replaceChildren();

  const mailBody = DOMPurify.sanitize(mail.document.raw_body);
  detailView.innerHTML = mailBody;

  // reset scroll
  document.getElementById("detail-view-mail-body").scroll(0, 0);

  // update width of create mail popup when mail has a scrollbar.
  const newMailPopup = document.getElementById("new-mail-popup");
  var right = 20;

  if (hasVerticalScrollbar(detailView)) {
    right += scrollbarWidth;
  }

  newMailPopup.style.cssText = `right: ${right}px;`;
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

  return maillistMail;
}

function getScrollbarWidth() {
  // Creating invisible container
  const outer = document.createElement("div");
  outer.style.visibility = "hidden";
  outer.style.overflow = "scroll"; // forcing scrollbar to appear
  outer.style.msOverflowStyle = "scrollbar"; // needed for WinJS apps
  document.body.appendChild(outer);

  // Creating inner element and placing it in the container
  const inner = document.createElement("div");
  outer.appendChild(inner);

  // Calculating difference between container's full width and the child width
  const scrollbarWidth = outer.offsetWidth - inner.offsetWidth;

  // Removing temporary elements from the DOM
  outer.parentNode.removeChild(outer);

  return scrollbarWidth;
}

function hasVerticalScrollbar(element) {
  return element.scrollHeight > element.clientHeight;
}

setMaillist();
