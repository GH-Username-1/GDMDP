import { VaultEntry } from "../types";

interface EntryCardProps {
  entry: VaultEntry;
  onEdit: (entry: VaultEntry) => void;
  onDelete: (id: string) => void;
  onCopyPassword: (password: string) => void;
}

export function EntryCard({ entry, onEdit, onDelete, onCopyPassword }: EntryCardProps) {
  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString("fr-FR", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  };

  return (
    <div className="entry-card">
      <div className="entry-header">
        <h3>{entry.service_name}</h3>
        <div className="entry-actions">
          <button
            className="btn-icon"
            onClick={() => onCopyPassword(entry.password)}
            title="Copier le mot de passe"
          >
            📋
          </button>
          <button
            className="btn-icon"
            onClick={() => onEdit(entry)}
            title="Modifier"
          >
            ✏️
          </button>
          <button
            className="btn-icon btn-danger"
            onClick={() => onDelete(entry.id)}
            title="Supprimer"
          >
            🗑️
          </button>
        </div>
      </div>

      <div className="entry-content">
        <div className="entry-field">
          <span className="field-label">👤 Username:</span>
          <span className="field-value">{entry.username}</span>
        </div>

        <div className="entry-field">
          <span className="field-label">🔑 Password:</span>
          <span className="field-value password-hidden">••••••••</span>
        </div>

        {entry.url && (
          <div className="entry-field">
            <span className="field-label">🌐 URL:</span>
            <a
              href={entry.url}
              target="_blank"
              rel="noopener noreferrer"
              className="field-value field-link"
            >
              {entry.url}
            </a>
          </div>
        )}

        {entry.notes && (
          <div className="entry-field">
            <span className="field-label">📝 Notes:</span>
            <span className="field-value">{entry.notes}</span>
          </div>
        )}

        {entry.tags && entry.tags.length > 0 && (
          <div className="entry-field">
            <span className="field-label">🏷️ Tags:</span>
            <div className="tags">
              {entry.tags.map((tag, idx) => (
                <span key={idx} className="tag">
                  {tag}
                </span>
              ))}
            </div>
          </div>
        )}

        <div className="entry-meta">
          <small>Modifié: {formatDate(entry.updated_at)}</small>
        </div>
      </div>
    </div>
  );
}
