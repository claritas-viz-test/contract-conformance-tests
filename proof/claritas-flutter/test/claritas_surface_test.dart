import 'package:claritas_flutter/claritas_flutter.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('contract version is stable', () {
    expect(claritasFlutterContractVersion, 'claritas-viz.flutter.v1');
  });

  testWidgets('surface preserves child and semantic label', (tester) async {
    await tester.pumpWidget(
      const Directionality(
        textDirection: TextDirection.ltr,
        child: ClaritasSurface(
          semanticLabel: 'Revenue visualization',
          child: Text('chart fixture'),
        ),
      ),
    );

    expect(find.text('chart fixture'), findsOneWidget);

    final semantics = tester.widget<Semantics>(find.byType(Semantics));
    expect(semantics.properties.container, isTrue);
    expect(semantics.properties.label, 'Revenue visualization');
  });
}
