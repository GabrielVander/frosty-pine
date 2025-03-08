import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosy_pine/features/ui/presentation/utils/context_extensions.dart';
import 'package:frosy_pine/features/ui/presentation/widgets/transactions/state/new_transaction_cubit.dart';

class TransactionsTab extends StatelessWidget {
  const TransactionsTab({required this.newTransactionCubit, super.key});

  static const String i18nPrefix = 'transactions_tab';
  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const TransactionListDisplay(transactions: []),
        Row(mainAxisAlignment: MainAxisAlignment.center, children: [AddNewTransactionButton(newTransactionCubit: newTransactionCubit)]),
      ],
    );
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
      shape: const RoundedRectangleBorder(borderRadius: BorderRadius.all(Radius.circular(20))),
      title: Text(context.i18n('$i18nPrefix.title')),
      content: BlocBuilder<NewTransactionCubit, NewTransactionCubitState>(
        bloc: newTransactionCubit,
        builder: (BuildContext context, NewTransactionCubitState state) {
          if (!state.initialized) {
            newTransactionCubit.loadStores();
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
                    () =>
                        showDatePicker(context: context, firstDate: DateTime(1999), lastDate: DateTime.now()).then(newTransactionCubit.setDate, onError: print),
                readOnly: true,
                decoration: InputDecoration(labelText: context.i18n('$i18nPrefix.date_field')),
              ),
            ],
          );
        },
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
