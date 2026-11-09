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
  flower: [
    path("flower-band", "M59 63q41-12 82 0l-3 10q-38 12-76 0Z", {
      fill: "#55a875",
      stroke: "#34744e",
      "stroke-width": 3,
    }),
    circle("flower-petal-top", 100, 19, 9, "#f27da2"),
    circle("flower-petal-right", 115, 34, 9, "#f27da2"),
    circle("flower-petal-bottom", 100, 49, 9, "#f27da2"),
    circle("flower-petal-left", 85, 34, 9, "#f27da2"),
    circle("flower-center", 100, 34, 10, "#f5b941"),
    path("flower-leaf", "M121 61q14-18 25-12-5 17-23 19Z", {
      fill: "#8bcf78",
      stroke: "#4b9656",
      "stroke-width": 2,
    }),
  ],
  duck: [
    path("duck-cap", "M61 61q5-21 34-23 35-2 48 24-41 13-82-1Z", {
      fill: "#f2c94c",
      stroke: "#b88120",
      "stroke-width": 3,
    }),
    path("duck-bill", "M117 37q21-8 29 1-7 12-25 9Z", {
      fill: "#f08c3e",
      stroke: "#b85a28",
      "stroke-width": 2,
    }),
    circle("duck-eye", 112, 31, 2.5, "#25212b"),
    path("duck-wing", "M88 46q13-12 24 1-11 10-24-1Z", {
      fill: "#ffe27a",
      stroke: "#d5a93a",
      "stroke-width": 2,
    }),
    path("duck-brim", "M53 63q47-13 94 0-47 15-94 0Z", {
      fill: "#f4b52f",
      stroke: "#b88120",
      "stroke-width": 3,
    }),
  ],
  cat: [
    path(
      "cat-hood",
      "M61 66V42l-4-25 25 15q18-9 36 0l25-15-4 25v24q-39 15-78 0Z",
      {
        fill: "#e7e4f2",
        stroke: "#77718d",
        "stroke-width": 3,
        "stroke-linejoin": "round",
      },
    ),
    path("cat-ear-inner-left", "m66 39-2-13 13 8Z", { fill: "#ec8ba4" }),
    path("cat-ear-inner-right", "m123 34 13-8-2 13Z", { fill: "#ec8ba4" }),
    path("cat-face", "M78 48q22 15 44 0", {
      fill: "none",
      stroke: "#625d73",
      "stroke-width": 2.5,
      "stroke-linecap": "round",
    }),
    circle("cat-eye-left", 86, 45, 2, "#625d73"),
    circle("cat-eye-right", 114, 45, 2, "#625d73"),
    path("cat-muzzle", "m97 51 3 3 3-3", {
      fill: "none",
      stroke: "#d96d83",
      "stroke-width": 2,
      "stroke-linecap": "round",
      "stroke-linejoin": "round",
    }),
  ],
  cowboy: [
    path("cowboy-crown", "M72 61q-4-25 8-34 20-13 40 0 12 9 8 34Z", {
      fill: "#c98248",
      stroke: "#80502f",
      "stroke-width": 3,
    }),
    path("cowboy-crease", "M91 32q9 8 18 0", {
      fill: "none",
      stroke: "#9b6039",
      "stroke-width": 3,
      "stroke-linecap": "round",
    }),
    path("cowboy-band", "M72 49q28 8 56 0l1 9q-29 8-58 0Z", {
      fill: "#e3ae63",
      stroke: "#9b6039",
      "stroke-width": 2,
    }),
    path("cowboy-brim", "M49 63q51-20 102 0-13 17-51 17T49 63Z", {
      fill: "#b8733e",
      stroke: "#80502f",
      "stroke-width": 3,
    }),
    path("cowboy-star", "m100 48 3 6 7 1-5 4 1 7-6-3-6 3 1-7-5-4 7-1Z", {
      fill: "#f5ce61",
    }),
  ],
  beanie: [
    path("beanie-body", "M61 62q0-36 39-40 39 4 39 40-39 14-78 0Z", {
      fill: "#4d91dc",
      stroke: "#2f5f9e",
      "stroke-width": 3,
    }),
    path("beanie-stripe", "M64 45q36 12 72 0v11q-36 12-72 0Z", {
      fill: "#f3ca54",
      stroke: "#b98432",
      "stroke-width": 2,
    }),
    path("beanie-ribbing", "M76 58v10m12-8v11m12-9v11m12-12v10m12-14v10", {
      fill: "none",
      stroke: "#2f5f9e",
      "stroke-width": 2,
    }),
    circle("beanie-pom", 100, 16, 9, "#f27da2"),
  ],
  rainbow: [
    path("rainbow-outer", "M60 66a40 40 0 0 1 80 0", {
      fill: "none",
      stroke: "#ed6a63",
      "stroke-width": 12,
      "stroke-linecap": "round",
    }),
    path("rainbow-middle", "M72 66a28 28 0 0 1 56 0", {
      fill: "none",
      stroke: "#f2c94c",
      "stroke-width": 9,
      "stroke-linecap": "round",
    }),
    path("rainbow-inner", "M84 66a16 16 0 0 1 32 0", {
      fill: "none",
      stroke: "#6dbb8b",
      "stroke-width": 7,
      "stroke-linecap": "round",
    }),
    circle("rainbow-cloud-left", 58, 65, 12, "#f7f4ef", {
      stroke: "#b9c1c9",
      "stroke-width": 2,
    }),
    circle("rainbow-cloud-right", 142, 65, 12, "#f7f4ef", {
      stroke: "#b9c1c9",
      "stroke-width": 2,
    }),
  ],
  apple: [
    path(
      "apple-body",
      "M100 31q-11-12-23-3-18 13-8 34 10 26 31 16 21 10 31-16 10-21-8-34-12-9-23 3Z",
      {
        fill: "#e95056",
        stroke: "#a8313c",
        "stroke-width": 3,
      },
    ),
    path("apple-leaf", "M101 27q2-20 23-19-3 18-23 19Z", {
      fill: "#70bd73",
      stroke: "#43834d",
      "stroke-width": 2,
    }),
    path("apple-stem", "M99 28q-2-9-8-12", {
      fill: "none",
      stroke: "#80502f",
      "stroke-width": 4,
      "stroke-linecap": "round",
    }),
    path("apple-shine", "M80 39q-8 10-4 19", {
      fill: "none",
      stroke: "#ffb5a4",
      "stroke-width": 3,
      "stroke-linecap": "round",
    }),
    path("apple-band", "M60 67q40 10 80 0v7q-40 12-80 0Z", {
      fill: "#e95056",
      stroke: "#a8313c",
      "stroke-width": 2,
    }),
  ],
  leaf: [
    path("leaf-wreath-base", "M62 65q38-19 76 0", {
      fill: "none",
      stroke: "#4f9c68",
      "stroke-width": 9,
      "stroke-linecap": "round",
    }),
    path("leaf-left", "M68 58q-20-22-29-8 6 18 30 16Z", {
      fill: "#77c77c",
      stroke: "#3d8454",
      "stroke-width": 2.5,
    }),
    path("leaf-mid-left", "M88 51Q69 28 62 39q3 19 27 19Z", {
      fill: "#a0d978",
      stroke: "#4b9656",
      "stroke-width": 2.5,
    }),
    path("leaf-center", "M100 51Q87 24 100 19q13 5 0 32Z", {
      fill: "#73bf70",
      stroke: "#43834d",
      "stroke-width": 2.5,
    }),
    path("leaf-mid-right", "M112 51q19-23 26-12-3 19-27 19Z", {
      fill: "#a0d978",
      stroke: "#4b9656",
      "stroke-width": 2.5,
    }),
    path("leaf-right", "M132 58q20-22 29-8-6 18-30 16Z", {
      fill: "#77c77c",
      stroke: "#3d8454",
      "stroke-width": 2.5,
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
      ...(element.attributes.stroke === undefined ? { stroke: "#25212b" } : {}),
      ...(element.attributes["stroke-width"] === undefined
        ? { "stroke-width": 2.5 }
        : {}),
      ...(element.attributes["stroke-linecap"] === undefined
        ? { "stroke-linecap": "round" }
        : {}),
      ...(element.attributes["stroke-linejoin"] === undefined
        ? { "stroke-linejoin": "round" }
        : {}),
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
        : state === "curious"
          ? "M68 94q12-18 31-2m8 5q16-10 32 2"
          : state === "sleepy"
            ? "M68 107q15-10 31 1m8 0q16-11 32-1"
            : "M68 100q9-14 34 0m0 0q15-14 40 0";
  let eyeElements: AvatarSvgElement[];

  if (state === "blinking" || state === "happy" || state === "celebrating") {
    const eyePath =
      state === "celebrating" ? "M78 117q8-11 16 0" : "M78 115q8 7 16 0";
    eyeElements = [
      path("eye-left", eyePath, {
        fill: "none",
        stroke: featureColor,
        "stroke-width": 4.5,
        "stroke-linecap": "round",
      }),
      path(
        "eye-right",
        state === "celebrating" ? "M112 117q8-11 16 0" : "M112 116q8 7 16 0",
        {
          fill: "none",
          stroke: featureColor,
          "stroke-width": 4.5,
          "stroke-linecap": "round",
        },
      ),
    ];
  } else if (state === "sleepy") {
    eyeElements = [
      path("eye-left", "M77 116q9 7 18 0", {
        fill: "none",
        stroke: featureColor,
        "stroke-width": 4.5,
        "stroke-linecap": "round",
      }),
      path("eye-right", "M111 118q9 7 18 0", {
        fill: "none",
        stroke: featureColor,
        "stroke-width": 4.5,
        "stroke-linecap": "round",
      }),
    ];
  } else {
    eyeElements = [
      circle(
        "eye-left",
        state === "thinking" || state === "curious" ? 88 : 86,
        114,
        state === "curious" ? 6.5 : 5.5,
        featureColor,
      ),
      circle(
        "eye-right",
        state === "thinking" || state === "curious" ? 122 : 120,
        117,
        state === "curious" ? 4 : 5.5,
        featureColor,
      ),
    ];
  }

  const mouth: AvatarSvgElement[] =
    state === "happy"
      ? [
          path("mouth", "M88 143q12 12 25 0", {
            fill: "none",
            stroke: featureColor,
            "stroke-width": 4,
            "stroke-linecap": "round",
          }),
        ]
      : state === "celebrating"
        ? [
            shape("mouth", "ellipse", {
              cx: 100,
              cy: 145,
              rx: 10,
              ry: 12,
              fill: featureColor,
            }),
            path("mouth-tongue", "M94 151q6-5 12 0", {
              fill: "none",
              stroke: "#ef8f9b",
              "stroke-width": 3,
              "stroke-linecap": "round",
            }),
          ]
        : state === "curious"
          ? [
              shape("mouth", "ellipse", {
                cx: 101,
                cy: 147,
                rx: 4,
                ry: 5,
                fill: featureColor,
              }),
            ]
          : state === "sleepy"
            ? [
                path("mouth", "M94 148q7 4 14 0", {
                  fill: "none",
                  stroke: featureColor,
                  "stroke-width": 3,
                  "stroke-linecap": "round",
                }),
              ]
            : state === "error"
              ? [
                  path("mouth", "m91 148 8-7 8 7", {
                    fill: "none",
                    stroke: featureColor,
                    "stroke-width": 4,
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                  }),
                ]
              : [];

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
    ...mouth,
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
