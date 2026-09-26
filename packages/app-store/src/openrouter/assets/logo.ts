export const Logo = (id = "openrouter") => `
  <svg
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
    class="h-full w-auto"
  >
    <defs>
      <linearGradient id="${id}-grad" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#6366F1" />
        <stop offset="100%" stop-color="#8B5CF6" />
      </linearGradient>
    </defs>
    <path
      d="M12 2L2 7.5v9L12 22l10-5.5v-9L12 2zm0 2.8l7.5 4.1-3 1.6-7.5-4.1 3-1.6zm-8 4.7l7 3.8v7.4l-7-3.8V9.5zm9 11.2v-7.4l7-3.8v7.4l-7 3.8z"
      fill="url(#${id}-grad)"
    />
  </svg>
`;
