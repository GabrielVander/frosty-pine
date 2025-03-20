import 'package:frosy_pine/features/core/domain/entities/category.dart';
import 'package:frosy_pine/features/core/utilities/failures.dart';
import 'package:rust/rust.dart';

abstract interface class CategoryRepository {
  Future<Result<Category, CategoryRepositoryFailure>> fetchSingleById(
    String id,
  );
}

sealed class CategoryRepositoryFailure extends Failure {
  const CategoryRepositoryFailure({
    required super.message,
    required super.details,
  });
}

final class CategoryRepositoryNotFoundFailure
    extends CategoryRepositoryFailure {
  const CategoryRepositoryNotFoundFailure({
    required super.message,
    required super.details,
  });
}
