import 'package:frosy_pine/features/core/domain/entities/store.dart';
import 'package:frosy_pine/features/core/utilities/failures.dart';
import 'package:rust/rust.dart';

abstract interface class StoreRepository {
  Future<Result<List<Store>, StoreRepositoryFailure>> retrieveAllStores();
}

sealed class StoreRepositoryFailure extends Failure {
  const StoreRepositoryFailure({
    required super.message,
    required super.details,
  });
}

final class StoreRepositoryUnexpectedFailure extends StoreRepositoryFailure {
  const StoreRepositoryUnexpectedFailure({
    required super.message,
    required super.details,
  });
}
