import assert from 'node:assert/strict';
import test from 'node:test';

const SHA = '5dbda2127357b4be87821902d36e4ce9560f6876';
const BASE = `https://raw.githubusercontent.com/ORESoftware/ores-interfaces/${SHA}/contracts/ores-compose-machine/v1`;
const response = await fetch(`${BASE}/authored.schema.json`);
assert.equal(response.status, 200);
const schema = await response.json();
const defs = schema.$defs;

test('browser-facing machine responses keep u64 identities as decimal strings', () => {
  assert.equal(defs.EnqueueResponse.properties.job_id.type, 'string');
  assert.equal(defs.JobStatusResponse.properties.job_id.type, 'string');
  assert.equal(defs.ActiveSystem.properties.generation.type, 'string');
  for (const p of [defs.EnqueueResponse.properties.job_id.pattern, defs.JobStatusResponse.properties.job_id.pattern, defs.ActiveSystem.properties.generation.pattern]) {
    assert.equal(p, '^[1-9][0-9]{0,19}$');
  }
});

test('browser clients cannot request arbitrary commands URLs or filesystem paths', () => {
  for (const field of ['command','argv','shell','cwd','url','origin','host','port','path']) {
    assert.equal(defs.EnsureRequest.properties[field], undefined, field);
  }
  assert.equal(defs.EnsureRequest.additionalProperties, false);
});

test('machine ingress exposed to UI layers is loopback or Unix only', () => {
  const p = new RegExp(defs.MachineIngress.properties.authority.pattern);
  assert.ok(p.test('127.0.0.1:42001'));
  assert.ok(p.test('[::1]:42001'));
  assert.ok(p.test('/tmp/ores-compose/ui.sock'));
  for (const value of ['0.0.0.0:42001','10.0.0.3:42001','192.168.0.8:42001','example.com:443']) assert.equal(p.test(value), false, value);
});

test('machine wire objects do not carry auth tokens cookies or secrets into visualization clients', () => {
  const forbidden = ['access_token','refresh_token','id_token','cookie','authorization','client_secret','password','api_key'];
  for (const model of Object.values(defs)) {
    if (!model.properties) continue;
    for (const field of forbidden) assert.equal(model.properties[field], undefined, `${field} leaked`);
  }
});

test('schema version and snake_case fields stay stable for generated browser/WASM bindings', () => {
  assert.equal(defs.EnsureRequest.properties.schema_version.const, 'ores.compose.machine.v1');
  assert.equal(defs.EnqueueResponse.properties.schema_version.const, 'ores.compose.machine.v1');
  assert.ok(defs.EnqueueResponse.properties.job_id);
  assert.equal(defs.EnqueueResponse.properties.jobId, undefined);
  assert.equal(defs.EnsureRequest.properties.schemaVersion, undefined);
});
