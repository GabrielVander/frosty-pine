import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/utils/context_extensions.dart';

class TransactionsTab extends StatelessWidget {
  const TransactionsTab({super.key});

  static const String i18nPrefix = 'transactions_tab';

  @override
  Widget build(BuildContext context) {
    return const Column(
      children: [
        TransactionListDisplay(transactions: []),
        Row(mainAxisAlignment: MainAxisAlignment.center, children: [AddNewTransactionButton()]),
      ],
    );
  }
}

class AddNewTransactionButton extends StatelessWidget {
  const AddNewTransactionButton({super.key});

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: () => showDialog<void>(context: context, builder: (context) => const NewTransactionDialog()),
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
  const NewTransactionDialog({super.key});

  static const String i18nPrefix = '${TransactionsTab.i18nPrefix}.new_transaction_dialog';

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(context.i18n('$i18nPrefix.title')),
      content: Container(),
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
