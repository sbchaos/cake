use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageMetadata {
    pub last_tag_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootFS {
    pub r#type: String,
    pub layers: Vec<String>,
    pub base_layer: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspect {
    #[serde(rename = "Id")]
    pub id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests: Vec<String>,
    pub parent: String,
    pub comment: String,
    pub created: String,
    pub container: String,
    // containerConfig: *container.Config,
    pub docker_version: String,
    pub author: String,
    // config         : *container.Config,
    pub architecture: String,
    pub os: String,
    pub os_version: Option<String>,
    pub size: i64,
    pub virtual_size: i64,
    // graphDriver    : GraphDriverData,
    #[serde(rename = "RootFS")]
    pub root_fs: RootFS,
    pub metadata: ImageMetadata,
}

#[cfg(test)]
mod tests {
    use crate::image::inspect::ImageInspect;

    #[test]
    fn deserialize_cli_data() {
        let response_str = "[\n    {\n        \"Id\": \"sha256:6dbb9cc54074106d46d4ccb330f2a40a682d49dda5f4844962b7dce9fe44aaec\",\n        \"RepoTags\": [\n            \"alpine:3\"\n        ],\n        \"RepoDigests\": [\n            \"alpine@sha256:69e70a79f2d41ab5d637de98c1e0b055206ba40a8145e7bddb55ccc04e13cf8f\"\n        ],\n        \"Parent\": \"\",\n        \"Comment\": \"\",\n        \"Created\": \"2021-04-14T19:19:39.643236135Z\",\n        \"Container\": \"60a3cdd128a8b373b313ed3e1083ff45e6badaad5dca5187282b005c38d04712\",\n        \"ContainerConfig\": {\n            \"Hostname\": \"60a3cdd128a8\",\n            \"Domainname\": \"\",\n            \"User\": \"\",\n            \"AttachStdin\": false,\n            \"AttachStdout\": false,\n            \"AttachStderr\": false,\n            \"Tty\": false,\n            \"OpenStdin\": false,\n            \"StdinOnce\": false,\n            \"Env\": [\n                \"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\"\n            ],\n            \"Cmd\": [\n                \"/bin/sh\",\n                \"-c\",\n                \"#(nop) \",\n                \"CMD [\\\"/bin/sh\\\"]\"\n            ],\n            \"Image\": \"sha256:d3d4554f8b07cf59894bfb3551e10f89a559b24ee0992c4900c54175596b1389\",\n            \"Volumes\": null,\n            \"WorkingDir\": \"\",\n            \"Entrypoint\": null,\n            \"OnBuild\": null,\n            \"Labels\": {}\n        },\n        \"DockerVersion\": \"19.03.12\",\n        \"Author\": \"\",\n        \"Config\": {\n            \"Hostname\": \"\",\n            \"Domainname\": \"\",\n            \"User\": \"\",\n            \"AttachStdin\": false,\n            \"AttachStdout\": false,\n            \"AttachStderr\": false,\n            \"Tty\": false,\n            \"OpenStdin\": false,\n            \"StdinOnce\": false,\n            \"Env\": [\n                \"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\"\n            ],\n            \"Cmd\": [\n                \"/bin/sh\"\n            ],\n            \"Image\": \"sha256:d3d4554f8b07cf59894bfb3551e10f89a559b24ee0992c4900c54175596b1389\",\n            \"Volumes\": null,\n            \"WorkingDir\": \"\",\n            \"Entrypoint\": null,\n            \"OnBuild\": null,\n            \"Labels\": null\n        },\n        \"Architecture\": \"amd64\",\n        \"Os\": \"linux\",\n        \"Size\": 5613158,\n        \"VirtualSize\": 5613158,\n        \"GraphDriver\": {\n            \"Data\": {\n                \"MergedDir\": \"/var/lib/docker/overlay2/9a02dfc08f8c53b31077516595993354a9481dec1d13dc9390c101d866d268b6/merged\",\n                \"UpperDir\": \"/var/lib/docker/overlay2/9a02dfc08f8c53b31077516595993354a9481dec1d13dc9390c101d866d268b6/diff\",\n                \"WorkDir\": \"/var/lib/docker/overlay2/9a02dfc08f8c53b31077516595993354a9481dec1d13dc9390c101d866d268b6/work\"\n            },\n            \"Name\": \"overlay2\"\n        },\n        \"RootFS\": {\n            \"Type\": \"layers\",\n            \"Layers\": [\n                \"sha256:b2d5eeeaba3a22b9b8aa97261957974a6bd65274ebd43e1d81d0a7b8b752b116\"\n            ]\n        },\n        \"Metadata\": {\n            \"LastTagTime\": \"0001-01-01T00:00:00Z\"\n        }\n    }\n]\n";

        let inspect: Vec<ImageInspect> = serde_json::from_str(&response_str).unwrap();
        assert_eq!(inspect.get(0).unwrap().os, "linux");
        assert_eq!(inspect.get(0).unwrap().root_fs.r#type, "layers");
        assert_eq!(
            inspect.get(0).unwrap().repo_tags.get(0).unwrap(),
            "alpine:3"
        );
    }
}
