import { describe, expect, test } from "bun:test";
import { updateNodeProps } from "./node-ops";

describe("updateNodeProps", () => {
  test("removes legacy aliases when a property is explicitly cleared", () => {
    const doc = {
      version: "2.0",
      meta: {
        name: "Test",
        documentType: "test",
        entityType: "student",
        locale: "en",
        pageSetup: {
          size: "A4",
          orientation: "portrait",
          margins: { top: 0, bottom: 0, left: 0, right: 0 },
          unit: "mm",
        },
      },
      variables: {},
      assets: {},
      pages: [
        {
          type: "Page",
          id: "page",
          children: [
            {
              type: "Shape",
              id: "shape",
              props: {
                shapeType: "rectangle",
                fill: "#111111",
                fillColor: "#111111",
              },
            },
          ],
        },
      ],
    };

    const result = updateNodeProps(doc, "shape", {
      fill: "#222222",
      fillColor: undefined,
    });
    const shape = result.pages[0]?.children[0];

    expect(shape?.type).toBe("Shape");
    if (shape?.type !== "Shape") throw new Error("Expected a shape node");
    expect(shape.props.fill).toBe("#222222");
    expect("fillColor" in shape.props).toBe(false);
  });
});
