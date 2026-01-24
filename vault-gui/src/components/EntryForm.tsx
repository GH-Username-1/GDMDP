import { useState, useEffect } from "react";
import { VaultEntry, PasswordConfig } from "../types";
import { VaultService } from "../services/vaultService";

interface EntryFormProps {
  entry?: VaultEntry | null;
  onClose: () => void;
  onSave: () => void;
}

export function EntryForm({ entry, onClose, onSave }: EntryFormProps) {
  const [serviceName, setServiceName] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [url, setUrl] = useState("");
  const [notes, setNotes] = useState("");
  const [tags, setTags] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [showPasswordGen, setShowPasswordGen] = useState(false);

  // Password generator state
  const [genLength, setGenLength] = useState(16);
  const [genUppercase, setGenUppercase] = useState(true);
  const [genLowercase, setGenLowercase] = useState(true);
  const [genDigits, setGenDigits] = useState(true);
  const [genSymbols, setGenSymbols] = useState(true);
  const [genExcludeAmbiguous, setGenExcludeAmbiguous] = useState(true);

  useEffect(() => {
    if (entry) {
      setServiceName(entry.service_name);
      setUsername(entry.username);
      setPassword(entry.password);
      setUrl(entry.url || "");
      setNotes(entry.notes || "");
      setTags(entry.tags.join(", "));
    }
  }, [entry]);

  const handleGeneratePassword = async () => {
    const config: PasswordConfig = {
      length: genLength,
      use_uppercase: genUppercase,
      use_lowercase: genLowercase,
      use_digits: genDigits,
      use_symbols: genSymbols,
      exclude_ambiguous: genExcludeAmbiguous,
    };

    const result = await VaultService.generatePassword(config);
    if (result.success && result.data) {
      setPassword(result.data);
      setShowPasswordGen(false);
    } else {
      setError(result.error || "Erreur lors de la génération");
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    const tagArray = tags
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t.length > 0);

    try {
      let result;
      if (entry) {
        // Update
        result = await VaultService.updateEntry(
          entry.id,
          serviceName,
          username,
          password,
          url || undefined,
          notes || undefined,
          tagArray
        );
      } else {
        // Add
        result = await VaultService.addEntry(
          serviceName,
          username,
          password,
          url || undefined,
          notes || undefined,
          tagArray
        );
      }

      if (result.success) {
        onSave();
        onClose();
      } else {
        setError(result.error || "Erreur lors de la sauvegarde");
      }
    } catch (err) {
      setError("Erreur inattendue");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>{entry ? "Modifier l'entrée" : "Nouvelle entrée"}</h2>
          <button className="btn-close" onClick={onClose}>
            ×
          </button>
        </div>

        <form onSubmit={handleSubmit} className="entry-form">
          <div className="form-group">
            <label>Service *</label>
            <input
              type="text"
              value={serviceName}
              onChange={(e) => setServiceName(e.target.value)}
              required
              placeholder="GitHub, Gmail, etc."
            />
          </div>

          <div className="form-group">
            <label>Username *</label>
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
              placeholder="utilisateur@email.com"
            />
          </div>

          <div className="form-group">
            <label>Mot de passe *</label>
            <div className="password-input-group">
              <input
                type="text"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
              />
              <button
                type="button"
                className="btn-secondary"
                onClick={() => setShowPasswordGen(!showPasswordGen)}
              >
                🎲 Générer
              </button>
            </div>
          </div>

          {showPasswordGen && (
            <div className="password-generator">
              <h4>Générateur de mot de passe</h4>
              <div className="form-row">
                <div className="form-group">
                  <label>Longueur: {genLength}</label>
                  <input
                    type="range"
                    min="8"
                    max="64"
                    value={genLength}
                    onChange={(e) => setGenLength(parseInt(e.target.value))}
                  />
                </div>
              </div>
              <div className="form-row checkboxes">
                <label>
                  <input
                    type="checkbox"
                    checked={genUppercase}
                    onChange={(e) => setGenUppercase(e.target.checked)}
                  />
                  Majuscules (A-Z)
                </label>
                <label>
                  <input
                    type="checkbox"
                    checked={genLowercase}
                    onChange={(e) => setGenLowercase(e.target.checked)}
                  />
                  Minuscules (a-z)
                </label>
                <label>
                  <input
                    type="checkbox"
                    checked={genDigits}
                    onChange={(e) => setGenDigits(e.target.checked)}
                  />
                  Chiffres (0-9)
                </label>
                <label>
                  <input
                    type="checkbox"
                    checked={genSymbols}
                    onChange={(e) => setGenSymbols(e.target.checked)}
                  />
                  Symboles (!@#$...)
                </label>
                <label>
                  <input
                    type="checkbox"
                    checked={genExcludeAmbiguous}
                    onChange={(e) => setGenExcludeAmbiguous(e.target.checked)}
                  />
                  Exclure ambigus (0,O,1,l,I)
                </label>
              </div>
              <button
                type="button"
                className="btn-primary"
                onClick={handleGeneratePassword}
              >
                Générer le mot de passe
              </button>
            </div>
          )}

          <div className="form-group">
            <label>URL</label>
            <input
              type="url"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder="https://example.com"
            />
          </div>

          <div className="form-group">
            <label>Notes</label>
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              rows={3}
              placeholder="Notes additionnelles..."
            />
          </div>

          <div className="form-group">
            <label>Tags (séparés par des virgules)</label>
            <input
              type="text"
              value={tags}
              onChange={(e) => setTags(e.target.value)}
              placeholder="travail, personnel, important"
            />
          </div>

          {error && <div className="error">{error}</div>}

          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={onClose}>
              Annuler
            </button>
            <button type="submit" className="btn-primary" disabled={loading}>
              {loading ? "Sauvegarde..." : "Sauvegarder"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
