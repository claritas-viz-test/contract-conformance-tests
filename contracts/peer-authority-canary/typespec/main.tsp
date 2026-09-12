import "@typespec/json-schema";
using TypeSpec.JsonSchema;

// Synthetic independent TypeSpec authority used only by this public test org.
@jsonSchema
namespace ClaritasTest.Peer.V1;

model CanaryPacket {
  ...Record<never>;
  @minLength(1) @maxLength(64) id: string;
  @minLength(1) @maxLength(256) payload: string;
  @minValue(0) @maxValue(100) revision: integer;
  active: boolean;
}
