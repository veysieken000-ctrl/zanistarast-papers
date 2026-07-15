use crate::article_inventory::{
    ArticleInventoryBuilder,
    ArticleInventoryReport,
};
use crate::chat_interface::MiraChatCommand;
use crate::repository_scanner::{
    RepositoryScanReport,
    RepositoryScanner,
};
use crate::website_scanner::{
    WebsiteScanReport,
    WebsiteScanner,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

/// Mira komut yönlendiricisinin üretebildiği salt okunur sonuçlar.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommandExecutionResult {
    RepositoryScan(RepositoryScanReport),
    WebsiteScan(WebsiteScanReport),
    ArticleInventory(ArticleInventoryReport),
    NotExecutable {
        message: String,
    },
}

/// Yazılı Mira komutlarını ilgili gerçek modüllere yönlendirir.
///
/// Bu katman:
/// - yalnızca izin verilen salt okunur komutları çalıştırır,
/// - bilinmeyen komutları çalıştırmaz,
/// - dosya oluşturmaz,
/// - dosya değiştirmez,
/// - dosya taşımaz,
/// - dosya silmez,
/// - yayın veya merge işlemi yapmaz.
#[derive(Debug, Default)]
pub struct CommandRouter {
    repository_scanner: RepositoryScanner,
    website_scanner: WebsiteScanner,
    article_inventory_builder: ArticleInventoryBuilder,
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            repository_scanner: RepositoryScanner::new(),
            website_scanner: WebsiteScanner::new(),
            article_inventory_builder: ArticleInventoryBuilder::new(),
        }
    }

    /// Komutu verilen kök klasör üzerinde güvenli biçimde yürütür.
    pub fn execute(
        &self,
        command: &MiraChatCommand,
        root: impl AsRef<Path>,
    ) -> io::Result<CommandExecutionResult> {
        let root = root.as_ref();

        match command {
            MiraChatCommand::ScanRepository => {
                let report = self.repository_scanner.scan(root)?;

                Ok(CommandExecutionResult::RepositoryScan(
                    report,
                ))
            }

            MiraChatCommand::ScanWebsite => {
                let report = self.website_scanner.scan(root)?;

                Ok(CommandExecutionResult::WebsiteScan(
                    report,
                ))
            }

            MiraChatCommand::BuildArticleInventory => {
                let repository_report =
                    self.repository_scanner.scan(root)?;

                let website_report =
                    self.website_scanner.scan(root)?;

                let inventory =
                    self.article_inventory_builder.build(
                        &repository_report,
                        &website_report,
                    );

                Ok(CommandExecutionResult::ArticleInventory(
                    inventory,
                ))
            }

            MiraChatCommand::Help
            | MiraChatCommand::Status
            | MiraChatCommand::ListTasks => {
                Ok(CommandExecutionResult::NotExecutable {
                    message:
                        "Bu komut bilgi amaçlıdır; ayrı bir tarama işlemi gerektirmez."
                            .to_string(),
                })
            }

            MiraChatCommand::Unknown(command) => {
                Ok(CommandExecutionResult::NotExecutable {
                    message: format!(
                        "Tanımlanmamış komut güvenlik nedeniyle çalıştırılmadı: {command}"
                    ),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
   use std::path::{Path, PathBuf};

    fn create_test_site() -> PathBuf {
        let test_root = std::env::temp_dir().join(format!(
            "zanistarast-mira-command-router-{}",
            uuid::Uuid::new_v4()
        ));

        let articles = test_root.join("articles");

        fs::create_dir_all(&articles)
            .expect("test directories should be created");

        fs::write(
            test_root.join("README.md"),
            "Zanistarast repository",
        )
        .expect("README should be written");

        fs::write(
            articles.join("hebun.html"),
            r#"
                <html>
                    <head>
                        <title>Hebûn Makalesi</title>
                    </head>
                    <body>
                        <a href="../index.html">Ana sayfa</a>
                    </body>
                </html>
            "#,
        )
        .expect("Hebûn test page should be written");

        test_root
    }

    #[test]
    fn repository_command_runs_repository_scanner() {
        let test_root = create_test_site();
        let router = CommandRouter::new();

        let result = router
            .execute(
                &MiraChatCommand::ScanRepository,
                &test_root,
            )
            .expect("repository command should succeed");

        let CommandExecutionResult::RepositoryScan(report) =
            result
        else {
            panic!("repository scan result was expected");
        };

        assert_eq!(report.file_count(), 2);

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn website_command_runs_website_scanner() {
        let test_root = create_test_site();
        let router = CommandRouter::new();

        let result = router
            .execute(
                &MiraChatCommand::ScanWebsite,
                &test_root,
            )
            .expect("website command should succeed");

        let CommandExecutionResult::WebsiteScan(report) =
            result
        else {
            panic!("website scan result was expected");
        };

        assert_eq!(report.page_count(), 1);
        assert_eq!(
            report.pages[0].title.as_deref(),
            Some("Hebûn Makalesi")
        );

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn inventory_command_builds_real_inventory() {
        let test_root = create_test_site();
        let router = CommandRouter::new();

        let result = router
            .execute(
                &MiraChatCommand::BuildArticleInventory,
                &test_root,
            )
            .expect("inventory command should succeed");

        let CommandExecutionResult::ArticleInventory(report) =
            result
        else {
            panic!("article inventory result was expected");
        };

        assert_eq!(report.total_candidate_count(), 2);

        assert!(report.candidates.iter().any(|candidate| {
           candidate.relative_path
    == Path::new("articles/hebun.html")

        }));

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn unknown_command_is_not_executed() {
        let test_root = create_test_site();
        let router = CommandRouter::new();

        let result = router
            .execute(
                &MiraChatCommand::Unknown(
                    "Bütün dosyaları sil".to_string(),
                ),
                &test_root,
            )
            .expect("unknown command should return safe result");

        let CommandExecutionResult::NotExecutable {
            message,
        } = result
        else {
            panic!("not executable result was expected");
        };

        assert!(message.contains("çalıştırılmadı"));

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn informational_command_does_not_start_scan() {
        let test_root = create_test_site();
        let router = CommandRouter::new();

        let result = router
            .execute(
                &MiraChatCommand::Help,
                &test_root,
            )
            .expect("help command should return safely");

        assert!(matches!(
            result,
            CommandExecutionResult::NotExecutable { .. }
        ));

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }
}


