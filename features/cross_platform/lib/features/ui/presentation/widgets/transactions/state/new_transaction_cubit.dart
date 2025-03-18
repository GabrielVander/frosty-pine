import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosy_pine/features/core/domain/entities/product.dart';
import 'package:frosy_pine/features/core/domain/entities/store.dart';
import 'package:frosy_pine/features/core/domain/use_cases/retrieve_available_products_use_case.dart';
import 'package:frosy_pine/features/core/domain/use_cases/retrieve_available_stores_use_case.dart';
import 'package:intl/intl.dart';
import 'package:rust/rust.dart';

class NewTransactionCubit extends Cubit<NewTransactionCubitState> {
  NewTransactionCubit({
    required DateFormat dateFormat,
    required RetrieveAvailableStoresUseCase retrieveAvailableStoresUseCase,
    required RetrieveAvailableProductsUseCase retrieveAvailableProductsUseCase,
  }) : _dateFormat = dateFormat,
       _retrieveAvailableStoresUseCase = retrieveAvailableStoresUseCase,
       _retrieveAvailableProductsUseCase = retrieveAvailableProductsUseCase,
       super(
         NewTransactionCubitState(
           loading: true,
           initialized: false,
           date: '',
           selectedStoreValue: null,
           stores: const <StoreDisplayModel>[],
           products: const <ProductDisplayModel>[],
           items: const <TransactionItemDisplayModel>[],
           units: List.of([
             const UnitDisplayModel(value: 'none', displayTextI18nIdentifier: 'none'),
             const UnitDisplayModel(value: 'quantity', displayTextI18nIdentifier: 'quantity'),
             const UnitDisplayModel(value: 'kilograms', displayTextI18nIdentifier: 'kilograms'),
             const UnitDisplayModel(value: 'liters', displayTextI18nIdentifier: 'liters'),
           ]),
           showNewItemButton: true,
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
    final List<TransactionItemDisplayModel> newItems = List.from(state.items)..add(
      const TransactionItemDisplayModel(key: '', isNewItem: true, unitValue: '', productValue: '', unitaryPriceDisplay: '', shouldDisplayPriceField: true),
    );

    emit(state.copyWith(items: newItems, showNewItemButton: false));
  }

  void selectStore(String? storeValue) => emit(state.copyWith(selectedStoreValue: Some(storeValue)));

  void setDate(DateTime? value) => emit(state.copyWith(date: Some(value == null ? null : _dateFormat.format(value))));
}

@immutable
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
    required this.showNewItemButton,
  });

  final bool loading;
  final bool initialized;
  final String? selectedStoreValue;
  final String? date;
  final List<StoreDisplayModel> stores;
  final List<ProductDisplayModel> products;
  final List<TransactionItemDisplayModel> items;
  final List<UnitDisplayModel> units;
  final bool showNewItemButton;

  @override
  List<Object?> get props => [loading, initialized, selectedStoreValue, date, products, stores, items, units, showNewItemButton];

  NewTransactionCubitState copyWith({
    bool? loading,
    bool? initialized,
    Option<String?>? selectedStoreValue,
    Option<String?>? date,
    List<StoreDisplayModel>? stores,
    List<ProductDisplayModel>? products,
    List<TransactionItemDisplayModel>? items,
    List<UnitDisplayModel>? units,
    bool? showNewItemButton,
  }) => NewTransactionCubitState(
    loading: loading ?? this.loading,
    initialized: initialized ?? this.initialized,
    selectedStoreValue: selectedStoreValue?.toNullable() ?? this.selectedStoreValue,
    date: date?.toNullable() ?? this.date,
    stores: stores ?? this.stores,
    products: products ?? this.products,
    items: items ?? this.items,
    units: units ?? this.units,
    showNewItemButton: showNewItemButton ?? this.showNewItemButton,
  );

  @override
  String toString() {
    return 'NewTransactionCubitState(loading: $loading, initialized: $initialized, selectedStoreValue: $selectedStoreValue, date: $date, products: $products, items: $items, units: $units, showNewItem: $showNewItemButton)';
  }
}

@immutable
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

@immutable
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

@immutable
final class TransactionItemDisplayModel extends Equatable {
  const TransactionItemDisplayModel({
    required this.isNewItem,
    required this.key,
    required this.productValue,
    required this.unitValue,
    required this.shouldDisplayPriceField,
    required this.unitaryPriceDisplay,
  });

  final bool isNewItem;
  final String key;
  final String productValue;
  final String unitValue;
  final bool shouldDisplayPriceField;
  final String? unitaryPriceDisplay;

  TransactionItemDisplayModel copyWith({
    bool? isNewItem,
    String? key,
    String? productValue,
    String? unitValue,
    bool? shouldDisplayPriceField,
    Option<String?>? unitaryPriceDisplay,
  }) => TransactionItemDisplayModel(
    isNewItem: isNewItem ?? this.isNewItem,
    key: key ?? this.key,
    unitValue: unitValue ?? this.unitValue,
    productValue: productValue ?? this.productValue,
    shouldDisplayPriceField: shouldDisplayPriceField ?? this.shouldDisplayPriceField,
    unitaryPriceDisplay: unitaryPriceDisplay?.toNullable() ?? this.unitaryPriceDisplay,
  );

  @override
  List<Object?> get props => [isNewItem, key, productValue, unitValue, shouldDisplayPriceField, unitaryPriceDisplay];
}

@immutable
final class UnitDisplayModel extends Equatable {
  const UnitDisplayModel({required this.value, required this.displayTextI18nIdentifier});

  final String value;
  final String displayTextI18nIdentifier;

  @override
  String toString() => 'UnitDisplayModel(value: $value, displayTextI18nIdentifier: $displayTextI18nIdentifier)';

  @override
  List<Object?> get props => [value, displayTextI18nIdentifier];
}
