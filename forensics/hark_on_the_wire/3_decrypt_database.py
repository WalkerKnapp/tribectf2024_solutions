import sqlcipher3

conn = sqlcipher3.connect("output/wm_encrypted_db.db")
cursor = conn.cursor()

# Authenticate the database with the key from the conversation
cursor.execute("PRAGMA key = 'TheSecretOfTheCrimDell'")

# Find all tables
database_tables = []
cursor.execute("SELECT name FROM sqlite_master WHERE type='table';")
for (name,) in cursor.fetchall():
    database_tables.append(name)
    print("Table found", name)

# Print the contents of each table
for table in database_tables:
    cursor.execute(f"SELECT * FROM {table}")
    print("Contents of table", table)
    for row in cursor.fetchall():
        print(row)

