import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_i18n/flutter_i18n.dart';
import 'package:frosty_pine/adapters/repositories/brand_repository_in_memory_impl.dart' as dart_brand_repository_in_memory_impl;
import 'package:frosty_pine/adapters/repositories/category_repository_in_memory_impl.dart';
import 'package:frosty_pine/adapters/repositories/product_repository_in_memory_impl.dart';
import 'package:frosty_pine/adapters/repositories/store_repository_in_memory_impl.dart';
import 'package:frosty_pine/adapters/translations/add_new_brand_use_case.dart';
import 'package:frosty_pine/adapters/translations/rust_factory.dart';
import 'package:frosty_pine/domain/use_cases/retrieve_available_products_use_case.dart';
import 'package:frosty_pine/domain/use_cases/retrieve_available_stores_use_case.dart';
import 'package:frosty_pine/frb_generated.dart';
import 'package:frosty_pine/infrastructures/ui/application.dart';
import 'package:frosty_pine/infrastructures/ui/utils/bloc_observer.dart';
import 'package:frosty_pine/infrastructures/ui/widgets/brands/state/brand_cubit.dart';
import 'package:frosty_pine/infrastructures/ui/widgets/transactions/state/new_transaction_cubit.dart';
import 'package:intl/intl.dart';

Future<void> main() async {
  Bloc.observer = SimpleBlocObserver();
  await RustLib.init();

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

  final dart_brand_repository_in_memory_impl.BrandRepositoryInMemoryImpl brandRepositoryInMemoryImpl =
      dart_brand_repository_in_memory_impl.BrandRepositoryInMemoryImpl(
        data: Map<String, String>.of(<String, String>{'9072B82A-D917-44C7-B4CF-1121F5451F82': 'pocket', 'EC233749-CCCF-4E8A-8173-F4B9725DF6AA': 'address'}),
      );

  final CategoryRepositoryInMemoryImpl categoryRepositoryInMemoryImpl = CategoryRepositoryInMemoryImpl(
    data: Map<String, String>.of(<String, String>{'3EA62CC1-D5FE-459E-A0C2-E4DA75C95C08': 'seminars', '36B5A7D1-A73D-4717-91CA-93A49DD54678': 'debug'}),
  );

  final ProductRepositoryInMemoryImpl productRepositoryInMemoryImpl = ProductRepositoryInMemoryImpl(
    data: Map<String, ProductModel>.of(<String, ProductModel>{
      'E11D5CE8-D71B-45AF-ABE9-BD14EB6E3B66': ProductModel(
        name: 'picking',
        brandId: '9072B82A-D917-44C7-B4CF-1121F5451F82',
        categoryId: '3EA62CC1-D5FE-459E-A0C2-E4DA75C95C08',
      ),
    }),
  );

  final ArcBrandRepository brandRepository = await RustFactory.brandRepositoryInMemoryImpl(initialData: List.empty());

  final ArcFlutterPresenter flutterPresenter = await RustFactory.flutterPresenter();

  final RetrieveAvailableStoresUseCase retrieveAvailableStoresUseCase = RetrieveAvailableStoresUseCase(storeRepository: storeRepositoryInMemoryImpl);
  final RetrieveAvailableProductsUseCase retrieveAvailableProductsUseCase = RetrieveAvailableProductsUseCase(
    productRepository: productRepositoryInMemoryImpl,
    brandRepository: brandRepositoryInMemoryImpl,
    categoryRepository: categoryRepositoryInMemoryImpl,
  );
  final AddNewBrandUseCase addNewBrandUseCase = AddNewBrandUseCase(brandRepository: brandRepository, flutterPresenter: flutterPresenter);

  final BrandCubit brandCubit = BrandCubit(addNewBrandUseCase: addNewBrandUseCase);
  final NewTransactionCubit newTransactionCubit = NewTransactionCubit(
    dateFormat: DateFormat.yMMMd(),
    retrieveAvailableStoresUseCase: retrieveAvailableStoresUseCase,
    retrieveAvailableProductsUseCase: retrieveAvailableProductsUseCase,
  );

  WidgetsFlutterBinding.ensureInitialized();

  runApp(FrostyPine(flutterI18nDelegate: flutterI18nDelegate, newTransactionCubit: newTransactionCubit, brandCubit: brandCubit));
}
