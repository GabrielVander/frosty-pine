import 'package:flutter/widgets.dart';
import 'package:flutter_i18n/flutter_i18n.dart';

extension FlutterI18nBuildContextExtension on BuildContext {
  String i18n(String key) => FlutterI18n.translate(this, key);
}
