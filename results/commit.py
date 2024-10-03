import os
import requests
import json

# Server URL
SERVER_URL = (
    "http://localhost:9009"  # Modify this according to your actual server address
)


# Read CSV files from the results folder
def read_csv_files(folder_path):
    for filename in os.listdir(folder_path):
        if filename.endswith(".csv"):
            file_path = os.path.join(folder_path, filename)
            yield filename, file_path


# Extract CPU name and committer from filename
def parse_filename(filename):
    parts = filename.split(" by-")
    cpu_name = parts[0]
    commit_by = parts[1].split(".csv")[0]
    return cpu_name, commit_by


# Read CSV file content
def read_file_content(file_path):
    encodings = ["utf-8", "iso-8859-1", "windows-1252"]
    for encoding in encodings:
        try:
            with open(file_path, "r", encoding=encoding) as file:
                return file.read()
        except UnicodeDecodeError:
            continue
    return None  # Return None if all encodings fail


# Send request to commit endpoint
def send_commit_request(data):
    response = requests.post(f"{SERVER_URL}/commit", json=data)
    return response


# Main function
def main():
    results_folder = "."  # Folder containing CSV files

    for filename, file_path in read_csv_files(results_folder):
        try:
            cpu_name, commit_by = parse_filename(filename)
            file_content = read_file_content(file_path)

            if file_content is None:
                print(f"Error: Unable to read file with known encodings: {filename}")
                print("---")
                continue

            request_body = {
                "cpu_name": cpu_name,
                "commit_by": commit_by,
                "file": file_content,
            }

            print(f"Submitting: {filename}")
            response = send_commit_request(request_body)

            if response.status_code == 200:
                print(f"Submission successful: {filename}")
                print(f"Server response: {response.json()}")
            else:
                print(f"Submission failed: {filename}")
                print(f"Error message: {response.text}")

        except Exception as e:
            print(f"Error processing {filename}: {str(e)}")

        print("---")


if __name__ == "__main__":
    main()
