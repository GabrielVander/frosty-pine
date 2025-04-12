import 'package:frosty_pine/domain/entities/brand.dart';
import 'package:frosty_pine/domain/entities/failures.dart';
import 'package:rust/rust.dart';

abstract interface class BrandRepository {
  Future<Result<Brand, BrandRepositoryFailure>> fetchSingleById(String id);
}

sealed class BrandRepositoryFailure extends Failure {
  const BrandRepositoryFailure({required super.message, required super.details});
}

final class BrandRepositoryNotFoundFailure extends BrandRepositoryFailure {
  const BrandRepositoryNotFoundFailure({required super.message, required super.details});
}
