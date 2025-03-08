import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_i18n/flutter_i18n.dart';
import 'package:frosy_pine/features/core/domain/use_cases/retrieve_available_stores_use_case.dart';
import 'package:frosy_pine/features/in_memory_persistence/application/repositories/store_repository_in_memory_impl.dart';
import 'package:frosy_pine/features/ui/presentation/application.dart';
import 'package:frosy_pine/features/ui/presentation/utils/bloc_observer.dart';
import 'package:frosy_pine/features/ui/presentation/widgets/transactions/state/new_transaction_cubit.dart';
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

  runApp(FrostyPine(flutterI18nDelegate: flutterI18nDelegate, newTransactionCubit: newTransactionCubit));
}
