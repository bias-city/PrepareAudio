'use strict';

/* Handbuch: Kapitel links, Inhalt rechts, Suche oben. Der Inhalt steht je Sprache in
   hilfe/<sprache>.js (dieselben Kapitel-ids und Bilddateien, de ist die Quelle).
   Inline-Auszeichnung im Text: **Beschriftung der Oberfläche**, [[Taste]], `Pfad`.
   Bilder: hilfe/<sprache>/<hell|dunkel>/<datei>.jpg, erzeugt von scripts/screenshots.mjs --hilfe. */
(() => {
  const tr = (k, p) => I18N.t(k, p);
  const q = (sel) => document.querySelector(sel);
  const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
  const STORE = 'prepareaudio.hilfe.kapitel';
  const kapitel = () => (window.HILFE && window.HILFE[I18N.lang()]) || window.HILFE.de;
  const dunkel = () => window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
  let aktiv = 0;
  let suche = '';

  /** **fett**, [[Taste]] und `code` in Auszeichnung; alles andere bleibt Text. */
  function markup(text) {
    return esc(text)
      .replace(/\*\*([^*]+)\*\*/g, '<b>$1</b>')
      .replace(/\[\[([^\]]+)\]\]/g, '<kbd>$1</kbd>')
      .replace(/`([^`]+)`/g, '<code>$1</code>');
  }

  /** Die Suchbegriffe im fertigen Auszeichnungstext hervorheben (nie in Tags hinein). */
  function markieren(html) {
    if (!suche) return html;
    const teile = suche.split(/\s+/).filter((w) => w.length > 1).map((w) => w.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'));
    if (!teile.length) return html;
    const re = new RegExp(`(${teile.join('|')})`, 'gi');
    return html.split(/(<[^>]*>)/).map((s) => (s.startsWith('<') ? s : s.replace(re, '<mark>$1</mark>'))).join('');
  }

  const norm = (s) => String(s).toLowerCase().normalize('NFD').replace(/[̀-ͯ]/g, '');

  /** Aller Text eines Blocks, für die Suche. */
  function blockText(b) {
    return [b.text, ...(b.punkte || []), ...(b.zeilen || []).flat(), ...(b.kopf || [])].filter(Boolean).join(' ');
  }
  const passt = (k) => !suche || norm([k.titel, k.kurz, ...k.bloecke.map(blockText)].join(' ')).includes(norm(suche));

  function blockHtml(b) {
    switch (b.art) {
      case 'h': return `<h2>${markieren(markup(b.text))}</h2>`;
      case 'p': return `<p>${markieren(markup(b.text))}</p>`;
      case 'hinweis': return `<p class="hilfe-hinweis">${markieren(markup(b.text))}</p>`;
      case 'liste': return `<ul>${b.punkte.map((x) => `<li>${markieren(markup(x))}</li>`).join('')}</ul>`;
      case 'schritte': return `<ol>${b.punkte.map((x) => `<li>${markieren(markup(x))}</li>`).join('')}</ol>`;
      case 'tasten': return `<table class="hilfe-tasten"><tbody>${b.zeilen.map(([t, w]) =>
        `<tr><td>${markup(t)}</td><td>${markieren(markup(w))}</td></tr>`).join('')}</tbody></table>`;
      case 'tabelle': return `<table class="hilfe-tabelle"><thead><tr>${b.kopf.map((x) => `<th>${markieren(markup(x))}</th>`).join('')}</tr></thead>`
        + `<tbody>${b.zeilen.map((z) => `<tr>${z.map((x) => `<td>${markieren(markup(x))}</td>`).join('')}</tr>`).join('')}</tbody></table>`;
      case 'bild': return `<figure><img src="hilfe/${I18N.lang()}/${dunkel() ? 'dunkel' : 'hell'}/${b.datei}.jpg" alt="${esc(b.text)}" loading="lazy">`
        + `<figcaption>${markieren(markup(b.text))}</figcaption></figure>`;
      default: return '';
    }
  }

  function render() {
    const alle = kapitel();
    const sichtbar = alle.filter(passt);
    if (!sichtbar.some((k) => k === alle[aktiv])) aktiv = alle.indexOf(sichtbar[0]) >= 0 ? alle.indexOf(sichtbar[0]) : aktiv;
    q('#hilfe-nav').innerHTML = alle.map((k, i) =>
      `<button data-kapitel="${i}" class="${i === aktiv ? 'on' : ''}" ${passt(k) ? '' : 'disabled'}>
         <span class="hilfe-nav-titel">${esc(k.titel)}</span><span class="hilfe-nav-kurz">${esc(k.kurz)}</span></button>`).join('');
    const k = alle[aktiv];
    q('#hilfe-inhalt').innerHTML = k ? `<h1>${esc(k.titel)}</h1>${k.bloecke.map(blockHtml).join('')}` : '';
    q('#hilfe-inhalt').scrollTop = 0;
    q('#hilfe-treffer').textContent = suche ? I18N.tn('hilfe.treffer', sichtbar.length) : '';
    q('#hilfe-lang').innerHTML = I18N.LANGS.map((l) =>
      `<button data-lang="${l}" lang="${l}" aria-pressed="${l === I18N.lang()}">${esc(I18N.NAMES[l])}</button>`).join('');
    document.title = tr('hilfe.titel');
  }

  q('#hilfe-nav').addEventListener('click', (e) => {
    const b = e.target.closest('[data-kapitel]');
    if (!b || b.disabled) return;
    aktiv = Number(b.dataset.kapitel);
    try { localStorage.setItem(STORE, kapitel()[aktiv].id); } catch (err) { /* storage unavailable */ }
    render();
  });
  q('#hilfe-lang').addEventListener('click', (e) => {
    const b = e.target.closest('[data-lang]');
    if (b && b.dataset.lang !== I18N.lang()) I18N.setLang(b.dataset.lang);
  });
  q('#hilfe-suche').addEventListener('input', (e) => { suche = e.target.value.trim(); render(); });
  document.addEventListener('keydown', (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'f') { e.preventDefault(); q('#hilfe-suche').focus(); q('#hilfe-suche').select(); }
    if (e.key === 'Escape' && suche) { suche = ''; q('#hilfe-suche').value = ''; render(); }
  });
  if (window.matchMedia) window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', render);
  I18N.onChange(() => {
    const id = kapitel()[aktiv] && kapitel()[aktiv].id;
    const nach = kapitel().findIndex((k) => k.id === id);
    aktiv = nach >= 0 ? nach : 0;
    I18N.apply();
    render();
  });

  // Beim Öffnen das zuletzt gelesene Kapitel, sonst das erste.
  try {
    const id = localStorage.getItem(STORE);
    const i = kapitel().findIndex((k) => k.id === id);
    if (i >= 0) aktiv = i;
  } catch (err) { /* storage unavailable */ }
  I18N.apply();
  render();
})();
