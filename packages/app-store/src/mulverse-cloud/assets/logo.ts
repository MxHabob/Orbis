export const Logo = (id = "mulverse-cloud") => `
  <svg width="36" height="36" viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg">
    <defs>
      <linearGradient id="${id}-grad" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#2F80ED" />
        <stop offset="100%" stop-color="#9B51E0" />
      </linearGradient>
    </defs>
    <rect width="36" height="36" rx="8" fill="#0A0F1D" />
    <path d="M18 10C14.5 10 12 12.5 12 15.5C10 16 9 17.5 9 19C9 21 10.5 22.5 12.5 22.5H23.5C25.5 22.5 27 21 27 19C27 17 25.5 15.5 23.5 15.5C23.5 12.5 21 10 18 10Z" fill="url(#${id}-grad)" />
    <circle cx="15" cy="18" r="1.5" fill="#FFFFFF" opacity="0.8" />
    <circle cx="18" cy="16" r="1.5" fill="#FFFFFF" opacity="0.8" />
    <circle cx="21" cy="18" r="1.5" fill="#FFFFFF" opacity="0.8" />
    <path d="M15 18H21" stroke="#FFFFFF" stroke-width="1" opacity="0.4" />
  </svg>
`;
