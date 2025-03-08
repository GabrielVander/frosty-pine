import 'package:frosy_pine/features/core/domain/entities/brand.dart';
import 'package:frosy_pine/features/core/utilities/failures.dart';
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
