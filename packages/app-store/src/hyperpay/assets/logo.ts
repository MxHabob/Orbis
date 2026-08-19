export const Logo = (id = "hyperpay") => `
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 32 32"
    fill="none"
    class="h-full w-auto"
  >
    <defs>
      <linearGradient id="${id}-grad" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#00D2FF" />
        <stop offset="100%" stop-color="#0066FF" />
      </linearGradient>
    </defs>
    <g transform="translate(2, 4)">
      <!-- HyperPay forward dual chevron symbol -->
      <path
        d="M4 24L11 4H16L9 24H4Z"
        fill="url(#${id}-grad)"
      />
      <path
        d="M14 24L21 4H26L19 24H14Z"
        fill="#0066FF"
      />
    </g>
  </svg>
`;
