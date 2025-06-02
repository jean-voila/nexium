use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum GitlabError {
    InvalidToken,
    NetworkError,
    UserNotFound,
    UnknownError,
    BadGPGFormat,
    NoWebBrowser,
    AbortedLogin,
    UnauthorizedAccessToPort,
}

impl Display for GitlabError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GitlabError::InvalidToken => write!(f, "Token invalide."),
            GitlabError::NetworkError => write!(f, "Erreur réseau."),
            GitlabError::UserNotFound => write!(f, "Utilisateur non trouvé."),
            GitlabError::UnknownError => write!(f, "Erreur inconnue."),
            GitlabError::BadGPGFormat => {
                write!(f, "Format de clé GPG invalide.")
            }
            GitlabError::NoWebBrowser => {
                write!(f, "Aucun navigateur web trouvé.")
            }
            GitlabError::AbortedLogin => write!(f, "Connexion annulée."),
            GitlabError::UnauthorizedAccessToPort => {
                write!(f, "Accès non autorisé au port.")
            }
        }
    }
}
