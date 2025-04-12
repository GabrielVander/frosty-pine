import 'package:frosty_pine/domain/entities/item.dart';
import 'package:frosty_pine/domain/entities/store.dart';

final class Transaction {
  Transaction({required this.id, required this.dateInUtc, required this.store, required this.items});

  final String id;
  final DateTime dateInUtc;
  final Store store;
  final List<Item> items;

  double calculateTotal() {
    return items.map<double>((Item i) => i.calculateFullPrice()).reduce((double value, double element) => value + element);
  }
}
