import re

with open('storage/src/lib.rs', 'r') as f:
    content = f.read()

new_content = re.sub(
    r'/// Reads credentials from `GCS_ACCESS_KEY_ID` and `GCS_SECRET_ACCESS_KEY` environment variables\.\n    /// Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and\n    /// `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type\.',
    r'/// Uses Google Application Default Credentials (ADC).\n    /// You can authenticate by running `gcloud auth application-default login` or by setting the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to the path of a service account JSON key file.',
    content
)

with open('storage/src/lib.rs', 'w') as f:
    f.write(new_content)
