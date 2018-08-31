// Copyright 2018 The Rio Advancement Inc

use api::base::ObjectReference;
use api::base::{MetaFields, ObjectMeta, Status, TypeMeta};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the job
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: BuildConfig
    object_meta: ObjectMeta, //Standard object metadata. owner_refered by <assemblyFactory_id> >
    #[serde(default)]
    meta_data: BTreeMap<String, String>,
    spec: SpecData, //Spec holds all the input necessary to produce a new build, and the conditions when  to trigger them.
    status: Status,
    #[serde(default)]
    created_at: String,
}

impl BuildConfig {
    pub fn new() -> BuildConfig {
        ::std::default::Default::default()
    }

    //Create a new build config with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> BuildConfig {
        BuildConfig {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_spec(&mut self, v: SpecData) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &SpecData {
        &self.spec
    }
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }
    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_meta_data(&mut self, v: BTreeMap<String, String>) {
        self.meta_data = v;
    }

    pub fn get_meta_data(&self) -> &BTreeMap<String, String> {
        &self.meta_data
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for BuildConfig {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildTriggerPolicy {
    trigger_type: String, //type of the build trigger
    #[serde(default)]
    webhook: WebHook, // WebHook contains the parameters for a webhook type of trigger
    #[serde(default)]
    image_change: ImageChange, // ImageChange contains parameters for an ImageChange type of trigger
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct WebHook {
    hook_type: String, //web hook type
    secret: String,    // Secret is the obfuscated webhook secret that triggered a build.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageChange {
    last_triggered_image_id: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildSource {
    #[serde(default)]
    binary: BinaryBuildSource, // Binary builds accept a binary as their input. The binary is generally assumed to be a tar, gzipped tar, or zip file depending on the strategy.
    #[serde(default)]
    docker_file: String, // Dockerfile is the raw contents of a Dockerfile which should be built
    #[serde(default)]
    git: GitBuildSource, // Git contains optional information about git build source
    #[serde(default)]
    source_secret: String, // SourceSecret is the name of a Secret that would be used for setting up the authentication for cloning private repository.
    images: Vec<ImageSource>, // Images describes a set of images to be used to provide source for the build
    #[serde(default)]
    context_dir: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BinaryBuildSource {
    as_file: String, // AsFile indicates that the provided binary input should be considered a single file within the build input.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct GitBuildSource {
    uri: String, // URI points to the source that will be built. The structure of the source will depend on the type of build to run
    reference: String, // Ref is the branch/tag/ref to build.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageSource {
    from: ObjectReference, // From is a reference to an ImageMark, ImageStreamImage, or DockerImage to copy source from.
    #[serde(default)]
    pull_secret: String, // PullSecret is a reference to a secret to be used to pull the image from a registry
    #[serde(default)]
    paths: Vec<ImageSourcePath>, // Paths is a list of source and destination paths to copy from the image.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageSourcePath {
    source_path: String, // SourcePath is the absolute path of the file or directory inside the image to copy to the build directory.
    destination_dir: String, // DestinationDir is the relative directory within the build directory where files copied from the image are placed.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SpecData {
    run_policy: String, // RunPolicy describes how the new build created from this build  configuration will be scheduled for execution. This is optional, if not specified we default to "Serial".
    #[serde(default)]
    build_trigger_policys: Vec<BuildTriggerPolicy>, // Triggers determine how new Builds can be launched from a BuildConfig. If  no triggers are defined, a new build can only occur as a result of an  explicit client build creation.
    source: BuildSource,              // Source describes the SCM in use.
    strategy: BuildStrategy,          // Strategy defines how to perform a build.
    output: BuildOutput, // Output describes the Docker image the Strategy should produce.
    post_commit: BuildPostCommitSpec, // PostCommit is a build hook executed after the build output image is committed, before it is pushed to a registry.
    #[serde(default)]
    node_selector: BTreeMap<String, String>, // NodeSelector is a selector which must be true for the build assembly to fit on a node
    #[serde(default)]
    last_version: i32, //LastVersion is used to inform about number of last triggered build.
    #[serde(default)]
    successful_builds_history_limit: i32, //SuccessfulBuildsHistoryLimit is the number of old successful builds to retain. This field is a pointer to allow for differentiation between an explicit zero and not specified.
    #[serde(default)]
    failed_builds_history_limit: i32, // FailedBuildsHistoryLimit is the number of old failed builds to retain.This field is a pointer to allow for differentiation between an explicit zero and not specified.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildStrategy {
    build_type: String, //type of the build strategy like docker
    #[serde(default)]
    source_strategy: SourceBuildStrategy,
    #[serde(default)]
    docker_strategy: DockerBuildStrategy,
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SourceBuildStrategy {
    from: ObjectReference, // From is reference to an DockerImage, ImageStream, ImageMark, or ImageStreamImage from which  the docker image should be pulled
    #[serde(default)]
    pull_secret: String, // PullSecret is the name of a Secret that would be used for setting up  the authentication for pulling the Docker images from the private Docker registries
    #[serde(default)]
    env: Vec<EnvVar>, // Env contains additional environment variables you want to pass into a builder container. ValueFrom is not supported.
    #[serde(default)]
    scripts: String, // Scripts is the location of Source scripts
    #[serde(default)]
    incremental: String, // Incremental flag forces the Source build to do incremental builds if true.
    #[serde(default)]
    force_pull: bool, // ForcePull describes if the controller should configure the build assembly to always pull the images for the builder or only pull if it is not present locally
    // RuntimeImage is an optional image that is used to run an application
    // without unneeded dependencies installed. The building of the application
    // is still done in the builder image but, post build, you can copy the
    // needed artifacts in the runtime image for use.
    // This field and the feature it enables are in tech preview.
    #[serde(default)]
    runtime_image: ObjectReference,
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DockerBuildStrategy {
    from: ObjectReference, // From is reference to an DockerImage, ImageStream, ImageMark, or ImageStreamImage from which  the docker image should be pulled
    #[serde(default)]
    pull_secret: String, // PullSecret is the name of a Secret that would be used for setting up  the authentication for pulling the Docker images from the private Docker registries
    #[serde(default)]
    env: Vec<EnvVar>, // Env contains additional environment variables you want to pass into a builder container. ValueFrom is not supported.
    #[serde(default)]
    force_pull: bool, // ForcePull describes if the controller should configure the build assembly to always pull the images for the builder or only pull if it is not present locally
    #[serde(default)]
    docker_filepath: String, // DockerfilePath is the path of the Dockerfile that will be used to build the Docker image, relative to the root of the context (contextDir).
    // ImageOptimizationPolicy describes what optimizations the system can use when building images
    // to reduce the final size or time spent building the image. The default policy is 'None' which
    // means the final build image will be equivalent to an image created by the Docker build API.
    // The experimental policy 'SkipLayerCache' will avoid commiting new layers in between each
    // image step, and will fail if the Dockerfile cannot provide compatibility with the 'None'
    // policy. An additional experimental policy 'SkipLayerCacheAndWarn' is the same as
    // 'SkipLayerCache' but simply warns if compatibility cannot be preserved.
    #[serde(default)]
    image_optimization_policy: String,
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct EnvVar {
    name: String,
    value: String,
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildOutput {
    to: ObjectReference, // To defines an optional location to push the output of this build to. Kind must be one of 'ImageMark' or 'DockerImage'.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildPostCommitSpec {
    // Script is a shell script to be run with `/bin/sh -ic`. It may not be
    // specified with Command. Use Script when a shell script is appropriate
    // to execute the post build hook, for example for running unit tests
    // with `rake test`.
    script: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Build {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the job
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: BuildConfig
    object_meta: ObjectMeta, //Standard object metadata. owner_refered by <assemblyFactory_id> >
    spec: BuildSpecData, //Spec holds all the input necessary to produce a new build, and the conditions when  to trigger them.
    status: BuildStatus, // BuildStatus contains the status of a build
    #[serde(default)]
    created_at: String,
}

impl Build {
    pub fn new() -> Build {
        ::std::default::Default::default()
    }
    //Create a new build with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Build {
        Build {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_spec(&mut self, v: BuildSpecData) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &BuildSpecData {
        &self.spec
    }

    pub fn set_status(&mut self, v: BuildStatus) {
        self.status = v;
    }
    pub fn get_status(&self) -> &BuildStatus {
        &self.status
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for Build {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildStatus {
    phase: String, // Phase is the point in the build lifecycle. Possible values are  "New", "Pending", "Running", "Complete", "Failed", "Error", and "Cancelled".
    #[serde(default)]
    cancelled: bool, // Cancelled describes if a cancel event was triggered for the build.
    #[serde(default)]
    reason: String, // Reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
    #[serde(default)]
    message: String, // Message is a human-readable message indicating details about why the build has this status.
    #[serde(default)]
    start_timestamp: String, // StartTimestamp is a timestamp representing the server time when this Build started running in a assembly.
    #[serde(default)]
    completion_timestamp: String, // CompletionTimestamp is a timestamp representing the server time when this Build was finished, whether that build failed or succeeded.  It reflects the time at which the assembly running the Build terminated.
    #[serde(default)]
    duration: String, // Duration contains time.Duration object describing build time.
    #[serde(default)]
    output_docker_image_reference: String, // OutputDockerImageReference contains a reference to the Docker image that will be built by this build.
    #[serde(default)]
    output: BuildStatusOutput, // Output describes the Docker image the build has produced.
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildStatusUpdate {
    pub status: BuildStatus,
    #[serde(default)]
    id: String,
}
impl BuildStatusUpdate {
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn get_status(&self) -> &BuildStatus {
        &self.status
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildStatusOutput {
    to: String, //To describes the status of the built image being pushed to a registry.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildSpecData {
    triggerd_by_causes: Vec<BuildTriggerCause>, // TriggeredBy describes which triggers started the most recent update to the buildconfig and contains information about those triggers.
    source: BuildSource,                        // Source describes the SCM in use.
    strategy: BuildStrategy,                    // Strategy defines how to perform a build.
    output: BuildOutput, // Output describes the Docker image the Strategy should produce.
    post_commit: BuildPostCommitSpec, // PostCommit is a build hook executed after the build output image is committed, before it is pushed to a registry.
    #[serde(default)]
    node_selector: BTreeMap<String, String>, // NodeSelector is a selector which must be true for the build assembly to fit on a node
}
// BuildTriggerCause holds information about a triggered build. It is used for
// displaying build trigger data for each build and build configuration in oc
// describe. It is also used to describe which triggers led to the most recent
// update in the build configuration.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BuildTriggerCause {
    message: String, // Message is used to store a human readable message for why the build was triggered. E.g.: "Manually triggered by user", "Configuration change",etc.
    #[serde(default)]
    webhook_cause: WebHookCause, // WebHook represents data for a specified webhook that fired a  specific build.
    #[serde(default)]
    image_build_cause: ImageChangeCause, // ImageChangeBuild stores information about an imagechange event that triggered a new build.
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct WebHookCause {
    hook_type: String,
    revision: SourceRevision, // Revision is an optional field that stores the git source revision information of the generic webhook trigger when it is available.
    secret: String,           // Secret is the obfuscated webhook secret that triggered a build.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SourceRevision {
    git: GitRevision, // Git contains information about git-based build source
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageChangeCause {
    image_id: String, // ImageID is the ID of the image that triggered a a new build.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct GitRevision {
    commit: String,  // Commit is the commit hash identifying a specific commit
    message: String, // Message is the description of a specific commit
}

// ImageStream stores a mapping of tags to images, metadata overrides that are applied
// when images are tagged in a stream, and an optional reference to a Docker image
// repository on a registry.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageReferences {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the job
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: BuildConfig
    object_meta: ObjectMeta,      //Standard object metadata
    spec: ImageReferenceSpec,     // Spec describes the desired state of this stream
    status: ImageReferenceStatus, // Status describes the current state of this stream
    #[serde(default)]
    created_at: String,
}

impl ImageReferences {
    pub fn new() -> ImageReferences {
        ::std::default::Default::default()
    }

    //Create a new image marks with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> ImageReferences {
        ImageReferences {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_spec(&mut self, v: ImageReferenceSpec) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &ImageReferenceSpec {
        &self.spec
    }
    pub fn set_status(&mut self, v: ImageReferenceStatus) {
        self.status = v;
    }
    pub fn get_status(&self) -> &ImageReferenceStatus {
        &self.status
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }
    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}
impl MetaFields for ImageReferences {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageReferenceSpec {
    #[serde(default)]
    lookup_policy: bool, // lookupPolicy controls how other resources reference images within this namespace.
    #[serde(default)]
    map_marks: BTreeMap<String, String>, //Tags map arbitrary string values to specific image locators
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageReferenceStatus {
    #[serde(default)]
    docker_image_repository: String, // DockerImageRepository represents the effective location this stream may be accessed at. May be empty until the server determines where the repository is located
    #[serde(default)]
    public_docker_image_repository: String, // PublicDockerImageRepository represents the public location from where the image can be pulled outside the cluster.
    #[serde(default)]
    tags: BTreeMap<String, TagEventList>, // A historical record of images associated with each tag. The first entry in the TagEvent array in the currently tagged image.
}
// TagEventList contains a historical record of images associated with a tag.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TagEventList {
    #[serde(default)]
    items: Vec<TagEvent>, // TagEvent is used by ImageRepositoryStatus to keep a historical record of images associated with a tag.
    #[serde(default)]
    conditions: Vec<TagEventCondition>, // Conditions is an array of conditions that apply to the tag event list.
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TagEventCondition {
    tag_event_condition_type: String, // Type of tag event condition, currently only ImportSuccess
    status: String,                   // Status of the condition, one of True, False, Unknown.
    last_transition_time: String, // LastTransitionTIme is the time the condition transitioned from one status to another.
    reason: String, // Reason is a brief machine readable explanation for the condition's last transition.
    generation: i64, // Generation is the spec tag generation that this status corresponds to. If this value is older than the spec tag generation, the user has requested this status tag be updated.
    message: String, // Message is a human readable description of the details about last transition, complementing reason.
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TagEvent {
    created: String,                // When the TagEvent was created
    docker_image_reference: String, // The string that can be used to pull this image
    image: String,
    generation: i64, // Generation is the spec tag generation that resulted in this tag being updated
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TagReference {
    name: String, // Name of the tag
    #[serde(default)]
    annotations: BTreeMap<String, String>, // Optional; if specified, annotations that are applied to images retrieved via ImageMarks.
    from: ObjectReference, // Optional; if specified, a reference to another image that this tag should point to. Valid values are ImageMark, ImageStreamImage, and DockerImage.
    #[serde(default)]
    reference: bool, // Reference states if the tag will be imported. Default value is false, which means the tag will be imported.
    #[serde(default)]
    generation: i64, // Generation is a counter that tracks mutations to the spec tag (user intent).
    import_policy: TagImportPolicy, // ImportPolicy is information that controls how images may be imported by the server.
    reference_policy: String, // ReferencePolicy defines how other components should consume the image.
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TagImportPolicy {
    #[serde(default)]
    insecure: bool, // Insecure is true if the server may bypass certificate verification or connect directly over HTTP during image import.
    #[serde(default)]
    scheduled: bool, // Scheduled indicates to the server that this tag should be periodically checked to ensure it is up to date, and imported
}

// ImageMark has a .Name in the format <stream name>:<tag>.//image marks
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageMarks {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the job
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: BuildConfig
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    tag: TagReference, // Tag is the spec tag associated with this image stream tag, and it may be null  if only pushes have occurred to this image stream.
    #[serde(default)]
    generation: i64, // Generation is the current generation of the tagged image
    #[serde(default)]
    conditions: Vec<TagEventCondition>, // Conditions is an array of conditions that apply to the image stream tag.
    #[serde(default)]
    lookup_policy: bool, // LookupPolicy indicates whether this tag will handle image references in this namespace.
    image: Image, // The Image associated with the ImageStream and tag.
    #[serde(default)]
    created_at: String,
}

impl ImageMarks {
    pub fn new() -> ImageMarks {
        ::std::default::Default::default()
    }

    //Create a new image marks with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> ImageMarks {
        ImageMarks {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_tag(&mut self, v: TagReference) {
        self.tag = v;
    }
    pub fn get_tag(&self) -> &TagReference {
        &self.tag
    }
    pub fn set_generation(&mut self, v: i64) {
        self.generation = v;
    }
    pub fn get_generation(&self) -> i64 {
        self.generation.clone()
    }
    pub fn set_lookup_policy(&mut self, v: bool) {
        self.lookup_policy = v;
    }
    pub fn get_lookup_policy(&self) -> bool {
        self.lookup_policy.clone()
    }
    pub fn set_conditions(&mut self, v: Vec<TagEventCondition>) {
        self.conditions = v;
    }
    pub fn get_conditions(&self) -> &Vec<TagEventCondition> {
        &self.conditions
    }
    pub fn set_image(&mut self, v: Image) {
        self.image = v;
    }
    pub fn get_image(&self) -> &Image {
        &self.image
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for ImageMarks {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Image {
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata
    name: String,
    #[serde(default)]
    size: u64,
    #[serde(default)]
    virtual_size: u64,
    #[serde(default)]
    docker_image_reference: String, // The string that can be used to pull this image.
    #[serde(default)]
    docker_image_layers: Vec<ImageLayer>, // DockerImageLayers represents the layers in the image. May not be set if the image does not define that data.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageLayer {
    #[serde(default)]
    layer_type: String, // type of the layer as defined by the underlying store.
    #[serde(default)]
    layers: Vec<String>, // DockerImage layers.
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_build() {
        let val = r#"{
            "object_meta":{
                "name":"ruby-build",
                "account":"931719409490206720",
                "owner_references":[{
                    "kind":"BuildConfig",
                    "api_version":"v1",
                    "name":"ruby-sample-build",
                    "uid":"921422565900042240",
                    "block_owner_deletion":false}]},
            "status":{
                "phase": "New",
                "cancelled": false},
            "spec": {
                "triggerd_by_causes": [{
                    "message": "",
                    "webhook_cause": {
                        "hook_type":"git",
                        "revision": {
                            "git": {
                                "commit": "78rftghjvbnm",
                                "message": "readme update"}},
                        "secret": "876543212345678909"}}],
                "source": {
                    "git": {
                        "uri": "https://github.com/openshift/ruby-hello-world",
                        "reference" : "master"},
                    "images": [{
                        "from": {
                            "kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},
                        "paths": [{
                            "source_path":"https:///avaf/vad",
                            "destination_dir":"/var/lib/"}]} ],
                    "source_secret": "" },
                    "strategy": {
                        "build_type":"Docker",
                        "source_strategy": {
                            "from": {
                                "kind": "ImageMarks","name": "ruby-20-centos7:latest","origin":"","uid":"","api_version":"","resource_version":"","field_path":"" }}  },
                            "output": {
                                "to": {
                                    "kind": "ImageMarks","name": "mydev-ruby-sample:latest","origin":"","uid":"","api_version":"","resource_version":"","field_path":""}},
                            "post_commit": {
                                "script": "bundle exec rake test" } }
                        }"#;
        let _build: Build = json_decode(val).unwrap();
    }
    #[test]
    fn decode_build_status() {
        let val = r#"{
            "phase": "New",
            "cancelled": false
        }"#;
        let build_status: BuildStatus = json_decode(val).unwrap();
        assert_eq!(build_status.phase, "New");
        assert_eq!(build_status.cancelled, false);
    }
    #[test]
    fn decode_build_config() {
        let val = r#"{
            "status":{
                "phase":"pending"},
            "object_meta":{
                "name":"ruby-sample-build",
                "account":"932243540329635840",
                "owner_references":[{
                    "kind":"AssemblyFactory",
                    "api_version":"v1",
                    "name":"levi.megam.io",
                    "uid":"891846866394710016",
                    "block_owner_deletion":false}]},
            "meta_data": {
                 "name": "ruby-sample-build"},
            "spec": {
                "run_policy": "Serial",
                "build_trigger_policys": [ {
                    "trigger_type": "gittrigger",
                    "webhook":  {
                        "hook_type": "GitHub",
                        "secret": "secret101"},
                    "image_change": {
                        "last_triggered_image_id": "1001" }} ],
                "source": {
                    "git": {
                        "uri": "https://github.com/openshift/ruby-hello-world",
                        "reference" : "master" },
                    "images": [ {
                        "from": {"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},
                    "paths": [{
                        "source_path":"https:///avaf/vad",
                        "destination_dir":"/var/lib/"}]} ],
                "source_secret": "secret_id"},
                "strategy":{
                    "build_type":"Source",
                    "source_strategy": {
                        "env":[{
                            "name":"DISABLE_ASSET_COMPILATION",
                            "value": "true"}],
                        "from":{"kind": "ImageMarks","name": "builder-image:latest","uid":"","api_version":"","resource_version":"","field_path":"","origin":""},
                    "scripts": "http://somehost.com/scripts_directory" } },
                "output": {
                    "to": {
                        "kind": "ImageMarks","name": "mydev-ruby-sample:latest" ,"uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""} },
                "post_commit": {"script": "bundle exec rake test"},
                "last_version": 10,
                "successful_builds_history_limit": 10,
                "failed_builds_history_limit": 1}
            }"#;
        let build_config: BuildConfig = json_decode(val).unwrap();
        assert!(build_config.meta_data.contains_key("name"));
    }
    #[test]
    fn decode_build_config_spec_data() {
        let val = r#"{
            "run_policy": "Serial",
            "build_trigger_policys": [ {
                "trigger_type": "gittrigger",
                "webhook":  {
                    "hook_type": "GitHub",
                    "secret": "secret101"},
                "image_change": {
                    "last_triggered_image_id": "1001" }} ],
            "source": {
                "git": {
                    "uri": "https://github.com/openshift/ruby-hello-world",
                    "reference" : "master" },
                "images": [ {
                    "from": {"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},
                "paths": [{
                    "source_path":"https:///avaf/vad",
                    "destination_dir":"/var/lib/"}]} ],
            "source_secret": "secret_id"},
            "strategy":{
                "build_type":"Source",
                "source_strategy": {
                    "env":[{
                        "name":"DISABLE_ASSET_COMPILATION",
                        "value": "true"}],
                    "from":{"kind": "ImageMarks","name": "builder-image:latest","uid":"","api_version":"","resource_version":"","field_path":"","origin":""},
                "scripts": "http://somehost.com/scripts_directory" } },
            "output": {
                "to": {
                    "kind": "ImageMarks","name": "mydev-ruby-sample:latest" ,"uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""} },
            "post_commit": {"script": "bundle exec rake test"},
            "last_version": 10,
            "successful_builds_history_limit": 10,
            "failed_builds_history_limit": 1
        }"#;
        let build_config_spec: SpecData = json_decode(val).unwrap();
        assert_eq!(build_config_spec.last_version, 10);
        assert_eq!(build_config_spec.successful_builds_history_limit, 10);
        assert_eq!(build_config_spec.failed_builds_history_limit, 1);
        assert_eq!(build_config_spec.run_policy, "Serial");
    }

    #[test]
    fn decode_image_reference() {
        let val = r#"{
            "object_meta":{
                "name":"ruby-image",
                "account":"946086857198804992",
                "owner_references":[{
                    "kind":"BuildConfig",
                    "api_version":"v1",
                    "name":"ruby-build",
                    "uid":"921422565900042240",
                    "block_owner_deletion":false}]},
            "spec":{
                "lookup_policy":false,
                "map_marks":{"ruby@371829c":"932309487992184832"}},
            "status":{
                "docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample",
                "tags":{
                    "docker": {
                        "items":[{
                            "created": "2016-01-29T13:40:11Z",
                            "docker_image_reference": "172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d",
                            "image": "sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d",
                            "generation": 1}]
                                }
                            }
                        }
                    }"#;
        let _image_ref: ImageReferences = json_decode(val).unwrap();
    }

    #[test]
    fn decode_image_reference_status() {
        let val = r#"{
            "docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample",
            "tags":{
                "docker": {
                    "items":[{
                        "created": "2016-01-29T13:40:11Z",
                        "docker_image_reference": "172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d",
                        "image": "sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d",
                        "generation": 1}]
                            }
                        }
                    }"#;
        let image_ref_status: ImageReferenceStatus = json_decode(val).unwrap();
        assert_eq!(
            image_ref_status.docker_image_repository,
            "172.30.56.218:5000/test/origin-ruby-sample"
        );
        assert_eq!(image_ref_status.public_docker_image_repository, "");
        assert!(image_ref_status.tags.contains_key("docker"));
    }
    #[test]
    fn decode_image_marks() {
        let val = r#"{
            "object_meta":{
                "name":"ruby@371829c",
                "account":"946050327142998016",
                "owner_references":[{
                    "kind":"Build",
                    "api_version":"v1",
                    "name":"ruby-i",
                    "uid":"921422565900042240",
                    "block_owner_deletion":false}]},
            "lookup_policy":false,
            "generation":0,
            "image":{
                "name": "ruby@123",
                "size":156800,
                "virtual_size": 168400,
                "docker_image_reference":"registry.rioos.xyz/test.megam.io/ruby:latest",
                "docker_image_layers":[{
                    "layer_type":"",
                    "layers":[]
                                }]
                            }
                        }"#;
        let image_marks: ImageMarks = json_decode(val).unwrap();
        assert_eq!(image_marks.lookup_policy, false);
        assert_eq!(image_marks.generation, 0);
    }

    #[test]
    fn decode_image_data() {
        let val = r#"{
                "name": "ruby@123",
                "size":156800,
                "virtual_size": 168400,
                "docker_image_reference":"registry.rioos.xyz/test.megam.io/ruby:latest",
                "docker_image_layers":[{
                    "layer_type":"",
                    "layers":[]
                                }]
                            }"#;
        let image: Image = json_decode(val).unwrap();
        assert_eq!(image.name, "ruby@123");
        assert_eq!(image.size, 156800);
        assert_eq!(image.virtual_size, 168400);
        assert_eq!(
            image.docker_image_reference,
            "registry.rioos.xyz/test.megam.io/ruby:latest"
        );
    }

}
