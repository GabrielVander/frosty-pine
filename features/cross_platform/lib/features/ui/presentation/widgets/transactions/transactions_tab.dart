import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosty_pine/features/ui/presentation/utils/context_extensions.dart';
import 'package:frosty_pine/features/ui/presentation/widgets/transactions/state/new_transaction_cubit.dart';

class TransactionsTab extends StatelessWidget {
  const TransactionsTab({required this.newTransactionCubit, super.key});

  static const String i18nPrefix = 'transactions_tab';
  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    return Column(children: [const TransactionListDisplay(transactions: []), AddNewTransactionButton(newTransactionCubit: newTransactionCubit)]);
  }
}

class AddNewTransactionButton extends StatelessWidget {
  const AddNewTransactionButton({required this.newTransactionCubit, super.key});

  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: () => showDialog<void>(context: context, builder: (BuildContext context) => NewTransactionDialog(newTransactionCubit: newTransactionCubit)),
      icon: const Icon(Icons.add),
      iconSize: 60,
      color: Theme.of(context).primaryColor,
    );
  }
}

class TransactionListDisplay extends StatelessWidget {
  const TransactionListDisplay({required this.transactions, super.key});

  static const String i18nPrefix = '${TransactionsTab.i18nPrefix}.transaction_list_display';
  final List<String> transactions;

  @override
  Widget build(BuildContext context) {
    if (transactions.isEmpty) {
      return Expanded(child: Center(child: Text(context.i18n('$i18nPrefix.empty_list'))));
    }

    return Expanded(
      child: ListView.separated(
        itemCount: transactions.length,
        itemBuilder: (BuildContext _, int index) => Card(child: ListTile(title: Text(transactions[index]))),
        separatorBuilder: (BuildContext _, int _) => const SizedBox(height: 10),
      ),
    );
  }
}

class NewTransactionDialog extends StatelessWidget {
  const NewTransactionDialog({required this.newTransactionCubit, super.key});

  static const String i18nPrefix = '${TransactionsTab.i18nPrefix}.new_transaction_dialog';

  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(context.i18n('$i18nPrefix.title')),
      shape: const RoundedRectangleBorder(borderRadius: BorderRadius.all(Radius.circular(20))),
      content: SizedBox(
        width: MediaQuery.of(context).size.width,
        child: BlocBuilder<NewTransactionCubit, NewTransactionCubitState>(
          bloc: newTransactionCubit,
          builder: (BuildContext context, NewTransactionCubitState state) {
            if (!state.initialized) {
              newTransactionCubit
                ..loadStores()
                ..loadProducts();
            }

            if (state.loading) {
              return const Center(child: CircularProgressIndicator());
            }

            return Column(
              children: [
                DropdownButton<String>(
                  onChanged: newTransactionCubit.selectStore,
                  hint: Text(context.i18n('$i18nPrefix.store_dropdown_hint')),
                  value: state.selectedStoreValue,
                  items:
                      state.stores
                          .map<DropdownMenuItem<String>>((StoreDisplayModel s) => DropdownMenuItem<String>(value: s.value, child: Text(s.displayName)))
                          .toList(),
                  isExpanded: true,
                ),
                TextField(
                  controller: TextEditingController(text: state.date),
                  onTap:
                      () => showDatePicker(
                        context: context,
                        firstDate: DateTime(1999),
                        lastDate: DateTime.now(),
                      ).then(newTransactionCubit.setDate, onError: print),
                  readOnly: true,
                  decoration: InputDecoration(labelText: context.i18n('$i18nPrefix.date_field')),
                ),
                const Divider(),
                ItemsListDisplay(newTransactionCubit: newTransactionCubit, state: state),
              ],
            );
          },
        ),
      ),
      actions: [
        TextButton(
          child: Text(context.i18n('$i18nPrefix.cancel_button')),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        FilledButton(
          child: Text(context.i18n('$i18nPrefix.save_button')),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
      ],
    );
  }
}

class ItemsListDisplay extends StatelessWidget {
  const ItemsListDisplay({required this.state, required this.newTransactionCubit, super.key});

  final NewTransactionCubitState state;
  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    final Iterable<Widget> items = state.items.map((i) => toItemRow(context, i));

    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      spacing: 15,
      children: [
        if (items.isEmpty) Text(context.i18n('transactions_tab.new_transaction_dialog.items.no_items')) else ...items,
        if (state.showAddItemButton)
          TextButton(onPressed: newTransactionCubit.showNewItem, child: Text(context.i18n('transactions_tab.new_transaction_dialog.items.add_item_button'))),
      ],
    );
  }

  Widget toItemRow(BuildContext context, TransactionItemDisplayModel i) => OverflowBar(
    key: ValueKey<String>(i.key),
    alignment: MainAxisAlignment.center,
    overflowSpacing: 10,
    overflowAlignment: OverflowBarAlignment.center,
    spacing: 10,
    children: [
      ProductsDropDown(products: state.products, value: i.productValue, onChanged: (String? value) => newTransactionCubit.setProductValue(i.key, value)),
      UnitsDropDown(units: state.units, value: i.unitValue, onChanged: (unit) => newTransactionCubit.setUnitValue(i.key, unit)),
      if (i.unitValue?.requiresNumericalValue ?? false)
        TextField(
          decoration: InputDecoration(label: Text(context.i18n('transactions_tab.new_transaction_dialog.items.unit_amount_label'))),
          inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(8)],
          textInputAction: TextInputAction.done,
          keyboardType: TextInputType.numberWithOptions(decimal: i.unitValue?.hasDecimalPlaces ?? false),
          onChanged: (value) => newTransactionCubit.setUnitAmount(i.key, value, Localizations.localeOf(context)),
          controller:
              i.unitAmount != null
                  ? TextEditingController.fromValue(
                    TextEditingValue(text: i.unitAmount!, selection: TextSelection.fromPosition(TextPosition(offset: i.unitAmount!.length))),
                  )
                  : null,
        ),
      TextField(
        decoration: InputDecoration(label: Text(context.i18n('transactions_tab.new_transaction_dialog.items.unitary_price_label'))),
        inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(8)],
        textInputAction: TextInputAction.done,
        keyboardType: const TextInputType.numberWithOptions(decimal: true),
        onChanged: (value) => newTransactionCubit.setUnitaryPrice(i.key, value, Localizations.localeOf(context)),
        controller:
            i.unitaryPrice != null
                ? TextEditingController.fromValue(
                  TextEditingValue(text: i.unitaryPrice!, selection: TextSelection.fromPosition(TextPosition(offset: i.unitaryPrice!.length))),
                )
                : null,
      ),
    ],
  );
}

class ProductsDropDown extends StatelessWidget {
  const ProductsDropDown({required this.products, required this.onChanged, super.key, this.value});

  final List<ProductDisplayModel> products;
  final String? value;
  final void Function(String? value) onChanged;

  @override
  Widget build(BuildContext context) {
    return DropdownButton<String>(
      onChanged: onChanged,
      hint: Text(context.i18n('transactions_tab.new_transaction_dialog.items.product_dropdown_hint')),
      value: value,
      items: products.map<DropdownMenuItem<String>>((ProductDisplayModel s) => DropdownMenuItem<String>(value: s.value, child: Text(s.displayName))).toList(),
    );
  }
}

class UnitsDropDown extends StatelessWidget {
  const UnitsDropDown({required this.units, required this.onChanged, super.key, this.value});

  final List<UnitDisplayModel> units;
  final UnitDisplayModel? value;
  final void Function(UnitDisplayModel? value) onChanged;

  @override
  Widget build(BuildContext context) {
    return DropdownButton<UnitDisplayModel>(
      onChanged: onChanged,
      hint: Text(context.i18n('transactions_tab.new_transaction_dialog.items.unit_dropdown_hint')),
      value: value,
      items:
          units
              .map<DropdownMenuItem<UnitDisplayModel>>(
                (UnitDisplayModel s) => DropdownMenuItem<UnitDisplayModel>(
                  value: s,
                  child: Text(context.i18n('transactions_tab.new_transaction_dialog.items.units.${s.i18nIdentifier}')),
                ),
              )
              .toList(),
    );
  }
}
