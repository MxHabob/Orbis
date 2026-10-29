import {
  evaluateCondition,
  evaluateExpression,
  getPages,
  resolvePath,
  type TemplateDocument,
  type TemplateNode,
} from "@docforge/core";
import { useQueries } from "@tanstack/react-query";
import { memo, type ReactNode, useMemo } from "react";
import { Text, useWindowDimensions, View } from "react-native";
import QRCode from "react-native-qrcode-svg";
import Svg, {
  Ellipse,
  G,
  Line,
  Rect,
  Image as SvgImage,
  Text as SvgText,
} from "react-native-svg";
import { api } from "@/lib/api/client";
import { queryKeys } from "@/lib/api/query-keys";
import type { OrgDto, StudentDto, TemplateDto } from "@/lib/api/types";
import { useI18n } from "@/lib/i18n";

type TemplateContext = Record<string, unknown>;

type QrOverlay = {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  value: string;
  color: string;
  backgroundColor: string;
  errorCorrection: "L" | "M" | "Q" | "H";
  quietZone: number;
  logoUrl?: string;
};

function interpolate(value: string, context: TemplateContext): string {
  if (value.startsWith("$") && !value.includes("{{")) {
    const resolved = resolvePath(context, value.slice(1));
    return resolved == null ? "" : String(resolved);
  }
  return evaluateExpression(value, context);
}

function getProps(node: TemplateNode): Record<string, unknown> {
  return "props" in node && node.props
    ? (node.props as Record<string, unknown>)
    : {};
}

function getPosition(
  node: TemplateNode,
  pageWidth: number,
  pageHeight: number,
): { x: number; y: number; width: number; height: number } {
  const props = getProps(node);
  const position =
    (props.position as
      | Record<string, number | string | undefined>
      | undefined) ??
    (
      node as unknown as {
        position?: Record<string, number | string | undefined>;
      }
    ).position;
  const size = (
    value: number | string | undefined,
    total: number,
    fallback: number,
  ) => {
    if (typeof value === "string" && value.endsWith("%")) {
      return (Number.parseFloat(value) / 100) * total;
    }
    const parsed = typeof value === "number" ? value : Number(value);
    return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
  };

  return {
    x: Number(position?.x) || 0,
    y: Number(position?.y) || 0,
    width: size(
      position?.width ?? (props.width as number | string | undefined),
      pageWidth,
      pageWidth,
    ),
    height: size(
      position?.height ?? (props.height as number | string | undefined),
      pageHeight,
      Number(props.fontSize) > 0 ? Number(props.fontSize) * 0.53 : 8,
    ),
  };
}

function renderNode(
  node: TemplateNode,
  document: TemplateDocument,
  context: TemplateContext,
  pageWidth: number,
  pageHeight: number,
  qrOverlays: QrOverlay[],
  assets: Record<string, string>,
  parentX = 0,
  parentY = 0,
): ReactNode {
  if (node.visible === false) return null;

  const props = {
    ...(node.styleRef ? document.styles?.[node.styleRef]?.props : {}),
    ...getProps(node),
  };
  const { x, y, width, height } = getPosition(node, pageWidth, pageHeight);
  const positionX = parentX + x;
  const positionY = parentY + y;
  const opacity = node.opacity ?? 1;
  const rotation = node.rotation ?? Number(props.rotation) ?? 0;
  const transform = rotation
    ? `rotate(${rotation} ${width / 2} ${height / 2})`
    : undefined;

  if (node.type === "QRCode") {
    const value = interpolate(String(props.data ?? ""), context);
    if (value) {
      qrOverlays.push({
        id: node.id,
        x: positionX,
        y: positionY,
        width,
        height,
        value,
        color: String(props.color ?? "#000000"),
        backgroundColor: String(props.backgroundColor ?? "#ffffff"),
        errorCorrection:
          (props.errorCorrection as QrOverlay["errorCorrection"]) ?? "M",
        quietZone: Number(props.quietZone ?? 1),
        logoUrl: props.logoAssetRef
          ? assets[String(props.logoAssetRef)]
          : undefined,
      });
    }
    return null;
  }

  if (node.type === "Conditional") {
    const condition = String(props.condition ?? "");
    const branches = evaluateCondition(condition, context)
      ? node.children
      : (node.elseChildren ?? []);
    return (
      <G key={node.id} transform={transform} opacity={opacity}>
        {branches.map((child) =>
          renderNode(
            child,
            document,
            context,
            pageWidth,
            pageHeight,
            qrOverlays,
            assets,
            positionX,
            positionY,
          ),
        )}
      </G>
    );
  }

  if (
    node.type === "Frame" ||
    node.type === "Grid" ||
    node.type === "Header" ||
    node.type === "Footer" ||
    node.type === "Page" ||
    node.type === "Repeater"
  ) {
    const children =
      node.type === "Repeater"
        ? (() => {
            const rows = resolvePath(context, String(props.dataSource ?? ""));
            if (!Array.isArray(rows)) return [];
            return rows
              .slice(0, Number(props.limit) || rows.length)
              .flatMap((row, index) => {
                const childContext = {
                  ...context,
                  [String(props.itemVariable ?? "item")]: row,
                  ...(props.indexVariable
                    ? { [String(props.indexVariable)]: index }
                    : {}),
                };
                return node.children.map((child) => ({
                  node: child,
                  context: childContext,
                }));
              });
          })()
        : node.children.map((child) => ({ node: child, context }));
    return (
      <G key={node.id} transform={transform} opacity={opacity}>
        {node.type === "Frame" && props.backgroundColor ? (
          <Rect
            x={positionX}
            y={positionY}
            width={width}
            height={height}
            fill={String(props.backgroundColor)}
            stroke={String(props.borderColor ?? "none")}
            strokeWidth={Number(props.borderWidth ?? 0)}
            rx={Number(props.borderRadius ?? 0)}
          />
        ) : null}
        {children.map(({ node: child, context: childContext }) =>
          renderNode(
            child,
            document,
            childContext,
            pageWidth,
            pageHeight,
            qrOverlays,
            assets,
            positionX,
            positionY,
          ),
        )}
      </G>
    );
  }

  return (
    <G
      key={node.id}
      transform={`translate(${positionX} ${positionY})${transform ? ` ${transform}` : ""}`}
      opacity={opacity}
    >
      {node.type === "Shape" ? (
        props.shapeType === "circle" || props.shape === "circle" ? (
          <Ellipse
            cx={width / 2}
            cy={height / 2}
            rx={width / 2}
            ry={height / 2}
            fill={String(props.fill ?? props.fillColor ?? "transparent")}
            stroke={String(props.stroke ?? props.strokeColor ?? "none")}
            strokeWidth={Number(props.strokeWidth ?? 0)}
          />
        ) : props.shapeType === "line" ? (
          <Line
            x1={0}
            y1={height / 2}
            x2={width}
            y2={height / 2}
            stroke={String(props.stroke ?? props.strokeColor ?? "#000000")}
            strokeWidth={Number(props.strokeWidth ?? 1)}
          />
        ) : (
          <Rect
            width={width}
            height={height}
            rx={Number(props.borderRadius ?? 0)}
            fill={String(props.fill ?? props.fillColor ?? "transparent")}
            stroke={String(props.stroke ?? props.strokeColor ?? "none")}
            strokeWidth={Number(props.strokeWidth ?? 0)}
          />
        )
      ) : null}
      {node.type === "Text" ? (
        <SvgText
          x={
            textAnchor(props, context) === "middle"
              ? width / 2
              : textAnchor(props, context) === "end"
                ? width
                : 0
          }
          y={Math.min(height, Number(props.fontSize ?? 12) * 0.36)}
          fill={String(props.color ?? "#111827")}
          fontFamily={String(props.fontFamily ?? "Rubik")}
          fontSize={Number(props.fontSize ?? 12) * 0.3528}
          fontWeight={String(props.fontWeight ?? "normal")}
          fontStyle={props.fontStyle === "italic" ? "italic" : "normal"}
          textAnchor={textAnchor(props, context)}
          textDecoration={
            props.textDecoration === "underline" ||
            props.textDecoration === "line-through"
              ? props.textDecoration
              : "none"
          }
        >
          {interpolate(String(props.content ?? ""), context)}
        </SvgText>
      ) : null}
      {node.type === "Image" &&
      (props.src || props.expression || props.assetRef) ? (
        <SvgImage
          x={0}
          y={0}
          width={width}
          height={height}
          href={{
            uri:
              (typeof props.assetRef === "string"
                ? assets[props.assetRef]
                : undefined) ||
              interpolate(String(props.expression ?? props.src ?? ""), context),
          }}
          preserveAspectRatio={
            props.fit === "cover"
              ? "xMidYMid slice"
              : props.fit === "fill"
                ? "none"
                : "xMidYMid meet"
          }
        />
      ) : null}
      {node.type === "Separator" ? (
        <Line
          x1={0}
          y1={props.direction === "vertical" ? 0 : height / 2}
          x2={props.direction === "vertical" ? width / 2 : width}
          y2={props.direction === "vertical" ? height : height / 2}
          stroke={String(props.color ?? "#d1d5db")}
          strokeWidth={Number(props.thickness ?? 0.5)}
        />
      ) : null}
      {node.type === "Watermark" && props.watermarkType === "text" ? (
        <SvgText
          x={width / 2}
          y={height / 2}
          fill={String(props.color ?? "#94a3b8")}
          fillOpacity={Number(props.opacity ?? 0.15)}
          fontSize={Number(props.fontSize ?? 12) * 0.3528}
          textAnchor="middle"
        >
          {interpolate(String(props.text ?? ""), context)}
        </SvgText>
      ) : null}
    </G>
  );
}

function getPageDimensions(template: TemplateDto, document: TemplateDocument) {
  const pageSetup = document.meta?.pageSetup;
  const preset =
    template.page_size?.toUpperCase() ?? pageSetup?.size?.toUpperCase();
  let width = template.page_width_mm ?? pageSetup?.width ?? 210;
  let height = template.page_height_mm ?? pageSetup?.height ?? 297;
  if (preset === "ID_CARD") {
    width = 85.6;
    height = 53.98;
  } else if (preset === "BADGE") {
    width = 90;
    height = 60;
  }

  if (template.orientation === "landscape" && height > width) {
    [width, height] = [height, width];
  } else if (template.orientation === "portrait" && width > height) {
    [width, height] = [height, width];
  }
  return { width, height };
}

function collectUnsupportedNodes(nodes: TemplateNode[]): number {
  return nodes.reduce((count, node) => {
    const unsupported =
      node.type === "Barcode" ||
      node.type === "Table" ||
      (node.type === "Watermark" &&
        "props" in node &&
        node.props.watermarkType === "image") ||
      node.type === "ComponentRef";
    const children = "children" in node ? node.children : [];
    return count + Number(unsupported) + collectUnsupportedNodes(children);
  }, 0);
}

export const DigitalIdTemplate = memo(function DigitalIdTemplate({
  template,
  document,
  student,
  organization,
  isArabic,
}: {
  template: TemplateDto;
  document: TemplateDocument;
  student: StudentDto;
  organization?: OrgDto | null;
  isArabic: boolean;
}) {
  const { t } = useI18n();
  const { width: windowWidth } = useWindowDimensions();
  const dimensions = getPageDimensions(template, document);
  const cardWidth = Math.min(windowWidth - 40, 420);
  const cardHeight = (cardWidth * dimensions.height) / dimensions.width;
  const localizedOrganizationName = isArabic
    ? organization?.name_ar || organization?.name
    : organization?.name || organization?.name_ar;
  const referencedAssets = useMemo(
    () =>
      Object.entries(document.assets ?? {})
        .map(([reference, asset]) => ({
          reference,
          id: asset.assetId,
        }))
        .filter(({ id }) => Boolean(id)),
    [document.assets],
  );
  const assetQueries = useQueries({
    queries: referencedAssets.map(({ id }) => ({
      queryKey: queryKeys.storage.presignedUrl(id),
      queryFn: async () => {
        const response = await api.getAccessUrl({
          path: { id },
          body: { expires_in_seconds: 900 },
          throwOnError: true,
        });
        let data: unknown = response;
        while (typeof data === "object" && data !== null && "data" in data) {
          data = data.data;
        }
        if (
          typeof data !== "object" ||
          data === null ||
          !("url" in data) ||
          typeof data.url !== "string"
        ) {
          throw new Error("The template asset URL response was invalid.");
        }
        return data.url;
      },
      staleTime: 10 * 60 * 1000,
    })),
  });
  const resolvedAssets = useMemo(() => {
    const assets: Record<string, string> = {};
    referencedAssets.forEach(({ reference }, index) => {
      const url = assetQueries[index]?.data;
      if (url) assets[reference] = url;
    });
    return assets;
  }, [assetQueries, referencedAssets]);
  const context = useMemo<TemplateContext>(() => {
    const org = {
      ...(organization ?? {}),
      name: localizedOrganizationName ?? "",
      name_ar: organization?.name_ar ?? localizedOrganizationName ?? "",
      name_en: organization?.name ?? "",
    };
    const value: TemplateContext = {
      student: {
        ...student,
        name: isArabic
          ? student.name_ar || student.full_name
          : student.display_name || student.full_name,
        name_ar: student.name_ar || student.full_name,
        avatar: student.avatar_url ?? "",
      },
      org,
      organization: org,
      assets: resolvedAssets,
      locale: isArabic ? "ar" : "en",
    };
    for (const [key, definition] of Object.entries(document.variables ?? {})) {
      const source = definition.source.replace(/^\$/, "");
      const resolved =
        source === "computed" && definition.expression
          ? evaluateExpression(definition.expression, value)
          : resolvePath(value, source);
      value[key] = resolved ?? definition.defaultValue ?? "";
    }
    return value;
  }, [
    document.variables,
    isArabic,
    localizedOrganizationName,
    organization,
    resolvedAssets,
    student,
  ]);
  const qrOverlays: QrOverlay[] = [];
  const pages = getPages(document);
  const page = pages[0];
  const svgNodes = page
    ? page.children.map((node) =>
        renderNode(
          node,
          document,
          context,
          dimensions.width,
          dimensions.height,
          qrOverlays,
          resolvedAssets,
        ),
      )
    : [];
  const unsupportedNodeCount = pages.reduce(
    (count, currentPage) =>
      count + collectUnsupportedNodes(currentPage.children),
    0,
  );
  const assetLoadFailed = assetQueries.some(
    (query: { isError: boolean }) => query.isError,
  );
  const pageProps =
    page && "props" in page
      ? (page.props as Record<string, unknown> | undefined)
      : undefined;
  const backgroundColor = String(
    pageProps?.backgroundColor ?? document.theme?.secondaryColor ?? "#ffffff",
  );

  return (
    <View
      accessibilityLabel={
        isArabic ? template.name_ar || template.name : template.name
      }
      className="overflow-hidden rounded-xl border border-border bg-card"
      style={{
        width: cardWidth,
        height: cardHeight,
        maxWidth: "100%",
      }}
    >
      <Svg
        width="100%"
        height="100%"
        viewBox={`0 0 ${dimensions.width} ${dimensions.height}`}
        preserveAspectRatio="none"
      >
        <Rect
          width={dimensions.width}
          height={dimensions.height}
          fill={backgroundColor}
        />
        {svgNodes}
      </Svg>
      {qrOverlays.map((qr) => {
        const qrSize = Math.max(
          1,
          Math.round(
            cardWidth *
              Math.min(
                qr.width / dimensions.width,
                qr.height / dimensions.height,
              ),
          ),
        );
        return (
          <View
            key={qr.id}
            pointerEvents="none"
            style={{
              position: "absolute",
              left: `${(qr.x / dimensions.width) * 100}%`,
              top: `${(qr.y / dimensions.height) * 100}%`,
              width: `${(qr.width / dimensions.width) * 100}%`,
              height: `${(qr.height / dimensions.height) * 100}%`,
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <QRCode
              value={qr.value}
              size={qrSize}
              color={qr.color}
              backgroundColor={qr.backgroundColor}
              ecl={qr.errorCorrection}
              quietZone={qr.quietZone}
              logo={qr.logoUrl ? { uri: qr.logoUrl } : undefined}
            />
          </View>
        );
      })}
      {assetLoadFailed ? (
        <View className="absolute inset-x-0 bottom-0 bg-background/90 px-two py-one">
          <Text className="text-center text-[10px] text-destructive">
            {t("digital_id_assets_error")}
          </Text>
        </View>
      ) : unsupportedNodeCount > 0 ? (
        <View className="absolute inset-x-0 bottom-0 bg-background/90 px-two py-one">
          <Text className="text-center text-[10px] text-muted-foreground">
            {t("digital_id_unsupported_elements")}
          </Text>
        </View>
      ) : null}
    </View>
  );
});

function textAnchor(
  props: Record<string, unknown>,
  context: TemplateContext,
): "start" | "middle" | "end" {
  if (props.textAlign === "center") return "middle";
  if (props.textAlign === "end" || props.textAlign === "right") return "end";
  if (
    (props.textAlign === "start" || props.textAlign === "left") &&
    (props.direction === "rtl" || context.locale === "ar")
  ) {
    return "end";
  }
  return "start";
}
