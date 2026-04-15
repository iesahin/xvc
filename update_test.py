import re

with open("lib/tests/test_storage_new_gcs.rs", "r") as f:
    content = f.read()

content = content.replace(
    'let gsutil = |cmd: &str, append: &str| -> String {\n        let sh_cmd = format!("gsutil {cmd} {append}");\n        sh(sh_cmd)\n    };',
    'let gcloud_storage = |cmd: &str, append: &str| -> String {\n        let sh_cmd = format!("gcloud storage {cmd} {append}");\n        sh(sh_cmd)\n    };'
)

content = content.replace("gsutil(", "gcloud_storage(")

with open("lib/tests/test_storage_new_gcs.rs", "w") as f:
    f.write(content)

