document.getElementById('sidebar-toggle').setAttribute('aria-expanded', sidebar === 'visible');
document.getElementById('sidebar').setAttribute('aria-hidden', sidebar !== 'visible');
Array.from(document.querySelectorAll('#sidebar a')).forEach(function(link) {
    link.setAttribute('tabIndex', sidebar === 'visible' ? 0 : -1);
});
