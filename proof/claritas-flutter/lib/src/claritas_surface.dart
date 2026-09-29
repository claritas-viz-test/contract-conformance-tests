import 'package:flutter/widgets.dart';

/// Semantic contract version exposed by this presentation package.
const String claritasFlutterContractVersion = 'claritas-viz.flutter.v1';

/// A stable presentation boundary for embedding a Claritas visualization.
///
/// This widget owns Flutter presentation semantics only. Shared data, RPC, and
/// wire contracts remain authoritative in the Claritas interface layer.
class ClaritasSurface extends StatelessWidget {
  const ClaritasSurface({
    required this.child,
    this.semanticLabel = 'Claritas visualization',
    super.key,
  });

  final Widget child;
  final String semanticLabel;

  @override
  Widget build(BuildContext context) {
    return Semantics(container: true, label: semanticLabel, child: child);
  }
}
