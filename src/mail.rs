use lettre::message::header::ContentType;
use lettre::transport::smtp::{authentication::Credentials, response::Response};
use lettre::{SmtpTransport, Transport};
use tera::Context;

pub struct HtmlMailer {
    pub credentials: Credentials,
    pub smtp_host: String,
    pub template_engine: tera::Tera,
}

impl HtmlMailer {
    pub fn send(
        self,
        to: Vec<String>,
        subject: Option<String>,
        template_name: &str,
        context: &Context,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let html_body = self.template_engine.render(template_name, &context)?;
        let subject = subject.unwrap_or_else(|| "Cr8s digest".to_string());

        let mut message_builder = lettre::Message::builder()
            .subject(subject)
            .from("Ce8s <info@cr8s.com>".parse().unwrap())
            .to(to[0].parse()?)
            .header(ContentType::TEXT_HTML);

        if to.len() > 1 {
            for copy in &to[1..] {
                message_builder = message_builder.cc(copy.parse()?);
            }
        }

        let message = message_builder.body(html_body)?;

        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(self.credentials)
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}
