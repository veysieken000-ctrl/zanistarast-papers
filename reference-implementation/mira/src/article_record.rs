use std::time::SystemTime;

/// Bir makalenin Zanistarast yayın sistemi içindeki genel durumudur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleStatus {
    Draft,
    AwaitingRasterast,
    AwaitingMudebbir,
    Approved,
    PublicationQueued,
    OfficiallyPublished,
    Archived,
    Rejected,
}

/// Makalenin belirli bir yayın kanalındaki durumudur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticlePublicationState {
    NotScheduled,
    Queued,
    Publishing,
    Published,
    Failed,
}

/// GitHub, Zanistarast sitesi, Medium veya LinkedIn gibi
/// tek bir yayın hedefinin sonucunu tutar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticlePublicationTarget {
    pub state: ArticlePublicationState,
    pub url: Option<String>,
    pub identifier: Option<String>,
    pub published_at: Option<SystemTime>,
    pub last_error: Option<String>,
}

impl ArticlePublicationTarget {
    pub fn not_scheduled() -> Self {
        Self {
            state: ArticlePublicationState::NotScheduled,
            url: None,
            identifier: None,
            published_at: None,
            last_error: None,
        }
    }

    pub fn queued() -> Self {
        Self {
            state: ArticlePublicationState::Queued,
            url: None,
            identifier: None,
            published_at: None,
            last_error: None,
        }
    }

    pub fn published(
        url: impl Into<String>,
        identifier: Option<String>,
        published_at: SystemTime,
    ) -> Self {
        Self {
            state: ArticlePublicationState::Published,
            url: Some(url.into()),
            identifier,
            published_at: Some(published_at),
            last_error: None,
        }
    }

    pub fn failed(error: impl Into<String>) -> Self {
        Self {
            state: ArticlePublicationState::Failed,
            url: None,
            identifier: None,
            published_at: None,
            last_error: Some(error.into()),
        }
    }

    pub fn is_published(&self) -> bool {
        self.state == ArticlePublicationState::Published
            && self
                .url
                .as_ref()
                .is_some_and(|url| !url.trim().is_empty())
            && self.published_at.is_some()
    }
}

/// Makalenin bütün yaşam döngüsünü temsil eden kalıcı ana kayıttır.
///
/// `PublicationPackage` yayımlanacak LaTeX, PDF ve BibTeX dosyalarını
/// taşırken, `ArticleRecord` makalenin kimliğini, sınıflandırmasını,
/// onaylarını ve yayın kanallarındaki durumunu takip eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleRecord {
    pub article_id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub category: String,
    pub article_type: String,
    pub language: String,
    pub keywords: Vec<String>,
    pub version: String,
    pub status: ArticleStatus,

    pub created_at: SystemTime,
    pub rasterast_verified_at: Option<SystemTime>,
    pub mudebbir_approved_at: Option<SystemTime>,
    pub officially_published_at: Option<SystemTime>,

    pub github: ArticlePublicationTarget,
    pub website: ArticlePublicationTarget,
    pub medium: ArticlePublicationTarget,
    pub linkedin: ArticlePublicationTarget,
}

impl ArticleRecord {
    pub fn new(
        article_id: impl Into<String>,
        title: impl Into<String>,
        authors: Vec<String>,
        category: impl Into<String>,
        article_type: impl Into<String>,
        language: impl Into<String>,
        keywords: Vec<String>,
        version: impl Into<String>,
        created_at: SystemTime,
    ) -> Self {
        Self {
            article_id: article_id.into(),
            title: title.into(),
            authors,
            category: category.into(),
            article_type: article_type.into(),
            language: language.into(),
            keywords,
            version: version.into(),
            status: ArticleStatus::Draft,
            created_at,
            rasterast_verified_at: None,
            mudebbir_approved_at: None,
            officially_published_at: None,
            github: ArticlePublicationTarget::not_scheduled(),
            website: ArticlePublicationTarget::not_scheduled(),
            medium: ArticlePublicationTarget::not_scheduled(),
            linkedin: ArticlePublicationTarget::not_scheduled(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.article_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.authors.is_empty()
            && self
                .authors
                .iter()
                .all(|author| !author.trim().is_empty())
            && !self.category.trim().is_empty()
            && !self.article_type.trim().is_empty()
            && !self.language.trim().is_empty()
            && !self.version.trim().is_empty()
    }

    pub fn is_officially_published(&self) -> bool {
        self.status == ArticleStatus::OfficiallyPublished
            && self.officially_published_at.is_some()
    }

    pub fn is_archived(&self) -> bool {
        self.status == ArticleStatus::Archived
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_article_record() -> ArticleRecord {
        ArticleRecord::new(
            "duygu-akil-001",
            "Duygu ve Akılın Çift Kanatlı İdrak Modeli",
            vec!["Veysi yê MALA SAF".to_string()],
            "Çift Kanatlı İdrak",
            "Kuramsal Makale",
            "tr",
            vec![
                "Duygu".to_string(),
                "Akıl".to_string(),
                "Rasterast".to_string(),
            ],
            "1.0.0",
            SystemTime::now(),
        )
    }

    #[test]
    fn creates_complete_article_record_as_draft() {
        let article = complete_article_record();

        assert!(article.is_complete());
        assert_eq!(article.status, ArticleStatus::Draft);
        assert!(!article.is_officially_published());
        assert!(!article.is_archived());
    }

    #[test]
    fn new_article_has_no_scheduled_publication_targets() {
        let article = complete_article_record();

        assert_eq!(
            article.github.state,
            ArticlePublicationState::NotScheduled
        );

        assert_eq!(
            article.website.state,
            ArticlePublicationState::NotScheduled
        );

        assert_eq!(
            article.medium.state,
            ArticlePublicationState::NotScheduled
        );

        assert_eq!(
            article.linkedin.state,
            ArticlePublicationState::NotScheduled
        );
    }

    #[test]
    fn published_target_requires_url_and_publication_time() {
        let target = ArticlePublicationTarget::published(
            "https://veysieken000-ctrl.github.io/zanistarast-ai-native-model/",
            Some("duygu-akil-001".to_string()),
            SystemTime::now(),
        );

        assert!(target.is_published());
        assert_eq!(
            target.state,
            ArticlePublicationState::Published
        );
        assert_eq!(
            target.identifier.as_deref(),
            Some("duygu-akil-001")
        );
        assert_eq!(target.last_error, None);
    }

    #[test]
    fn failed_target_preserves_error() {
        let target = ArticlePublicationTarget::failed(
            "Medium publication failed",
        );

        assert_eq!(
            target.state,
            ArticlePublicationState::Failed
        );
        assert_eq!(
            target.last_error.as_deref(),
            Some("Medium publication failed")
        );
        assert!(!target.is_published());
    }

    #[test]
    fn empty_article_identifier_prevents_completeness() {
        let mut article = complete_article_record();
        article.article_id = String::new();

        assert!(!article.is_complete());
    }

    #[test]
    fn official_publication_requires_status_and_date() {
        let mut article = complete_article_record();

        article.status = ArticleStatus::OfficiallyPublished;

        assert!(!article.is_officially_published());

        article.officially_published_at =
            Some(SystemTime::now());

        assert!(article.is_officially_published());
    }

    #[test]
    fn archived_status_is_detected() {
        let mut article = complete_article_record();
        article.status = ArticleStatus::Archived;

        assert!(article.is_archived());
        assert!(!article.is_officially_published());
    }
}


