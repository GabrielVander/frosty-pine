import 'package:frosty_pine/domain/entities/failures.dart';
import 'package:frosty_pine/domain/entities/store.dart';
import 'package:rust/rust.dart';

abstract interface class StoreRepository {
  Future<Result<List<Store>, StoreRepositoryFailure>> retrieveAllStores();
}

sealed class StoreRepositoryFailure extends Failure {
  const StoreRepositoryFailure({required super.message, required super.details});
}

final class StoreRepositoryUnexpectedFailure extends StoreRepositoryFailure {
  const StoreRepositoryUnexpectedFailure({required super.message, required super.details});
}
