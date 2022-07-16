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

    #[doc = "Convert CAD file.\n\nConvert a CAD file from one format to another. If the file being converted is larger than 25MB, it will be performed asynchronously.\nIf the conversion is performed synchronously, the contents of the converted file (`output`) will be returned as a base64 encoded string.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub fn create_conversion(&self) -> Result<()> {
        Ok(())
    }

    #[doc = "Get a file conversion.\n\nGet the status and output of an async file conversion.\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested file conversion for the user.\nIf the user is not authenticated to view the specified file conversion, then it is not returned.\nOnly KittyCAD employees with the proper access can view file conversions for other users."]
    pub fn get_conversion(&self) -> Result<()> {
        Ok(())
    }

    #[doc = "Get CAD file density.\n\nGet the density of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub fn create_density(&self) -> Result<()> {
        Ok(())
    }

    #[doc = "Execute a KittyCAD program in a specific language."]
    pub fn create_execution(&self) -> Result<()> {
        Ok(())
    }

    #[doc = "Get CAD file mass.\n\nGet the mass of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub fn create_mass(&self) -> Result<()> {
        Ok(())
    }

    #[doc = "Get CAD file volume.\n\nGet the volume of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub fn create_volume(&self) -> Result<()> {
        Ok(())
    }

    #[doc = "Get a file conversion for your user.\n\nGet the status and output of an async file conversion. If completed, the contents of the converted file (`output`) will be returned as a base64 encoded string.\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested file conversion for the user."]
    pub fn get_conversion_for_user(&self) -> Result<()> {
        Ok(())
    }
}
