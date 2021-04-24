import 'dart:async';
import 'dart:core';
import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:sqflite/sqflite.dart';

void main() async {
  runApp(MyApp());
}

class Dictionary {
  late final int id;
  late final String word;
  late final String meaning;
  Dictionary({required this.id, required this.word, required this.meaning});

  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'word': word,
      'meaning': meaning,
    };
  }
}

Future<Database> database() async {
  Directory documentsDirectory = await getApplicationDocumentsDirectory();
  String path = join(documentsDirectory.path, 'asset_dictionary.db');

  if (FileSystemEntity.typeSync(path) == FileSystemEntityType.notFound) {
    ByteData data = await rootBundle.load(join('assets', 'dictionary.db'));
    List<int> bytes =
        data.buffer.asUint8List(data.offsetInBytes, data.lengthInBytes);
    await new File(path).writeAsBytes(bytes);
  }

  Directory appDocDir = await getApplicationDocumentsDirectory();
  String databasePath = join(appDocDir.path, 'asset_dictionary.db');

  return openDatabase(databasePath);
}

Future<void> insertMeaning(Dictionary dictionary) async {
  final Database db = await database();

  await db.insert(
    'dict',
    dictionary.toMap(),
    conflictAlgorithm: ConflictAlgorithm.ignore,
  );
}

Future<List<Dictionary>> getMeaning(String word) async {
  final Database db = await database();

  final List<Map<String, dynamic>> maps = await db.query(
    'dict',
    where: 'word = ?',
    whereArgs: [word],
  );

  return List.generate(maps.length, (i) {
    return Dictionary(
      id: maps[i]['id'],
      word: maps[i]['word'],
      meaning: maps[i]['meaning'],
    );
  });
}

class MyApp extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Dictionary Intent',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // Try running your application with "flutter run". You'll see the
        // application has a blue toolbar. Then, without quitting the app, try
        // changing the primarySwatch below to Colors.green and then invoke
        // "hot reload" (press "r" in the console where you ran "flutter run",
        // or simply save your changes to "hot reload" in a Flutter IDE).
        // Notice that the counter didn't reset back to zero; the application
        // is not restarted.
        primarySwatch: Colors.green,
      ),
      home: Scaffold(
        appBar: AppBar(
          title: Text("Dictionary Intent"),
        ),
        body: Center(
          child: HomePage(),
        ),
      ),
    );
  }
}

class HomePage extends StatefulWidget {
  @override
  HomePageState createState() {
    return HomePageState();
  }
}

class HomePageState extends State<HomePage> {
  final _formKey = GlobalKey<FormState>();
  final _myFormController = TextEditingController();

  @override
  void dispose() {
    _myFormController.dispose();
    super.dispose();
  }

  late List<Dictionary> wordMeaning = [];

  Future<void> _getMeaning() async {
    List<Dictionary> _wordMeaning = await getMeaning(_myFormController.text);
    setState(() {
      wordMeaning = _wordMeaning;
    });
  }

  _showMeaning(BuildContext context) async {
    await _getMeaning();
    String meaning = "";
    if (this.wordMeaning.length == 0) {
      meaning = "Not Found";
    } else {
      meaning = this.wordMeaning[0].meaning;
    }
    return showDialog(
        context: context,
        barrierDismissible: true,
        builder: (BuildContext context) {
          return CupertinoAlertDialog(
            title: Text("${_myFormController.text}"),
            content: Text("$meaning"),
            actions: <Widget>[
              CupertinoDialogAction(
                child: Text("Ok"),
                onPressed: () {
                  Navigator.of(context).pop();
                },
              )
            ],
          );
        });
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: <Widget>[
          ClipRRect(
            child: TextFormField(
                controller: _myFormController,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter some text';
                  } else if (value.contains(' ')) {
                    return 'Only Word lookup is possible';
                  }
                  return null;
                }),
          ),
          Padding(
            padding: EdgeInsets.symmetric(vertical: 16.0),
            child: ElevatedButton(
              onPressed: () {
                if (_formKey.currentState!.validate()) {
                  _showMeaning(context);
                }
              },
              child: Text('Submit'),
            ),
          )
        ],
      ),
    );
  }
}
