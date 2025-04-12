import 'package:flutter/material.dart';
import 'package:flutter_i18n/flutter_i18n.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:frosty_pine/infrastructures/ui/pages/main_page.dart';
import 'package:frosty_pine/infrastructures/ui/widgets/brands/state/brand_cubit.dart';
import 'package:frosty_pine/infrastructures/ui/widgets/transactions/state/new_transaction_cubit.dart';

class FrostyPine extends StatelessWidget {
  const FrostyPine({required this.flutterI18nDelegate, required this.newTransactionCubit, required this.brandCubit, super.key});

  final FlutterI18nDelegate flutterI18nDelegate;
  final NewTransactionCubit newTransactionCubit;
  final BrandCubit brandCubit;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      localizationsDelegates: <LocalizationsDelegate<dynamic>>[flutterI18nDelegate, GlobalMaterialLocalizations.delegate, GlobalWidgetsLocalizations.delegate],
      builder: FlutterI18n.rootAppBuilder(),
      home: MainPage(newTransactionCubit: newTransactionCubit, brandCubit: brandCubit),
    );
  }
}
