console.log('MOIX');

function showTemplate() {
    const hash = location.hash != '' ? location.hash.slice(1) : 'home';
    let template = document.getElementById('template/' + hash);

    if (!template) template = document.getElementById('template/not-found');

    document
        .querySelector('main')
        .replaceChildren(template.content.cloneNode(true));
}

window.onhashchange = showTemplate;
showTemplate();