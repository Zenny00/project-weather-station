from micropg_lite import connect # https://github.com/TimonW-Dev/micropg_lite

def open_db_connection(file_path):
    connection = None
    try:
        db_credentials = open(file_path, "r")
        ip, username, password = [line.strip() for line in db_credentials.readlines()]
        connection = connect(host=ip, user=username, password=password, database='weather_data')
    except OSError as error:
        print(f"Could not read database credential file {error}")
        
    return connection
    