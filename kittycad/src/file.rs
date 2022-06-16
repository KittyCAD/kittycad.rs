use anyhow::Result;

use crate::Client;

pub struct File {
    pub client: Client,
}

impl File {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        File { client }
    }

    /**
     * Convert CAD file.
     *
     * This function performs a `POST` to the `/file/conversion/{src_format}/{output_format}` endpoint.
     *
     * Convert a CAD file from one format to another. If the file being converted is larger than 25MB, it will be performed asynchronously.
     * If the conversion is performed synchronously, the contents of the converted file (`output`) will be returned as a base64 encoded string.
     * If the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `output_format: crate::types::FileOutputFormat` -- The format the file should be converted to.
     * * `src_format: crate::types::FileSourceFormat` -- The valid types of source file formats.
     */
    pub async fn create_conversion<B: Into<reqwest::Body>>(
        &self,
        output_format: crate::types::FileOutputFormat,
        src_format: crate::types::FileSourceFormat,
        body: B,
    ) -> Result<crate::types::FileConversion> {
        let url = format!(
            "/file/conversion/{}/{}",
            crate::progenitor_support::encode_path(&src_format.to_string()),
            crate::progenitor_support::encode_path(&output_format.to_string()),
        );

        self.client.post(&url, Some(body.into())).await
    }

    /**
     * Get a file conversion.
     *
     * This function performs a `GET` to the `/file/conversions/{id}` endpoint.
     *
     * Get the status and output of an async file conversion.
     * This endpoint requires authentication by any KittyCAD user. It returns details of the requested file conversion for the user.
     * If the user is not authenticated to view the specified file conversion, then it is not returned.
     * Only KittyCAD employees with the proper access can view file conversions for other users.
     *
     * **Parameters:**
     *
     * * `id: &str` -- The ID of the async operation.
     */
    pub async fn get_conversion(&self, id: &str) -> Result<crate::types::AsyncApiCallOutput> {
        let url = format!(
            "/file/conversions/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get CAD file density.
     *
     * This function performs a `POST` to the `/file/density` endpoint.
     *
     * Get the density of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.
     * If the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `material_mass: f64` -- The material mass.
     * * `src_format: crate::types::FileSourceFormat` -- The valid types of source file formats.
     */
    pub async fn create_density<B: Into<reqwest::Body>>(
        &self,
        material_mass: f64,
        src_format: crate::types::FileSourceFormat,
        body: B,
    ) -> Result<crate::types::FileDensity> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !material_mass.to_string().is_empty() {
            query_args.push(("material_mass".to_string(), material_mass.to_string()));
        }
        if !src_format.to_string().is_empty() {
            query_args.push(("src_format".to_string(), src_format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/file/density?{}", query_);

        self.client.post(&url, Some(body.into())).await
    }

    /**
     * Execute a KittyCAD program in a specific language.
     *
     * This function performs a `POST` to the `/file/execute/{lang}` endpoint.
     *
     * **Parameters:**
     *
     * * `lang: crate::types::CodeLanguage` -- The language code is written in.
     * * `output: &str` -- The output file we want to get the contents for (the paths are relative to where in litterbox it is being run). You can denote more than one file with a comma separated list of string paths.
     */
    pub async fn create_execution<B: Into<reqwest::Body>>(
        &self,
        lang: crate::types::CodeLanguage,
        output: &str,
        body: B,
    ) -> Result<crate::types::CodeOutput> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !output.is_empty() {
            query_args.push(("output".to_string(), output.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/file/execute/{}?{}",
            crate::progenitor_support::encode_path(&lang.to_string()),
            query_
        );

        self.client.post(&url, Some(body.into())).await
    }

    /**
     * Get CAD file mass.
     *
     * This function performs a `POST` to the `/file/mass` endpoint.
     *
     * Get the mass of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.
     * If the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `material_density: f64` -- The material density.
     * * `src_format: crate::types::FileSourceFormat` -- The valid types of source file formats.
     */
    pub async fn create_mass<B: Into<reqwest::Body>>(
        &self,
        material_density: f64,
        src_format: crate::types::FileSourceFormat,
        body: B,
    ) -> Result<crate::types::FileMass> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !material_density.to_string().is_empty() {
            query_args.push(("material_density".to_string(), material_density.to_string()));
        }
        if !src_format.to_string().is_empty() {
            query_args.push(("src_format".to_string(), src_format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/file/mass?{}", query_);

        self.client.post(&url, Some(body.into())).await
    }

    /**
     * Get CAD file volume.
     *
     * This function performs a `POST` to the `/file/volume` endpoint.
     *
     * Get the volume of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.
     * If the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `src_format: crate::types::FileSourceFormat` -- The valid types of source file formats.
     */
    pub async fn create_volume<B: Into<reqwest::Body>>(
        &self,
        src_format: crate::types::FileSourceFormat,
        body: B,
    ) -> Result<crate::types::FileVolume> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !src_format.to_string().is_empty() {
            query_args.push(("src_format".to_string(), src_format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/file/volume?{}", query_);

        self.client.post(&url, Some(body.into())).await
    }

    /**
     * Get a file conversion for your user.
     *
     * This function performs a `GET` to the `/user/file/conversions/{id}` endpoint.
     *
     * Get the status and output of an async file conversion. If completed, the contents of the converted file (`output`) will be returned as a base64 encoded string.
     * This endpoint requires authentication by any KittyCAD user. It returns details of the requested file conversion for the user.
     *
     * **Parameters:**
     *
     * * `id: &str` -- The ID of the async operation.
     */
    pub async fn get_conversion_for_user(
        &self,
        id: &str,
    ) -> Result<crate::types::AsyncApiCallOutput> {
        let url = format!(
            "/user/file/conversions/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
