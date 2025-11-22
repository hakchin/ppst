document.addEventListener('DOMContentLoaded', () => {
    const navToggle = document.querySelector('.nav-toggle');
    const nav = document.querySelector('#main-nav');

    if (navToggle && nav) {
        // Toggle navigation menu
        navToggle.addEventListener('click', () => {
            const isExpanded = navToggle.getAttribute('aria-expanded') === 'true';
            navToggle.setAttribute('aria-expanded', !isExpanded);
            nav.classList.toggle('nav--open');
        });

        // Close menu when clicking on navigation links (mobile only)
        const navLinks = nav.querySelectorAll('.nav__link');
        navLinks.forEach(link => {
            link.addEventListener('click', () => {
                // Only close if menu is currently open (mobile view)
                if (nav.classList.contains('nav--open')) {
                    nav.classList.remove('nav--open');
                    navToggle.setAttribute('aria-expanded', 'false');
                }
            });
        });
    }
});
