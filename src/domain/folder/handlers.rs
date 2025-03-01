//! Folder handling module.
//!
//! This module gathers all folder actions triggered by the CLI.

use anyhow::Result;
use dialoguer::Confirm;
use email::{account::AccountConfig, backend::Backend};
use std::process;

use crate::{
    printer::{PrintTableOpts, Printer},
    Folders,
};

pub async fn expunge<P: Printer>(
    printer: &mut P,
    backend: &mut dyn Backend,
    folder: &str,
) -> Result<()> {
    backend.expunge_folder(folder).await?;
    printer.print(format!("Folder {folder} successfully expunged!"))
}

pub async fn list<P: Printer>(
    config: &AccountConfig,
    printer: &mut P,
    backend: &mut dyn Backend,
    max_width: Option<usize>,
) -> Result<()> {
    let folders: Folders = backend.list_folders().await?.into();
    printer.print_table(
        // TODO: remove Box
        Box::new(folders),
        PrintTableOpts {
            format: &config.email_reading_format,
            max_width,
        },
    )
}

pub async fn create<P: Printer>(
    printer: &mut P,
    backend: &mut dyn Backend,
    folder: &str,
) -> Result<()> {
    backend.add_folder(folder).await?;
    printer.print("Folder successfully created!")
}

pub async fn delete<P: Printer>(
    printer: &mut P,
    backend: &mut dyn Backend,
    folder: &str,
) -> Result<()> {
    if let Some(false) | None = Confirm::new()
        .with_prompt(format!("Confirm deletion of folder {folder}?"))
        .default(false)
        .report(false)
        .interact_opt()?
    {
        process::exit(0);
    };

    backend.delete_folder(folder).await?;
    printer.print("Folder successfully deleted!")
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use email::{
        account::AccountConfig,
        backend::Backend,
        email::{Envelope, Envelopes, Flags, Messages},
        folder::{Folder, Folders},
    };
    use std::{any::Any, fmt::Debug, io};
    use termcolor::ColorSpec;

    use crate::printer::{Print, PrintTable, WriteColor};

    use super::*;

    #[tokio::test]
    async fn it_should_list_mboxes() {
        #[derive(Debug, Default, Clone)]
        struct StringWriter {
            content: String,
        }

        impl io::Write for StringWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.content
                    .push_str(&String::from_utf8(buf.to_vec()).unwrap());
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                self.content = String::default();
                Ok(())
            }
        }

        impl termcolor::WriteColor for StringWriter {
            fn supports_color(&self) -> bool {
                false
            }

            fn set_color(&mut self, _spec: &ColorSpec) -> io::Result<()> {
                io::Result::Ok(())
            }

            fn reset(&mut self) -> io::Result<()> {
                io::Result::Ok(())
            }
        }

        impl WriteColor for StringWriter {}

        #[derive(Debug, Default)]
        struct PrinterServiceTest {
            pub writer: StringWriter,
        }

        impl Printer for PrinterServiceTest {
            fn print_table<T: Debug + PrintTable + erased_serde::Serialize + ?Sized>(
                &mut self,
                data: Box<T>,
                opts: PrintTableOpts,
            ) -> anyhow::Result<()> {
                data.print_table(&mut self.writer, opts)?;
                Ok(())
            }
            fn print_log<T: Debug + Print>(&mut self, _data: T) -> anyhow::Result<()> {
                unimplemented!()
            }
            fn print<T: Debug + Print + serde::Serialize>(
                &mut self,
                _data: T,
            ) -> anyhow::Result<()> {
                unimplemented!()
            }
            fn is_json(&self) -> bool {
                unimplemented!()
            }
        }

        struct TestBackend;

        #[async_trait]
        impl Backend for TestBackend {
            fn name(&self) -> String {
                unimplemented!();
            }
            async fn add_folder(&mut self, _: &str) -> email::Result<()> {
                unimplemented!();
            }
            async fn list_folders(&mut self) -> email::Result<Folders> {
                Ok(Folders::from_iter([
                    Folder {
                        name: "INBOX".into(),
                        desc: "desc".into(),
                    },
                    Folder {
                        name: "Sent".into(),
                        desc: "desc".into(),
                    },
                ]))
            }
            async fn expunge_folder(&mut self, _: &str) -> email::Result<()> {
                unimplemented!();
            }
            async fn purge_folder(&mut self, _: &str) -> email::Result<()> {
                unimplemented!();
            }
            async fn delete_folder(&mut self, _: &str) -> email::Result<()> {
                unimplemented!();
            }
            async fn get_envelope(&mut self, _: &str, _: &str) -> email::Result<Envelope> {
                unimplemented!();
            }
            async fn list_envelopes(
                &mut self,
                _: &str,
                _: usize,
                _: usize,
            ) -> email::Result<Envelopes> {
                unimplemented!()
            }
            async fn search_envelopes(
                &mut self,
                _: &str,
                _: &str,
                _: &str,
                _: usize,
                _: usize,
            ) -> email::Result<Envelopes> {
                unimplemented!()
            }
            async fn add_email(&mut self, _: &str, _: &[u8], _: &Flags) -> email::Result<String> {
                unimplemented!()
            }
            async fn get_emails(&mut self, _: &str, _: Vec<&str>) -> email::Result<Messages> {
                unimplemented!()
            }
            async fn preview_emails(&mut self, _: &str, _: Vec<&str>) -> email::Result<Messages> {
                unimplemented!()
            }
            async fn copy_emails(&mut self, _: &str, _: &str, _: Vec<&str>) -> email::Result<()> {
                unimplemented!()
            }
            async fn move_emails(&mut self, _: &str, _: &str, _: Vec<&str>) -> email::Result<()> {
                unimplemented!()
            }
            async fn delete_emails(&mut self, _: &str, _: Vec<&str>) -> email::Result<()> {
                unimplemented!()
            }
            async fn add_flags(&mut self, _: &str, _: Vec<&str>, _: &Flags) -> email::Result<()> {
                unimplemented!()
            }
            async fn set_flags(&mut self, _: &str, _: Vec<&str>, _: &Flags) -> email::Result<()> {
                unimplemented!()
            }
            async fn remove_flags(
                &mut self,
                _: &str,
                _: Vec<&str>,
                _: &Flags,
            ) -> email::Result<()> {
                unimplemented!()
            }
            fn as_any(&self) -> &dyn Any {
                unimplemented!()
            }
        }

        let account_config = AccountConfig::default();
        let mut printer = PrinterServiceTest::default();
        let mut backend = TestBackend {};

        assert!(list(&account_config, &mut printer, &mut backend, None)
            .await
            .is_ok());
        assert_eq!(
            concat![
                "\n",
                "NAME  │DESC \n",
                "INBOX │desc \n",
                "Sent  │desc \n",
                "\n"
            ],
            printer.writer.content
        );
    }
}
