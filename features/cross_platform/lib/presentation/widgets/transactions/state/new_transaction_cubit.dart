import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosy_pine/features/core/domain/entities/store.dart';
import 'package:frosy_pine/features/core/domain/use_cases/retrieve_available_stores_use_case.dart';
import 'package:intl/intl.dart';
import 'package:rust/rust.dart';

class NewTransactionCubit extends Cubit<NewTransactionCubitState> {
  NewTransactionCubit({required DateFormat dateFormat, required RetrieveAvailableStoresUseCase retrieveAvailableStoresUseCase})
    : _dateFormat = dateFormat,
      _retrieveAvailableStoresUseCase = retrieveAvailableStoresUseCase,
      super(const NewTransactionCubitState(loading: true, initialized: false, date: '', selectedStoreValue: null, stores: []));

  final DateFormat _dateFormat;
  final RetrieveAvailableStoresUseCase _retrieveAvailableStoresUseCase;

  Future<void> loadStores() async => (await _retrieveAvailableStoresUseCase.execute())
      .map<List<StoreDisplayModel>>((List<Store> stores) => stores.map<StoreDisplayModel>(StoreDisplayModel.fromStoreEntity).toList())
      .inspect((List<StoreDisplayModel> models) => emit(state.copyWith(loading: false, initialized: true, stores: models)));

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
  });

  final bool loading;
  final bool initialized;
  final String? selectedStoreValue;
  final String? date;
  final List<StoreDisplayModel> stores;

  @override
  List<Object?> get props => [loading, initialized, selectedStoreValue, date, stores];

  NewTransactionCubitState copyWith({
    bool? loading,
    bool? initialized,
    Option<String?>? selectedStoreValue,
    Option<String?>? date,
    List<StoreDisplayModel>? stores,
  }) => NewTransactionCubitState(
    loading: loading ?? this.loading,
    initialized: initialized ?? this.initialized,
    selectedStoreValue: selectedStoreValue?.toNullable() ?? this.selectedStoreValue,
    date: date?.toNullable() ?? this.date,
    stores: stores ?? this.stores,
  );
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
}
