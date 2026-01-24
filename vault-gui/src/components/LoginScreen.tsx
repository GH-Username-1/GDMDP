import { useState } from "react";
import { VaultService } from "../services/vaultService";
import { VaultEntry } from "../types";

interface LoginScreenProps {
  onUnlock: (entries: VaultEntry[]) => void;
}

export function LoginScreen({ onUnlock }: LoginScreenProps) {
  const [vaultPath, setVaultPath] = useState("vault.dat");
  const [masterPassword, setMasterPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [mode, setMode] = useState<"open" | "create">("open");

  const handleOpenVault = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    const result = await VaultService.openVault(vaultPath, masterPassword);

    if (result.success && result.data) {
      onUnlock(result.data);
    } else {
      setError(result.error || "Erreur lors de l'ouverture du coffre");
    }

    setLoading(false);
  };

  const handleCreateVault = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    if (masterPassword.length < 8) {
      setError("Le master password doit contenir au moins 8 caractères");
      setLoading(false);
      return;
    }

    if (masterPassword !== confirmPassword) {
      setError("Les mots de passe ne correspondent pas");
      setLoading(false);
      return;
    }

    const result = await VaultService.createVault(vaultPath, masterPassword);

    if (result.success) {
      // Ouvrir le coffre nouvellement créé
      const openResult = await VaultService.openVault(vaultPath, masterPassword);
      if (openResult.success && openResult.data) {
        onUnlock(openResult.data);
      }
    } else {
      setError(result.error || "Erreur lors de la création du coffre");
    }

    setLoading(false);
  };

  return (
    <div className="login-screen">
      <div className="login-container">
        <div className="login-header">
          <h1>🔐 Vault Password Manager</h1>
          <p>Gestionnaire de mots de passe local et sécurisé</p>
        </div>

        <div className="login-tabs">
          <button
            className={`tab ${mode === "open" ? "active" : ""}`}
            onClick={() => {
              setMode("open");
              setError("");
              setConfirmPassword("");
            }}
          >
            Ouvrir un coffre
          </button>
          <button
            className={`tab ${mode === "create" ? "active" : ""}`}
            onClick={() => {
              setMode("create");
              setError("");
              setConfirmPassword("");
            }}
          >
            Créer un coffre
          </button>
        </div>

        <form onSubmit={mode === "open" ? handleOpenVault : handleCreateVault}>
          <div className="form-group">
            <label>📁 Chemin du coffre</label>
            <input
              type="text"
              value={vaultPath}
              onChange={(e) => setVaultPath(e.target.value)}
              placeholder="vault.dat"
              required
            />
            <small className="form-hint">
              Chemin relatif ou absolu vers votre fichier de coffre
            </small>
          </div>

          <div className="form-group">
            <label>🔑 Master Password</label>
            <input
              type="password"
              value={masterPassword}
              onChange={(e) => setMasterPassword(e.target.value)}
              placeholder="Entrez votre master password"
              required
              minLength={mode === "create" ? 8 : undefined}
            />
            {mode === "create" && (
              <small className="form-hint">
                Minimum 8 caractères - Ce mot de passe ne peut pas être récupéré
                !
              </small>
            )}
          </div>

          {mode === "create" && (
            <div className="form-group">
              <label>🔑 Confirmer le Master Password</label>
              <input
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                placeholder="Confirmez votre master password"
                required
                minLength={8}
              />
              <small className="form-hint">
                Retapez le même mot de passe pour confirmation
              </small>
            </div>
          )}

          {error && <div className="error-message">{error}</div>}

          <button type="submit" className="btn-primary btn-large" disabled={loading}>
            {loading ? (
              <span>⏳ Chargement...</span>
            ) : mode === "open" ? (
              <span>🔓 Déverrouiller</span>
            ) : (
              <span>✨ Créer le coffre</span>
            )}
          </button>
        </form>

        <div className="login-footer">
          <p>
            <strong>⚠️ Important :</strong> Votre master password n'est jamais
            stocké. <br />
            Si vous le perdez, aucune récupération n'est possible.
          </p>
        </div>
      </div>
    </div>
  );
}
