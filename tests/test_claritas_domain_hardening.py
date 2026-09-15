import unittest

from deep_tests.contract_model import Command, IdempotencyConflict, ReferenceStore, generate_valid_trace, replay


class ClaritasDomainHardeningTests(unittest.TestCase):
    def test_duplicate_scene_revision_is_exactly_once(self) -> None:
        store = ReferenceStore()
        scene = Command("create", "scene-42", "camera-a|layers-v1", "scene-create-42")
        first = store.apply(scene)
        revision = store.revision
        for _ in range(36):
            self.assertEqual(store.apply(scene), first)
        self.assertEqual(store.revision, revision)

    def test_scene_idempotency_key_cannot_change_render_intent(self) -> None:
        store = ReferenceStore()
        store.apply(Command("create", "scene-42", "layers-v1", "stable-scene-key"))
        store.apply(Command("create", "scene-99", "layers-other", "scene-99"))
        with self.assertRaises(IdempotencyConflict):
            store.apply(Command("update", "scene-42", "layers-v2", "stable-scene-key"))

    def test_visualization_trace_converges_across_duplicate_schedules(self) -> None:
        commands = generate_valid_trace(2026091404, steps=780)
        snapshots = {replay(commands, duplicate_every=n).snapshot() for n in (2, 6, 13, 23)}
        self.assertEqual(len(snapshots), 1)


if __name__ == "__main__":
    unittest.main()
