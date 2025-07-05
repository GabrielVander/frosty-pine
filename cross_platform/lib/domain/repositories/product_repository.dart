import 'package:frosty_pine/domain/entities/failures.dart';
import 'package:frosty_pine/domain/entities/product.dart';
import 'package:rust/rust.dart';

abstract interface class ProductRepository {
  Future<Result<List<LazyProduct>, ProductRepositoryFailure>> fetchAllLazy();
}

sealed class ProductRepositoryFailure extends Failure {
  const ProductRepositoryFailure({required super.message, required super.details});
}

final class ProductRepositoryUnexpectedFailure extends ProductRepositoryFailure {
  const ProductRepositoryUnexpectedFailure({required super.message, required super.details});
}
