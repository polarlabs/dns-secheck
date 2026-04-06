use hickory_proto::rr::IntoName;
use hickory_resolver::name_server::{GenericConnector, TokioConnectionProvider};
use hickory_proto::runtime::TokioRuntimeProvider;
use hickory_resolver::lookup_ip::LookupIp;
use hickory_resolver::system_conf::read_system_conf;

pub struct Resolver(hickory_resolver::Resolver<GenericConnector<TokioRuntimeProvider>>);

impl Resolver {
    pub fn new() -> Resolver {
        let (config, opts) = read_system_conf().unwrap();

        // Build resolver that uses the system configuration
        let resolver = hickory_resolver::Resolver::builder_with_config(
            config,
            TokioConnectionProvider::default()
        ).build();

        Resolver(resolver)
    }

    pub async fn lookup_ip(&self, host: impl IntoName) -> LookupIp {
        let lookup = self.0.lookup_ip(host).await.unwrap();
        lookup
    }
}
