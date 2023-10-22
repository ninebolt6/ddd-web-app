use crate::external::database::ConnectionFactory;

pub struct Injector<CF>
where
    Self: Clone,
    CF: ConnectionFactory,
{
    connection_factory: CF,
}

impl<CF: ConnectionFactory + Clone> Clone for Injector<CF> {
    fn clone(&self) -> Self {
        Self {
            connection_factory: self.connection_factory.clone(),
        }
    }
}
impl<CF: ConnectionFactory + Clone> Injector<CF> {
    pub fn new(connection_factory: CF) -> Self {
        Self { connection_factory }
    }

    pub fn connection_factory(&self) -> &CF {
        &self.connection_factory
    }
}
