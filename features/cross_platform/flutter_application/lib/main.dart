import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/glue/frb_generated.dart';
import 'package:flutter_application/src/presentation/pages/main_page.dart';
import 'package:flutter_application/src/presentation/utils/bloc_observer.dart';
import 'package:flutter_application/src/presentation/widgets/transactions/state/new_transaction_cubit.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_i18n/flutter_i18n.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

Future<void> main() async {
  await RustLib.init();

  Bloc.observer = SimpleBlocObserver();
  final FlutterI18nDelegate flutterI18nDelegate = FlutterI18nDelegate(
    translationLoader: FileTranslationLoader(useCountryCode: true, basePath: 'assets/i18n', fallbackFile: 'en_us.yaml'),
    missingTranslationHandler: (key, locale) => print('[$locale] Key not found: $key'),
  );

  final NewTransactionCubit newTransactionCubit = NewTransactionCubit();

  WidgetsFlutterBinding.ensureInitialized();
  runApp(MyApp(flutterI18nDelegate: flutterI18nDelegate, newTransactionCubit: newTransactionCubit));
}

class MyApp extends StatelessWidget {
  const MyApp({required this.flutterI18nDelegate, required this.newTransactionCubit, super.key});

  final FlutterI18nDelegate flutterI18nDelegate;
  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      localizationsDelegates: [
        flutterI18nDelegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ],
      builder: FlutterI18n.rootAppBuilder(),
      home: MainPage(newTransactionCubit: newTransactionCubit),
    );
  }
}
