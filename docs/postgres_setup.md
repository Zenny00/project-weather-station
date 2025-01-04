# Setting up Postgres on the Raspberry Pi 4

1. Run `sudo apt update` to get the latest packages
2. Run `sudo apt install postgresql postgresql-contrib`
3. Check the status of the Postgres process `systemctl status postgresql`
4. Start the Postgres process `sudo systemctl start postgresql.service`
5. Run `ps -ef | grep -i postgres` to ensure the process has started
6. Run `sudo su postgres` to run the process
7. Create a new user using `createuser {username} -P --interactive` (Note: The username should be the same as the system user for peer authentication)
8. Exit Postgres
9. Run `psql -d postgres -U {username} -W {password}` to connect to the database server
10. Create a new database using `CREATE DATABASE {database_name};`
11. Run `connect {database_name};` to connect to the database
12. Create a new table in the database using `CREATE TABLE {table_name} (attribute type, attribute type);`
13. Insert new data into the table using `INSERT INTO {table_name} (attribute, attribute) VALUES (value, value);`
14. List the data in the table using `SELECT * FROM {table_name};`

### Example Table:
![Simple Postgres Database with Table](/running_postgres-table.PNG)

### References:
- https://www.youtube.com/watch?v=DZlxuf2kzEU
- https://www.digitalocean.com/community/tutorials/how-to-install-postgresql-on-ubuntu-20-04-quickstart