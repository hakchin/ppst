#![recursion_limit = "512"]

pub mod app;
pub mod components;
pub mod models;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod server;

// Re-export server functions for client-side access
pub mod server_fns {
    use leptos::prelude::*;

    #[server]
    pub async fn submit_contact(
        name: String,
        phone: String,
        message: String,
    ) -> Result<(), ServerFnError> {
        use crate::models::ContactInquiry;
        use crate::server::file_store::save_contact_inquiry;

        // Validate and create inquiry
        let inquiry = ContactInquiry::new(name, phone, message)
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Save to file storage
        save_contact_inquiry(&inquiry)
            .map_err(|e| ServerFnError::new(format!("Failed to save inquiry: {}", e)))?;

        tracing::info!("Contact inquiry saved from: {}", inquiry.phone);

        Ok(())
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
