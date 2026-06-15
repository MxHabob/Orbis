import {
  lazy,
  Suspense,
  type ComponentType,
  type LazyExoticComponent,
  type ReactNode,
} from "react";

export function lazyNamed<T extends ComponentType<unknown>>(
  factory: () => Promise<Record<string, T>>,
  exportName: string,
): LazyExoticComponent<T> {
  return lazy(() =>
    factory().then((mod) => {
      const component = mod[exportName];
      if (!component) {
        throw new Error(`Export "${exportName}" was not found in lazy module`);
      }
      return { default: component };
    }),
  );
}

export function withSuspense<P extends object>(
  Component: LazyExoticComponent<ComponentType<P>>,
  fallback: ReactNode = null,
) {
  return function SuspenseWrapper(props: P) {
    return (
      <Suspense fallback={fallback}>
        <Component {...props} />
      </Suspense>
    );
  };
}
