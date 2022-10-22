use std::{cell::RefCell, sync::Arc};

use crate::{gatt_server::service::Service, utilities::BleUuid};
use esp_idf_sys::*;
use log::{debug, info};

#[derive(Debug, Clone)]
pub struct Profile {
    name: Option<String>,
    pub(crate) services: Vec<Arc<RefCell<Service>>>,
    pub(crate) identifier: u16,
    pub(crate) interface: Option<u8>,
}

impl Profile {
    pub fn new(name: &str, identifier: u16) -> Self {
        Profile {
            name: Some(String::from(name)),
            services: Vec::new(),
            identifier,
            interface: None,
        }
    }

    pub fn add_service<S: Into<Arc<RefCell<Service>>>>(mut self, service: S) -> Self {
        self.services.push(service.into());
        self
    }

    pub(crate) fn get_service(&self, handle: u16) -> Option<Arc<RefCell<Service>>> {
        for service in &self.services {
            if service.borrow().handle == Some(handle) {
                return Some(service.clone());
            }
        }

        None
    }

    pub(crate) fn get_service_by_id(&self, id: esp_gatt_id_t) -> Option<Arc<RefCell<Service>>> {
        for service in &self.services {
            if service.borrow().uuid == id.into() {
                return Some(service.clone());
            }
        }

        None
    }

    pub(crate) fn register_self(&self) {
        debug!("Registering {}.", self);
        unsafe { esp_nofail!(esp_ble_gatts_app_register(self.identifier)) };
    }

    pub(crate) fn register_services(&mut self) {
        debug!("Registering {}'s services.", &self);
        self.services.iter_mut().for_each(|service| {
            service.borrow_mut().register_self(self.interface.unwrap());
        });
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (0x{:04x})",
            self.name
                .clone()
                .unwrap_or_else(|| "Unnamed profile".to_string()),
            self.identifier,
        )
    }
}
