Aragog
===============================================================================

While looking for a database for my next rust project, I decide to give ArganoDB a try through
[Aragog] crate. [Aragog] is a "simple lightweight object-document mapper" for ArangoDB.

Setting Up [Arangodb]
-------------------------------------------------------------------------------

I've tried to use nix for installation and was planning to have `.envrc`, but sadly ArganoDB is not
yet support on neither darwin_x86 or darein_arm.

My Current workaround for macOs M1 is to install brew under x86_64 using `arch -x86_64 ...`.  then,

```bash 
/usr/local/Homebrew/bin/brew install arangodb
/usr/local/opt/arangodb/sbin/arangod --server.authentication=false
```

Creating [Arangodb] Database
-------------------------------------------------------------------------------

```bash
arangosh> db._createDatabase("playground_db");
arangosh> require("@arangodb/users").save("playground_user", "playground_password");
arangosh> require("@arangodb/users").grantDatabase("playground_user", "playground_db");
```


Afterward, to connect to the database using arangosh:
```bash
arangosh --server.username playground_user --server.database playground_db
```


[Arangodb] Concepts
-------------------------------------------------------------------------------

- Databases contains collection.
- collections contains documents or records and are the equivalent of tables in RDBMS.
- documents are the equivalent of tables in rows.


Resources
-------------------------------------------------------------------------------

Here are few helpful resources to browse to learn more about [Aragog] and [Arangodb]:

- [Aragog Official book]
- [ArangoDB Databases, collections and documents]

[Arangodb]: http://arangodb.com
[Aragog]: http://gitlab.com/qonfucius/aragog
[Aragog Official book]: https://aragog.rs/book/arangodb.html
[ArangoDB Graph Database Syntax]: https://www.youtube.com/watch?v=0U8TfLOp184
[ArangoDB Databases, collections and documents]: https://www.arangodb.com/docs/stable/getting-started-databases-collections-documents.html 
