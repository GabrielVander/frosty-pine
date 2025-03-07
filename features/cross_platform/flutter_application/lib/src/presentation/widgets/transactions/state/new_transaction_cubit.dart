import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/glue/api/store.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class NewTransactionCubit extends Cubit<NewTransactionCubitState> {
  NewTransactionCubit() : super(const NewTransactionCubitState(loading: true, date: '', storeName: '', stores: [])) {
    retrieveAvailableStores().then((stores) => emit(state.copyWith(loading: false, stores: stores)));
  }

  void setDate(DateTime value) {
    emit(state.copyWith(date: value.toString()));
  }
}

@immutable
final class NewTransactionCubitState extends Equatable {
  const NewTransactionCubitState({
    required this.loading,
    required this.storeName,
    required this.date,
    required this.stores,
  });

  final bool loading;
  final String storeName;
  final String date;
  final List<StoreModel> stores;

  @override
  List<Object?> get props => [loading, storeName, date, stores];

  NewTransactionCubitState copyWith({bool? loading, String? storeName, String? date, List<StoreModel>? stores}) =>
      NewTransactionCubitState(
        loading: loading ?? this.loading,
        storeName: storeName ?? this.storeName,
        date: date ?? this.date,
        stores: stores ?? this.stores,
      );
}
