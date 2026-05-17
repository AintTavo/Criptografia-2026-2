// ── Guard: si no hay JWT redirigir a login ─────────────────────────────
(function() {
  const jwt = sessionStorage.getItem('jwt');
  if (!jwt) { window.location.href = '../index.html'; }
})();

// ── Header con blur al scroll ──────────────────────────────────────────
const header = document.getElementById('site-header');
window.addEventListener('scroll', () => {
  header.classList.toggle('scrolled', window.scrollY > 40);
});

// ── Logout ─────────────────────────────────────────────────────────────
function doLogout() {
  sessionStorage.removeItem('jwt');
  window.location.href = '../index.html';
}