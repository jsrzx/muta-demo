use derive_more::{Display, From};
use metadata::MetadataService;
use muta::MutaBuilder;
use protocol::traits::{SDKFactory, Service, ServiceMapping, ServiceSDK};
use protocol::{ProtocolError, ProtocolErrorKind, ProtocolResult};

struct DefaultServiceMapping;

impl ServiceMapping for DefaultServiceMapping {
    fn get_service<SDK: 'static + ServiceSDK, Factory: SDKFactory<SDK>>(
        &self,
        name: &str,
        factory: &Factory,
    ) -> ProtocolResult<Box<dyn Service>> {
        let sdk = factory.get_sdk(name)?;

        let service = match name {
            "metadata" => Box::new(MetadataService::new(sdk)) as Box<dyn Service>,
            _ => return Err(MappingError::NotFoundService(name.to_string()).into()),
        };
        Ok(service)
    }

    fn list_service_name(&self) -> Vec<String> {
        vec!["asset".to_owned(), "metadata".to_owned()]
    }
}

fn main() {
    let builder = MutaBuilder::new();

    // set configs
    let muta_builder = builder
        .config_path("config/chain.toml")
        .genesis_path("config/genesis.toml");

    // set service-mapping
    muta_builder
        .service_mapping(DefaultServiceMapping {})
        .build()
        .unwrap()
        .run()
        .unwrap();
}

#[derive(Debug, Display, From)]
pub enum MappingError {
    #[display(fmt = "service {:?} was not found", _0)]
    NotFoundService(String),
}
impl std::error::Error for MappingError {}

impl From<MappingError> for ProtocolError {
    fn from(err: MappingError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::Binding, Box::new(err))
    }
}
