export const Logo = (id = "hyperpay") => `
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="36"
    height="36"
    viewBox="0 0 48 48"
    fill="none"
    class="w-9 h-9"
  >
    <defs>
      <linearGradient id="${id}-grad" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#00D2FF" />
        <stop offset="100%" stop-color="#0066FF" />
      </linearGradient>
    </defs>
    <rect width="48" height="48" rx="10" fill="#0B132B" />
    <g transform="translate(10, 10)">
      <!-- HyperPay forward dual chevron symbol -->
      <path
        d="M6 24L13 4H18L11 24H6Z"
        fill="url(#${id}-grad)"
      />
      <path
        d="M16 24L23 4H28L21 24H16Z"
        fill="#FFFFFF"
      />
    </g>
  </svg>
`;
