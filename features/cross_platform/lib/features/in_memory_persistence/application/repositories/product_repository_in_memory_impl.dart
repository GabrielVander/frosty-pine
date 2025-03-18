import 'package:frosy_pine/features/core/domain/entities/product.dart';
import 'package:frosy_pine/features/core/domain/repositories/product_repository.dart';
import 'package:rust/rust.dart';

class ProductRepositoryInMemoryImpl implements ProductRepository {
  ProductRepositoryInMemoryImpl({required this.data});

  final Map<String, ProductModel> data;

  @override
  Future<Result<List<LazyProduct>, ProductRepositoryFailure>>
  fetchAllLazy() async => Ok<List<LazyProduct>, ProductRepositoryFailure>(
    data.entries.map<LazyProduct>(_toLazyProduct).toList(),
  );

  LazyProduct _toLazyProduct(MapEntry<String, ProductModel> entry) =>
      LazyProduct(
        id: entry.key,
        name: entry.value.name,
        brandId: entry.value.brandId,
        categoryId: entry.value.categoryId,
      );
}

final class ProductModel {
  ProductModel({
    required this.name,
    required this.brandId,
    required this.categoryId,
  });

  final String name;
  final String brandId;
  final String categoryId;
}
