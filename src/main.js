// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.core.invoke;

document.addEventListener("DOMContentLoaded", initialize);
const scrollbarWidth = getScrollbarWidth();
const topbarIndexLevels = ["index-0", "index-1", "index-2", "index-3"];

function initialize() {
  // list view
  const newMailPopup = document.getElementById("new-mail-popup");
  const mailSearchbarInput = document.getElementById("mail-searchbar-input");

  document
    .getElementById("mail-searchbar")
    .addEventListener("click", function (event) {
      mailSearchbarInput.focus();
    });

  mailSearchbarInput.addEventListener("input", searchMail);

  document
    .getElementById("new-mail-button")
    .addEventListener("mousedown", function (event) {
      newMailPopup.classList.remove("closed", "minimized");
      setIndexLevel(subjectRow, "index-1");
    });

  // create mail popup
  const toInput = document.getElementById("new-mail-popup-to");
  const CCRow = document.getElementById("new-mail-popup-cc-row");
  const BCCRow = document.getElementById("new-mail-popup-bcc");
  const subjectRow = document.getElementById("new-mail-popup-subject");
  const closeIcon = document.getElementById("new-mail-popup-close-icon");
  const maximizeIcon = document.getElementById("new-mail-popup-maximize-icon");
  const minimizeIcon = document.getElementById("new-mail-popup-minimize-icon");
  const toggleCCIcon = document.getElementById("new-mail-popup-toggle-cc");
  const toggleBCCIcon = document.getElementById("new-mail-popup-toggle-bcc");
  const mailBody = document.getElementById("new-mail-popup-body");

  closeIcon.addEventListener("mousedown", function (event) {
    newMailPopup.classList.add("closed");
  });

  toggleCCIcon.addEventListener("mousedown", function (event) {
    const isToggled = CCRow.classList.contains("index-1");

    if (!isToggled) {
      setIndexLevel(subjectRow, "index-2");
      setIndexLevel(CCRow, "index-1");
      setIndexLevel(BCCRow, "index-1");
    } else {
      setIndexLevel(subjectRow, "index-1");
      setIndexLevel(CCRow, "index-0");
      setIndexLevel(BCCRow, "index-0");
    }
  });

  toggleBCCIcon.addEventListener("mousedown", function (event) {
    const isToggled = BCCRow.classList.contains("index-2");

    if (!isToggled) {
      setIndexLevel(subjectRow, "index-3");
      setIndexLevel(BCCRow, "index-2");
    } else {
      setIndexLevel(subjectRow, "index-2");
      setIndexLevel(BCCRow, "index-1");
    }
  });

  minimizeIcon.addEventListener("mousedown", function (event) {
    newMailPopup.classList.add("minimized");
    toInput.classList.add("minimized");
    minimizeIcon.classList.add("hidden");
    maximizeIcon.classList.remove("hidden");
    toggleCCIcon.classList.add("hidden");
    mailBody.classList.add("hidden");

    setIndexLevel(CCRow, "index-0");
    setIndexLevel(BCCRow, "index-0");
    setIndexLevel(subjectRow, "index-0");
  });

  maximizeIcon.addEventListener("mousedown", function (event) {
    newMailPopup.classList.remove("minimized");
    toInput.classList.remove("minimized");
    minimizeIcon.classList.remove("hidden");
    maximizeIcon.classList.add("hidden");
    toggleCCIcon.classList.remove("hidden");
    mailBody.classList.remove("hidden");

    setIndexLevel(subjectRow, "index-1");
  });
}

function setIndexLevel(element, indexLevel) {
  element.classList.remove(...topbarIndexLevels);
  element.classList.add(indexLevel);

  if (element.id == "new-mail-popup-subject") {
    const newMailBody = document.getElementById("new-mail-popup-body");
    setIndexLevel(newMailBody, indexLevel);
  }
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
