import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/glue/frb_generated.dart';
import 'package:flutter_application/src/presentation/pages/main_page.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(home: MainPage());
  }
}
