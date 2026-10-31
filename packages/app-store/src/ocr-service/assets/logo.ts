export const Logo = () => `
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" class="h-full w-auto">
    <defs>
      <linearGradient id="ocrBg" x1="0" y1="0" x2="64" y2="64" gradientUnits="userSpaceOnUse">
        <stop stop-color="#6E6CE0"/>
        <stop offset="1" stop-color="#5856D6"/>
      </linearGradient>
      <linearGradient id="ocrBeam" x1="12" y1="0" x2="52" y2="0" gradientUnits="userSpaceOnUse">
        <stop stop-color="#FFB340"/>
        <stop offset="0.5" stop-color="#FF9500"/>
        <stop offset="1" stop-color="#FFB340"/>
      </linearGradient>
    </defs>
    <rect width="64" height="64" rx="14" fill="url(#ocrBg)"/>
    <rect x="18" y="11" width="28" height="42" rx="4" fill="#FFFFFF"/>
    <rect x="23" y="18" width="18" height="3.2" rx="1.6" fill="#CFCDF3"/>
    <rect x="23" y="25" width="13" height="3.2" rx="1.6" fill="#CFCDF3"/>
    <rect x="23" y="43" width="18" height="3.2" rx="1.6" fill="#CFCDF3"/>
    <rect x="23" y="48.5" width="13" height="3.2" rx="1.6" fill="#CFCDF3"/>
    <path d="M28.6 33.4L32 25.8L35.4 33.4M29.9 30.9H34.1"
          fill="none" stroke="#5856D6" stroke-width="2.6"
          stroke-linecap="round" stroke-linejoin="round"/>
    <rect x="12" y="40" width="40" height="5" fill="#FF9500" opacity="0.15"/>
    <rect x="12" y="37.2" width="40" height="2.8" rx="1.4" fill="url(#ocrBeam)"/>
    <path d="M14 35.5V41.7M50 35.5V41.7"
          stroke="#FFB340" stroke-width="2.4" stroke-linecap="round"/>
  </svg>
`;
