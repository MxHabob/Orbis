import type { AvatarState } from "./avatar-state-machine.js";
import type { AvatarHat } from "./types.js";

export type AvatarSvgTag = "circle" | "ellipse" | "line" | "path" | "rect";
export type AvatarSvgAttributes = Record<string, string | number>;

export interface AvatarSvgElement {
  key: string;
  tag: AvatarSvgTag;
  attributes: AvatarSvgAttributes;
}

export interface AvatarSvgOptions {
  hat?: AvatarHat;
  state?: AvatarState;
  faceColor?: string;
  featureColor?: string;
}

type StaticAvatarHat = Exclude<AvatarHat, "none" | "random">;

function shape(
  key: string,
  tag: AvatarSvgTag,
  attributes: AvatarSvgAttributes,
): AvatarSvgElement {
  return { key, tag, attributes };
}

function path(
  key: string,
  d: string,
  attributes: AvatarSvgAttributes = {},
): AvatarSvgElement {
  return shape(key, "path", { d, ...attributes });
}

function circle(
  key: string,
  cx: number,
  cy: number,
  r: number,
  fill: string,
  attributes: AvatarSvgAttributes = {},
): AvatarSvgElement {
  return shape(key, "circle", { cx, cy, r, fill, ...attributes });
}

const HATS: Record<StaticAvatarHat, readonly AvatarSvgElement[]> = {
  crown: [
    path(
      "crown-body",
      "M64 68 53 37l25 14 22-28 22 28 25-14-11 31q-36 8-72 0Z",
      {
        fill: "#f5a623",
        stroke: "#9a5b13",
        "stroke-width": 3,
        "stroke-linejoin": "round",
      },
    ),
    path("crown-rim", "M67 67q33 7 66 0", {
      fill: "none",
      stroke: "#7c4610",
      "stroke-width": 5,
    }),
    circle("crown-jewel-left", 53, 37, 4, "#ea5555"),
    circle("crown-jewel-center", 100, 23, 4, "#5598e8"),
    circle("crown-jewel-right", 147, 37, 4, "#4ca97b"),
  ],
  graduation: [
    path("graduation-cap", "M72 53 100 35l28 18v17q-28 14-56 0Z", {
      fill: "#26364b",
    }),
    path("graduation-board", "m49 45 51-14 51 14-51 14Z", {
      fill: "#182638",
      stroke: "#344a63",
      "stroke-width": 3,
      "stroke-linejoin": "round",
    }),
    shape("graduation-tassel", "line", {
      x1: 145,
      y1: 47,
      x2: 145,
      y2: 71,
      stroke: "#f1be4c",
      "stroke-width": 3,
    }),
    circle("graduation-tassel-end", 145, 73, 4, "#f1be4c"),
  ],
  firefighter: [
    path(
      "firefighter-shell",
      "M51 66q4-29 23-34 5-20 26-20t26 20q19 5 23 34-49 15-98 0Z",
      {
        fill: "#e94b45",
        stroke: "#9b302f",
        "stroke-width": 4,
      },
    ),
    path("firefighter-brim", "M50 67q50-12 100 0-50 15-100 0Z", {
      fill: "#bc3334",
    }),
    shape("firefighter-badge", "rect", {
      x: 91,
      y: 44,
      width: 18,
      height: 17,
      fill: "#f3ca54",
    }),
  ],
  wizard: [
    path("wizard-brim", "M48 70q52-15 104 0-52 16-104 0Z", {
      fill: "#5046a8",
      stroke: "#342d79",
      "stroke-width": 3,
    }),
    path("wizard-cone", "m65 67 35-52 37 53q-36-9-72-1Z", {
      fill: "#6255bf",
      stroke: "#342d79",
      "stroke-width": 3,
      "stroke-linejoin": "round",
    }),
    path("wizard-star", "m96 38 4-8 4 8 9 1-7 6 2 9-8-5-8 5 2-9-7-6Z", {
      fill: "#f6d35e",
    }),
  ],
  detective: [
    path("detective-crown", "M59 66q2-28 41-29 39 1 41 29-41 12-82 0Z", {
      fill: "#99652e",
      stroke: "#68431f",
      "stroke-width": 3,
    }),
    path("detective-brim", "M49 68q51-13 102 0-51 13-102 0Z", {
      fill: "#ad7839",
      stroke: "#68431f",
      "stroke-width": 3,
    }),
    path("detective-ribbon", "M92 43q8-9 16 0l-8 6Z", { fill: "#f1e3c5" }),
  ],
  chef: [
    path(
      "chef-puff",
      "M72 57q-17-15-8-29 7-12 23-6 13-19 27 0 17-6 23 7 7 14-9 28Z",
      {
        fill: "#fbfaf6",
        stroke: "#b9c1c9",
        "stroke-width": 3,
      },
    ),
    path("chef-band", "M74 56h52v13q-26 10-52 0Z", {
      fill: "#e6e9ed",
      stroke: "#b9c1c9",
      "stroke-width": 3,
    }),
    path("chef-folds", "M88 25v27m12-31v34m12-30v27", {
      fill: "none",
      stroke: "#e1e5e9",
      "stroke-width": 2,
    }),
  ],
  astronaut: [
    path("astronaut-shell", "M63 68V47q0-35 37-35t37 35v21q-37 11-74 0Z", {
      fill: "#e7edf3",
      stroke: "#aab8c6",
      "stroke-width": 4,
    }),
    path("astronaut-visor", "M72 45q0-20 28-20t28 20q0 18-28 18T72 45Z", {
      fill: "#3594bd",
      stroke: "#176480",
      "stroke-width": 3,
    }),
    path("astronaut-reflection", "M79 39q13-10 31-7", {
      fill: "none",
      stroke: "#a8e5f7",
      "stroke-width": 3,
      "stroke-linecap": "round",
    }),
  ],
  headset: [
    path("headset-band", "M51 102a49 49 0 0 1 98 0", {
      fill: "none",
      stroke: "#34465d",
      "stroke-width": 7,
    }),
    shape("headset-ear-left", "rect", {
      x: 42,
      y: 92,
      width: 20,
      height: 31,
      rx: 8,
      fill: "#4b8fe2",
    }),
    shape("headset-ear-right", "rect", {
      x: 138,
      y: 92,
      width: 20,
      height: 31,
      rx: 8,
      fill: "#4b8fe2",
    }),
    path("headset-microphone", "M50 115q8 23 31 20", {
      fill: "none",
      stroke: "#34465d",
      "stroke-width": 4,
      "stroke-linecap": "round",
    }),
    circle("headset-microphone-tip", 82, 135, 4, "#e95056"),
  ],
  pilot: [
    path("pilot-crown", "M66 65q3-34 34-34t34 34q-34 12-68 0Z", {
      fill: "#243b67",
      stroke: "#172747",
      "stroke-width": 3,
    }),
    path("pilot-band", "M67 56q33-10 66 0v8q-33 12-66 0Z", { fill: "#edf1f5" }),
    path("pilot-brim", "M50 68q50-15 100 0-50 14-100 0Z", {
      fill: "#172747",
      stroke: "#101b30",
      "stroke-width": 3,
    }),
    path("pilot-badge", "m91 43 5 5 4-6 5 6 5-5-3 10H94Z", { fill: "#f2bf4e" }),
  ],
  party: [
    path("party-cone", "m66 70 34-55 34 55q-34 14-68 0Z", {
      fill: "#e95d9a",
      stroke: "#aa3670",
      "stroke-width": 3,
    }),
    path("party-stripes", "M78 51q22-8 44 0M87 35q13-5 26 0", {
      fill: "none",
      stroke: "#f5c454",
      "stroke-width": 4,
    }),
    circle("party-pom", 100, 14, 6, "#f5c454"),
  ],
  pirate: [
    path("pirate-brim", "M52 67q48-25 96 0-48 16-96 0Z", {
      fill: "#252b35",
      stroke: "#111820",
      "stroke-width": 3,
    }),
    path("pirate-crown", "M65 62q2-25 35-27 33 2 35 27Z", { fill: "#343c47" }),
    path("pirate-trim", "M54 66q46-20 92 0", {
      fill: "none",
      stroke: "#e5b84b",
      "stroke-width": 3,
    }),
    circle("pirate-skull", 100, 52, 6, "#f7f3e8"),
    circle("pirate-eye-left", 98, 52, 1.5, "#202733"),
    circle("pirate-eye-right", 102, 52, 1.5, "#202733"),
  ],
  ninja: [
    path("ninja-band", "M63 62q37-9 74 0l-3 14q-34-9-68 0Z", {
      fill: "#202733",
      stroke: "#111820",
      "stroke-width": 3,
    }),
    shape("ninja-plate", "rect", {
      x: 86,
      y: 63,
      width: 28,
      height: 12,
      rx: 3,
      fill: "#8293a6",
    }),
    path("ninja-ribbon", "M136 67q12 8 20 0l-2 8q-9 7-20 0Z", {
      fill: "#202733",
    }),
  ],
  tophat: [
    path("tophat-brim", "M52 69q48-14 96 0-48 15-96 0Z", {
      fill: "#252b35",
      stroke: "#111820",
      "stroke-width": 3,
    }),
    path("tophat-body", "M71 66 69 26h62l-2 40Z", {
      fill: "#303844",
      stroke: "#171c23",
      "stroke-width": 3,
    }),
    path("tophat-ribbon", "M70 55h60l-1 9H71Z", { fill: "#c9484f" }),
  ],
  viking: [
    path(
      "viking-horns",
      "M68 61Q49 48 42 25q19 9 32 32m58 4q19-13 26-36-19 9-32 32",
      {
        fill: "#f2f4f5",
        stroke: "#b9c3cc",
        "stroke-width": 4,
        "stroke-linejoin": "round",
      },
    ),
    path("viking-helmet", "M65 65q2-30 35-30t35 30q-35 13-70 0Z", {
      fill: "#78899a",
      stroke: "#455669",
      "stroke-width": 3,
    }),
    path("viking-ridge", "M100 39v29", {
      fill: "none",
      stroke: "#536578",
      "stroke-width": 4,
    }),
  ],
  propeller: [
    path("propeller-cap", "M66 67q0-29 34-29t34 29q-34 12-68 0Z", {
      fill: "#5889d6",
    }),
    shape("propeller-stem", "line", {
      x1: 100,
      y1: 39,
      x2: 100,
      y2: 28,
      stroke: "#34465d",
      "stroke-width": 3,
    }),
    shape("propeller-left", "ellipse", {
      cx: 88,
      cy: 27,
      rx: 12,
      ry: 4,
      fill: "#f4f4ef",
      stroke: "#aab4bf",
      "stroke-width": 2,
    }),
    shape("propeller-right", "ellipse", {
      cx: 112,
      cy: 27,
      rx: 12,
      ry: 4,
      fill: "#f4f4ef",
      stroke: "#aab4bf",
      "stroke-width": 2,
    }),
    circle("propeller-hub", 100, 27, 4, "#e65859"),
    path("propeller-brim", "M64 65q36-10 72 0", {
      fill: "none",
      stroke: "#efc04f",
      "stroke-width": 6,
    }),
  ],
  santa: [
    path("santa-hat", "M64 66q28-30 66-2 17-20 28-9 4 11-24 18-35 5-70-7Z", {
      fill: "#dc454b",
      stroke: "#9e3039",
      "stroke-width": 3,
    }),
    path("santa-brim", "M59 65q39-12 82 0-8 13-40 10-34 3-42-10Z", {
      fill: "#faf7f1",
      stroke: "#c7cdd2",
      "stroke-width": 3,
    }),
    circle("santa-pom", 157, 55, 8, "#faf7f1", {
      stroke: "#c7cdd2",
      "stroke-width": 3,
    }),
  ],
};

export function getAvatarHatDrawing(
  hat: AvatarHat = "none",
): AvatarSvgElement[] {
  if (hat === "none" || hat === "random") return [];
  return HATS[hat].map((element) => ({
    ...element,
    key: `hat-${element.key}`,
    attributes: {
      ...element.attributes,
      transform: element.attributes.transform
        ? `${element.attributes.transform} translate(0 3)`
        : "translate(0 3)",
    },
  }));
}

export function getAvatarDrawing({
  hat = "none",
  state = "idle",
  faceColor = "var(--muted, #f1f5f9)",
  featureColor = "var(--foreground, #18181b)",
}: AvatarSvgOptions = {}): AvatarSvgElement[] {
  const browPath =
    state === "thinking"
      ? "M68 99q9-14 29-3m8-1q16-12 32-1"
      : state === "error"
        ? "M68 92q14 8 29 7m8 0q16 1 32-7"
        : "M68 100q9-14 34 0m0 0q15-14 40 0";
  const eyeElements: AvatarSvgElement[] =
    state === "blinking"
      ? [
          path("eye-left", "M78 115q8 7 16 0", {
            fill: "none",
            stroke: featureColor,
            "stroke-width": 4.5,
            "stroke-linecap": "round",
          }),
          path("eye-right", "M112 116q8 7 16 0", {
            fill: "none",
            stroke: featureColor,
            "stroke-width": 4.5,
            "stroke-linecap": "round",
          }),
        ]
      : state === "happy"
        ? [
            path("eye-left", "M78 117q8-9 16 0", {
              fill: "none",
              stroke: featureColor,
              "stroke-width": 4.5,
              "stroke-linecap": "round",
            }),
            path("eye-right", "M112 118q8-9 16 0", {
              fill: "none",
              stroke: featureColor,
              "stroke-width": 4.5,
              "stroke-linecap": "round",
            }),
          ]
        : [
            circle(
              "eye-left",
              state === "thinking" ? 88 : 86,
              114,
              5.5,
              featureColor,
            ),
            circle(
              "eye-right",
              state === "thinking" ? 122 : 120,
              117,
              5.5,
              featureColor,
            ),
          ];
  const mouth =
    state === "happy"
      ? path("mouth", "M88 143q12 12 25 0", {
          fill: "none",
          stroke: featureColor,
          "stroke-width": 4,
          "stroke-linecap": "round",
        })
      : state === "error"
        ? path("mouth", "m91 148 8-7 8 7", {
            fill: "none",
            stroke: featureColor,
            "stroke-width": 4,
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
          })
        : null;

  return [
    circle("head", 100, 120, 58, faceColor),
    path("eyebrows", browPath, {
      fill: "none",
      stroke: featureColor,
      "stroke-width": 4.5,
      "stroke-linecap": "round",
    }),
    path("nose", "M102 100 75 146l29 1", {
      fill: "none",
      stroke: featureColor,
      "stroke-width": 4.5,
      "stroke-linecap": "round",
      "stroke-linejoin": "round",
    }),
    ...eyeElements,
    ...(mouth ? [mouth] : []),
    ...getAvatarHatDrawing(hat),
  ];
}

function escapeAttribute(value: string | number): string {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll('"', "&quot;")
    .replaceAll("<", "&lt;");
}

export function renderAvatarSvg(options: AvatarSvgOptions = {}): string {
  const nodes = getAvatarDrawing(options)
    .map(({ tag, attributes }) => {
      const serialized = Object.entries(attributes)
        .map(([name, value]) => `${name}="${escapeAttribute(value)}"`)
        .join(" ");
      return `<${tag} ${serialized}/>`;
    })
    .join("");
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="100%" height="100%">${nodes}</svg>`;
}
