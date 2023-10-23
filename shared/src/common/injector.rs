use crate::external::database::ConnectionFactory;

pub struct Injector<CF>
where
    CF: ConnectionFactory + 'static,
{
    connection_factory: CF,
}

impl<CF: ConnectionFactory + Clone> Injector<CF> {
    pub fn new(connection_factory: CF) -> Self {
        Self { connection_factory }
    }

    pub fn connection_factory(&self) -> &CF {
        &self.connection_factory
    }
}
