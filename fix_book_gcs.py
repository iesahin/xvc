import re

with open('book/src/ref/xvc-storage-new-gcs.md', 'r') as f:
    content = f.read()

content = re.sub(
    r'Reads credentials from `GCS_ACCESS_KEY_ID` and `GCS_SECRET_ACCESS_KEY` environment variables\. Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type\.',
    r'Uses Google Application Default Credentials (ADC). You can authenticate by running `gcloud auth application-default login` or by setting the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to the path of a service account JSON key file.',
    content
)

content = re.sub(
    r'Please \[configure S3 compatible interface to your Google Cloud Storage account\].*?before using this command\.',
    r'You can configure this using Google Cloud Storage SDK and Application Default Credentials natively.',
    content,
    flags=re.DOTALL
)

content = re.sub(
    r'Before calling any commands that use this storage, you must set the following environment variables\.\n\n- `GCS_ACCESS_KEY_ID`.*?\n- `GCS_SECRET_ACCESS_KEY`.*?\n',
    r'Before calling any commands that use this storage, make sure you have either run `gcloud auth application-default login` or set the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to a valid service account JSON key.\n',
    content,
    flags=re.DOTALL
)

with open('book/src/ref/xvc-storage-new-gcs.md', 'w') as f:
    f.write(content)
