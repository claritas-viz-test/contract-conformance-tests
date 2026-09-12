// Independent TypeSpec authority A. The generated JSON Schema B is comparison evidence only.
import "@typespec/json-schema";
using TypeSpec.JsonSchema;

namespace Claritas.Test.VisualizationCatalog;

enum VisualizationFamily {
  chart: "chart",
  plot: "plot",
  graph: "graph",
  diagram: "diagram",
  map: "map",
  matrix: "matrix",
  timeline: "timeline",
  scientific: "scientific",
  formal: "formal",
  hybrid: "hybrid",
}

enum RendererTarget {
  mash: "mash",
  leptos: "leptos",
  dioxus: "dioxus",
  rustWasm: "rust-wasm",
  flutter: "flutter",
  htmlSvg: "html-svg",
  htmx: "htmx",
  webComponents: "web-components",
  webviewBridge: "webview-bridge",
  qtQml: "qt-qml",
}

model VisualizationDescriptorV1 {
  @minLength(1)
  @maxLength(96)
  id: string;

  @minLength(1)
  @maxLength(128)
  title: string;

  family: VisualizationFamily;

  @minItems(1)
  @maxItems(10)
  renderers: RendererTarget[];
}

model VisualizationCatalogV1 {
  @minValue(1)
  @maxValue(1)
  version: integer;

  @minItems(1)
  @maxItems(512)
  visualizations: VisualizationDescriptorV1[];
}
