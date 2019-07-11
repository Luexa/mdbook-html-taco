// Work around some values being stored in localStorage wrapped in quotes
try {
    var theme = localStorage.getItem('mdbook-theme');
    var sidebar = localStorage.getItem('mdbook-sidebar');

    if (theme.startsWith('"') && theme.endsWith('"')) {
        localStorage.setItem('mdbook-theme', theme.slice(1, theme.length - 1));
    }

    if (sidebar.startsWith('"') && sidebar.endsWith('"')) {
        localStorage.setItem('mdbook-sidebar', sidebar.slice(1, sidebar.length - 1));
    }
} catch (e) { }

// Set the theme before any content is loaded, prevents flash
var theme;
try { theme = localStorage.getItem('mdbook-theme'); } catch(e) { }
if (theme === null || theme === undefined) { theme = default_theme; }
document.body.className = theme;
document.querySelector('html').className = theme + ' js';

// Hide / unhide sidebar before it is displayed
var html = document.querySelector('html');
var sidebar = 'hidden';
if (document.body.clientWidth >= 1080) {
    try { sidebar = localStorage.getItem('mdbook-sidebar'); } catch(e) { }
    sidebar = sidebar || 'visible';
}
html.classList.remove('sidebar-visible');
html.classList.add("sidebar-" + sidebar);
