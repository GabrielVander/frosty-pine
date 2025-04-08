import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosy_pine/src/rust/api/brand_controller.dart';
import 'package:rust/rust.dart';

class BrandCubit extends Cubit<BrandCubitState> {
  BrandCubit({required this.addNewBrand, required this.retrieveAllBrands}) : super(BrandCubitState(newBrandName: null, brands: List<String>.empty()));

  final Future<BrandModel> Function({required String name}) addNewBrand;
  final Future<List<BrandModel>> Function() retrieveAllBrands;

  void onNewBrandNameChange(String? value) {
    emit(state.copyWith(newBrandName: Some(value)));
  }

  Future<void> createBrand() async {
    await addNewBrand(name: state.newBrandName ?? '');
    await updateBrands();
  }

  Future<void> updateBrands() async {
    final List<BrandModel> brands = await retrieveAllBrands();
    emit(state.copyWith(brands: brands.map((b) => b.name).toList()));
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
