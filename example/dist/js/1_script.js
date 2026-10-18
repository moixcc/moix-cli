console.log("WWW.MOIX.CC");

function showTemplate() {
  const template = document.querySelector(
    location.hash != "" ? location.hash : "#home",
  );

  document
    .querySelector("main")
    .replaceChildren(template.content.cloneNode(true));
}

window.onhashchange = showTemplate;
showTemplate();
