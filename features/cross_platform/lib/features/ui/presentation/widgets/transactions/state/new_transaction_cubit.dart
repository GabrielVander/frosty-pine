import 'dart:ui';

import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosy_pine/features/core/domain/entities/product.dart';
import 'package:frosy_pine/features/core/domain/entities/store.dart';
import 'package:frosy_pine/features/core/domain/use_cases/retrieve_available_products_use_case.dart';
import 'package:frosy_pine/features/core/domain/use_cases/retrieve_available_stores_use_case.dart';
import 'package:intl/intl.dart';
import 'package:rust/rust.dart';
import 'package:uuid/uuid.dart';

class NewTransactionCubit extends Cubit<NewTransactionCubitState> {
  NewTransactionCubit({
    required DateFormat dateFormat,
    required RetrieveAvailableStoresUseCase retrieveAvailableStoresUseCase,
    required RetrieveAvailableProductsUseCase retrieveAvailableProductsUseCase,
  }) : _dateFormat = dateFormat,
       _retrieveAvailableStoresUseCase = retrieveAvailableStoresUseCase,
       _retrieveAvailableProductsUseCase = retrieveAvailableProductsUseCase,
       super(
         const NewTransactionCubitState(
           loading: true,
           initialized: false,
           date: '',
           selectedStoreValue: null,
           stores: <StoreDisplayModel>[],
           products: <ProductDisplayModel>[],
           items: <TransactionItemDisplayModel>[],
           units: UnitDisplayModel.values,
         ),
       );

  final DateFormat _dateFormat;
  final RetrieveAvailableStoresUseCase _retrieveAvailableStoresUseCase;
  final RetrieveAvailableProductsUseCase _retrieveAvailableProductsUseCase;

  Future<void> loadStores() async {
    emit(state.copyWith(loading: true));

    await _retrieveAvailableStoresUseCase
        .execute()
        .map<List<StoreDisplayModel>>((List<Store> stores) => stores.map<StoreDisplayModel>(StoreDisplayModel.fromStoreEntity).toList())
        .inspect((List<StoreDisplayModel> models) => emit(state.copyWith(loading: false, initialized: true, stores: models)));
  }

  Future<void> loadProducts() async {
    emit(state.copyWith(loading: true));

    await _retrieveAvailableProductsUseCase
        .execute()
        .map<List<ProductDisplayModel>>((List<Product> products) => products.map<ProductDisplayModel>(ProductDisplayModel.fromProductEntity).toList())
        .inspect((List<ProductDisplayModel> models) => emit(state.copyWith(loading: false, initialized: true, products: models)));
  }

  void showNewItem() {
    emit(
      state.copyWith(
        items: List<TransactionItemDisplayModel>.from(state.items)..add(
          TransactionItemDisplayModel(
            key: const Uuid().v4(),
            unitValue: null,
            productValue: null,
            unitAmount: null,
            unitaryPriceDisplay: '',
            shouldDisplayPriceField: true,
          ),
        ),
      ),
    );
  }

  void selectStore(String? storeValue) => emit(state.copyWith(selectedStoreValue: Some<String?>(storeValue)));

  void setDate(DateTime? value) => emit(state.copyWith(date: Some<String?>(value == null ? null : _dateFormat.format(value))));

  void setProductValue(String itemKey, String? value) {
    emit(
      state.copyWith(
        items:
            List<TransactionItemDisplayModel>.from(state.items)
                .map<TransactionItemDisplayModel>((TransactionItemDisplayModel i) => i.key == itemKey ? i.copyWith(productValue: Some<String?>(value)) : i)
                .toList(),
      ),
    );
  }

  void setUnitValue(String itemKey, UnitDisplayModel? value) => emit(
    state.copyWith(
      items:
          List<TransactionItemDisplayModel>.from(state.items)
              .map<TransactionItemDisplayModel>((TransactionItemDisplayModel i) => i.key == itemKey ? i.copyWith(unitValue: Some<UnitDisplayModel?>(value)) : i)
              .toList(),
    ),
  );

  void setUnitAmount(String itemKey, String? value, Locale locale) => emit(
    state.copyWith(
      items:
          List<TransactionItemDisplayModel>.from(state.items).map<TransactionItemDisplayModel>((TransactionItemDisplayModel i) {
            final String Function(String) numberFormatter =
                i.unitValue?.hasDecimalPlaces ?? false
                    ? (String number) =>
                        NumberFormat.decimalPatternDigits(locale: locale.toLanguageTag(), decimalDigits: 3).format((int.tryParse(number) ?? 0) / 1000)
                    : (String number) => NumberFormat.compact(locale: locale.toLanguageTag()).format(int.tryParse(number) ?? 0);

            return i.key == itemKey ? i.copyWith(unitAmount: Some<String?>(value != null && value.isNotEmpty ? numberFormatter(value) : value)) : i;
          }).toList(),
    ),
  );
}

final class NewTransactionCubitState extends Equatable {
  const NewTransactionCubitState({
    required this.loading,
    required this.initialized,
    required this.selectedStoreValue,
    required this.date,
    required this.stores,
    required this.products,
    required this.items,
    required this.units,
  });

  final bool loading;
  final bool initialized;
  final String? selectedStoreValue;
  final String? date;
  final List<StoreDisplayModel> stores;
  final List<ProductDisplayModel> products;
  final List<TransactionItemDisplayModel> items;
  final List<UnitDisplayModel> units;

  bool get showAddItemButton => !items.any((TransactionItemDisplayModel i) => i.isPlaceholder);

  @override
  List<Object?> get props => [loading, initialized, selectedStoreValue, date, products, stores, items, units, showAddItemButton];

  NewTransactionCubitState copyWith({
    bool? loading,
    bool? initialized,
    Option<String?>? selectedStoreValue,
    Option<String?>? date,
    List<StoreDisplayModel>? stores,
    List<ProductDisplayModel>? products,
    List<TransactionItemDisplayModel>? items,
    List<UnitDisplayModel>? units,
  }) => NewTransactionCubitState(
    loading: loading ?? this.loading,
    initialized: initialized ?? this.initialized,
    selectedStoreValue: selectedStoreValue?.toNullable() ?? this.selectedStoreValue,
    date: date?.toNullable() ?? this.date,
    stores: stores ?? this.stores,
    products: products ?? this.products,
    items: items ?? this.items,
    units: units ?? this.units,
  );

  @override
  String toString() =>
      'NewTransactionCubitState(loading: $loading, initialized: $initialized, selectedStoreValue: $selectedStoreValue, date: $date, products: $products, items: $items, units: $units, showAddItemButton: $showAddItemButton)';
}

final class StoreDisplayModel extends Equatable {
  const StoreDisplayModel({required this.value, required this.displayName});

  factory StoreDisplayModel.fromStoreEntity(Store entity) => StoreDisplayModel(value: entity.id, displayName: entity.name);

  final String value;
  final String displayName;

  StoreDisplayModel copyWith({String? value, String? displayName}) =>
      StoreDisplayModel(value: value ?? this.value, displayName: displayName ?? this.displayName);

  @override
  List<Object?> get props => [value, displayName];

  @override
  String toString() {
    return 'StoreDisplayModel(value : $value , displayName: $displayName)';
  }
}

final class ProductDisplayModel extends Equatable {
  const ProductDisplayModel({required this.value, required this.displayName});

  factory ProductDisplayModel.fromProductEntity(Product entity) =>
      ProductDisplayModel(value: entity.id, displayName: '[${entity.category.name}] [${entity.brand.name}] ${entity.name}');

  final String value;
  final String displayName;

  ProductDisplayModel copyWith({String? value, String? displayName}) =>
      ProductDisplayModel(value: value ?? this.value, displayName: displayName ?? this.displayName);

  @override
  List<Object> get props => [value, displayName];
}

final class TransactionItemDisplayModel extends Equatable {
  const TransactionItemDisplayModel({
    required this.key,
    required this.productValue,
    required this.unitValue,
    required this.unitAmount,
    required this.shouldDisplayPriceField,
    required this.unitaryPriceDisplay,
  });

  final String key;
  final String? productValue;
  final UnitDisplayModel? unitValue;
  final String? unitAmount;
  final bool shouldDisplayPriceField;
  final String? unitaryPriceDisplay;

  bool get isPlaceholder =>
      productValue == null ||
      unitValue == null ||
      unitValue!.requiresNumericalValue && (unitAmount == null || unitAmount!.isEmpty) ||
      unitaryPriceDisplay == null;

  TransactionItemDisplayModel copyWith({
    String? key,
    Option<String?>? productValue,
    Option<UnitDisplayModel?>? unitValue,
    Option<String?>? unitAmount,
    bool? shouldDisplayPriceField,
    Option<String?>? unitaryPriceDisplay,
  }) => TransactionItemDisplayModel(
    key: key ?? this.key,
    unitValue: unitValue?.toNullable() ?? this.unitValue,
    productValue: productValue?.toNullable() ?? this.productValue,
    unitAmount: unitAmount?.expect('unitAmount should be set') ?? this.unitAmount,
    shouldDisplayPriceField: shouldDisplayPriceField ?? this.shouldDisplayPriceField,
    unitaryPriceDisplay: unitaryPriceDisplay?.toNullable() ?? this.unitaryPriceDisplay,
  );

  @override
  List<Object?> get props => [isPlaceholder, key, productValue, unitValue, unitAmount, shouldDisplayPriceField, unitaryPriceDisplay];

  @override
  String toString() =>
      'TransactionItemDisplayModel(isPlaceholder: $isPlaceholder, key: $key, unitValue: $unitValue, productValue: $productValue, unitAmount: $unitAmount, shouldDisplayPriceField: $shouldDisplayPriceField, unitaryPriceDisplay: $unitaryPriceDisplay)';
}

enum UnitDisplayModel {
  none(i18nIdentifier: 'none', requiresNumericalValue: false, hasDecimalPlaces: false),
  quantity(i18nIdentifier: 'quantity', requiresNumericalValue: true, hasDecimalPlaces: false),
  kilograms(i18nIdentifier: 'kilograms', requiresNumericalValue: true, hasDecimalPlaces: true),
  liters(i18nIdentifier: 'liters', requiresNumericalValue: true, hasDecimalPlaces: true);

  const UnitDisplayModel({required this.i18nIdentifier, required this.requiresNumericalValue, required this.hasDecimalPlaces});

  final String i18nIdentifier;
  final bool requiresNumericalValue;
  final bool hasDecimalPlaces;

  @override
  String toString() =>
      'UnitDisplayModel(i18nIdentifier: $i18nIdentifier, requiresNumericalValue: $requiresNumericalValue, hasDecimalPlaces: $hasDecimalPlaces)';
}
