use xvc_core::XvcCachePath;

use super::XvcStoragePath;

pub fn build_remote_path(
    remote_prefix: &str,
    repo_guid: &str,
    cache_path: &XvcCachePath,
) -> XvcStoragePath {
    XvcStoragePath::from(format!("{}/{}/{}", remote_prefix, repo_guid, cache_path))
}
