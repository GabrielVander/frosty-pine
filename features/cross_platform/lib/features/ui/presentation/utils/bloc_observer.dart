import 'package:flutter_bloc/flutter_bloc.dart';

class SimpleBlocObserver extends BlocObserver {
  @override
  void onChange(BlocBase<dynamic> bloc, Change<dynamic> change) {
    super.onChange(bloc, change);
    print(
      '\n\n[${bloc.runtimeType}]\n\nPrevious state:\n${change.currentState}\nNew state:\n${change.nextState}\n',
    );
  }
}
