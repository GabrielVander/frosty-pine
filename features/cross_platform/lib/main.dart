import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_i18n/flutter_i18n.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:frosy_pine/features/core/domain/use_cases/retrieve_available_stores_use_case.dart';
import 'package:frosy_pine/features/in_memory_persistence/data/repositories/store_repository_in_memory_impl.dart';
import 'package:frosy_pine/presentation/pages/main_page.dart';
import 'package:frosy_pine/presentation/utils/bloc_observer.dart';
import 'package:frosy_pine/presentation/widgets/transactions/state/new_transaction_cubit.dart';
import 'package:intl/intl.dart';

Future<void> main() async {
  Bloc.observer = SimpleBlocObserver();

  final FlutterI18nDelegate flutterI18nDelegate = FlutterI18nDelegate(
    translationLoader: FileTranslationLoader(useCountryCode: true, basePath: 'assets/i18n', fallbackFile: 'en_us.yaml'),
    missingTranslationHandler: (String key, Locale? locale) => print('[$locale] Key not found: $key'),
  );

  final StoreRepositoryInMemoryImpl storeRepositoryInMemoryImpl = StoreRepositoryInMemoryImpl(
    data: Map<String, String>.of(<String, String>{
      'C943DC1C-A294-433F-A8DF-74E103D7E632': 'Melissa Urenio',
      '80FA7922-8CFC-4B91-8D09-FEF6209C60EA': 'Laverne Dobrasz',
    }),
  );

  final RetrieveAvailableStoresUseCase retrieveAvailableStoresUseCase = RetrieveAvailableStoresUseCase(storeRepository: storeRepositoryInMemoryImpl);

  final NewTransactionCubit newTransactionCubit = NewTransactionCubit(
    dateFormat: DateFormat.yMMMd(),
    retrieveAvailableStoresUseCase: retrieveAvailableStoresUseCase,
  );

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
      localizationsDelegates: <LocalizationsDelegate<dynamic>>[flutterI18nDelegate, GlobalMaterialLocalizations.delegate, GlobalWidgetsLocalizations.delegate],
      builder: FlutterI18n.rootAppBuilder(),
      home: MainPage(newTransactionCubit: newTransactionCubit),
    );
  }
}
