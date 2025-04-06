import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosy_pine/src/rust/api/brand_controller.dart';
import 'package:rust/rust.dart';

class BrandCubit extends Cubit<BrandCubitState> {
  BrandCubit({required this.brandController}) : super(BrandCubitState(newBrandName: null, brands: List<String>.empty()));

  final BrandController brandController;

  void onNewBrandNameChange(String? value) {
    emit(state.copyWith(newBrandName: Some(value)));
  }

  Future<void> createBrand() async {
    await brandController.addNewBrand(name: state.newBrandName ?? '');
  }
}

final class BrandCubitState extends Equatable {
  const BrandCubitState({required this.newBrandName, required this.brands});

  final String? newBrandName;
  final List<String> brands;

  BrandCubitState copyWith({Option<String?>? newBrandName, List<String>? brands}) =>
      BrandCubitState(brands: brands ?? this.brands, newBrandName: newBrandName?.toNullable() ?? this.newBrandName);

  @override
  String toString() => 'BrandCubitState(brands: $brands, newBrandName: $newBrandName)';

  @override
  List<Object?> get props => [newBrandName, brands];
}
