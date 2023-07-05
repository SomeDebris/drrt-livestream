use serde::Serialize;

// #[derive(Serialize)]
// #[serde(tag = "requestType", content = "requestData")]
// pub(crate) enum Request<'a> {

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "get_downstream_keyers")]
    DownstreamKeyers,
    #[serde(rename = "get_downstream_keyer")]
    DownstreamKeyer {
        dsk_name: &'a str,
    },
    #[serde(rename = "dsk_select_scene")]
    ChangeScene {
        #[serde(rename = "dsk_name")]
        dsk_name: &'a str,
        #[serde(rename = "scene")]
        scene_name: &'a str,
    },
    // #[serde(rename = "dsk_add_scene")]
    // #[serde(rename = "dsk_remove_scene")]
    // #[serde(rename = "dsk_set_tie")]
    // #[serde(rename = "dsk_set_transition")]
    // #[serde(rename = "dsk_add_exclude_scene")]
    // #[serde(rename = "dsk_remove_exclude_scene")]


}
