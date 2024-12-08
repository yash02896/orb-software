use zbus::{
    blocking::{connection, Connection, object_server::InterfaceRef},
    interface,
};

const OBJECT_PATH: &str = "/org/worldcoin/OrbUpdateAgent/Status";
const BUS_NAME: &str = "org.worldcoin.OrbUpdateAgent";

pub struct UpdateStatus {
    status: String,
}

impl Default for UpdateStatus {
    fn default() -> Self {
        Self {
            status: "none".to_string(),
        }
    }
}

#[interface(name = "org.worldcoin.OrbUpdateAgent1.Status")]
impl UpdateStatus {
    #[zbus(property)]
    fn update_status(&self) -> &str {
        &self.status
    }

    #[zbus(property)]
    fn set_update_status(&mut self, value: String) {
        self.status = value;
    }
}

impl UpdateStatus {
    /// Create a new connection to D-Bus and serve the `UpdateStatus` interface.
    pub fn create_dbus_conn() -> zbus::Result<Connection> {
        connection::Builder::session()?
            .name(BUS_NAME)?
            .serve_at(OBJECT_PATH, UpdateStatus::default())?
            .build()
    }

    fn get_iface_ref(conn: &Connection) -> Result<InterfaceDerefMut<'_,UpdateStatus>> {
        let object_server = conn.object_server();
        Ok(object_server.interface::<_, UpdateStatus>(OBJECT_PATH)?.get_mut())
    }

    pub fn set_conn_status(conn: &Connection, status: String) -> Result<()> {
        match UpdateStatus::get_iface_ref(conn) {
            Ok(iface) => {
                iface.set_update_status(status)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
