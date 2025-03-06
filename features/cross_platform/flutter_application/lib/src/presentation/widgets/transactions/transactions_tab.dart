import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/utils/context_extensions.dart';
import 'package:flutter_application/src/presentation/widgets/transactions/state/new_transaction_cubit.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class TransactionsTab extends StatelessWidget {
  const TransactionsTab({required this.newTransactionCubit, super.key});

  static const String i18nPrefix = 'transactions_tab';
  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const TransactionListDisplay(transactions: []),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [AddNewTransactionButton(newTransactionCubit: newTransactionCubit)],
        ),
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
      onPressed:
          () => showDialog<void>(
            context: context,
            builder: (context) => NewTransactionDialog(newTransactionCubit: newTransactionCubit),
          ),
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
        itemBuilder: (_, index) => Card(child: ListTile(title: Text(transactions[index]))),
        separatorBuilder: (_, _) => const SizedBox(height: 10),
      ),
    );
  }
}

class NewTransactionDialog extends StatelessWidget {
  NewTransactionDialog({required this.newTransactionCubit, super.key});

  static const String i18nPrefix = '${TransactionsTab.i18nPrefix}.new_transaction_dialog';

  final NewTransactionCubit newTransactionCubit;

  final TextEditingController storeController = TextEditingController();
  final TextEditingController dateController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(context.i18n('$i18nPrefix.title')),
      content: BlocBuilder<NewTransactionCubit, NewTransactionCubitState>(
        bloc: newTransactionCubit,
        builder: (BuildContext context, NewTransactionCubitState state) {
          storeController.text = state.storeName;
          dateController.text = state.date;

          return Column(
            children: [
              TextField(
                controller: storeController,
                decoration: InputDecoration(labelText: context.i18n('$i18nPrefix.store_field')),
              ),
              TextField(
                controller: dateController,
                onTap:
                    () => showDatePicker(context: context, firstDate: DateTime(1999), lastDate: DateTime.now()).then(
                      (date) => date ?? newTransactionCubit.setDate(date!),
                      onError: (dynamic error) => print(error),
                    ),
                readOnly: true,
                decoration: InputDecoration(labelText: context.i18n('$i18nPrefix.date_field')),
              ),
            ],
          );
        },
      ),
      actions: <Widget>[
        TextButton(
          child: Text(context.i18n('$i18nPrefix.cancel_button')),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        ElevatedButton(
          child: Text(context.i18n('$i18nPrefix.save_button')),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
      ],
    );
  }
}
