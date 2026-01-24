import { useState } from "react";
import { VaultService } from "./services/vaultService";
import { VaultEntry } from "./types";
import { LoginScreen } from "./components/LoginScreen";
import { SearchBar } from "./components/SearchBar";
import { EntryCard } from "./components/EntryCard";
import { EntryForm } from "./components/EntryForm";
import { useAutoLock } from "./hooks/useAutoLock";
import { writeText } from "@tauri-apps/api/clipboard";

function App() {
  const [isLocked, setIsLocked] = useState(true);
  const [entries, setEntries] = useState<VaultEntry[]>([]);
  const [filteredEntries, setFilteredEntries] = useState<VaultEntry[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [showEntryForm, setShowEntryForm] = useState(false);
  const [editingEntry, setEditingEntry] = useState<VaultEntry | null>(null);
  const [notification, setNotification] = useState("");
  const [autoLockEnabled, setAutoLockEnabled] = useState(true);
  const [deleteConfirm, setDeleteConfirm] = useState<{ show: boolean; entryId: string; entryName: string }>({
    show: false,
    entryId: "",
    entryName: "",
  });

  // Auto-lock après 5 minutes d'inactivité
  useAutoLock(
    () => {
      showNotification("🔒 Coffre verrouillé automatiquement (inactivité)");
      setTimeout(() => handleLock(), 100); // Délai pour que la notification s'affiche
    },
    5,
    autoLockEnabled && !isLocked
  );

  // Charger les entrées au déverrouillage
  const handleUnlock = (loadedEntries: VaultEntry[]) => {
    setEntries(loadedEntries);
    setFilteredEntries(loadedEntries);
    setIsLocked(false);
    showNotification("✅ Coffre déverrouillé avec succès");
  };

  // Verrouiller le coffre
  const handleLock = async () => {
    await VaultService.lockVault();
    setIsLocked(true);
    setEntries([]);
    setFilteredEntries([]);
    setSearchQuery("");
    setShowEntryForm(false);
    setEditingEntry(null);
  };

  // Recherche en temps réel
  const handleSearch = async (query: string) => {
    setSearchQuery(query);

    if (!query.trim()) {
      setFilteredEntries(entries);
      return;
    }

    const result = await VaultService.searchEntries(query);
    if (result.success && result.data) {
      setFilteredEntries(result.data);
    }
  };

  // Rafraîchir la liste
  const refreshEntries = async () => {
    const result = await VaultService.listEntries();
    if (result.success && result.data) {
      setEntries(result.data);
      if (searchQuery) {
        handleSearch(searchQuery);
      } else {
        setFilteredEntries(result.data);
      }
    }
  };

  // Copier le mot de passe
  const handleCopyPassword = async (password: string) => {
    try {
      await writeText(password);
      showNotification("📋 Mot de passe copié dans le presse-papiers");
    } catch (error) {
      showNotification("❌ Erreur lors de la copie");
    }
  };

  // Ouvrir le formulaire d'édition
  const handleEdit = (entry: VaultEntry) => {
    setEditingEntry(entry);
    setShowEntryForm(true);
  };

  // Ouvrir le formulaire d'ajout
  const handleAdd = () => {
    setEditingEntry(null);
    setShowEntryForm(true);
  };

  // Fermer le formulaire
  const handleCloseForm = () => {
    setShowEntryForm(false);
    setEditingEntry(null);
  };

  // Sauvegarder (ajouter ou modifier)
  const handleSave = async () => {
    await refreshEntries();
    showNotification(
      editingEntry
        ? "✅ Entrée modifiée avec succès"
        : "✅ Entrée ajoutée avec succès"
    );
  };

  // Demander confirmation de suppression
  const handleDeleteRequest = (id: string, name: string) => {
    setDeleteConfirm({ show: true, entryId: id, entryName: name });
  };

  // Confirmer la suppression
  const handleDeleteConfirm = async () => {
    const result = await VaultService.deleteEntry(deleteConfirm.entryId);
    if (result.success) {
      await refreshEntries();
      showNotification("✅ Entrée supprimée avec succès");
    } else {
      showNotification("❌ " + (result.error || "Erreur lors de la suppression"));
    }
    setDeleteConfirm({ show: false, entryId: "", entryName: "" });
  };

  // Annuler la suppression
  const handleDeleteCancel = () => {
    setDeleteConfirm({ show: false, entryId: "", entryName: "" });
  };

  // Créer un backup
  const handleBackup = async () => {
    const result = await VaultService.createBackup();
    if (result.success) {
      showNotification("✅ Backup créé avec succès");
    } else {
      showNotification("❌ " + (result.error || "Erreur lors du backup"));
    }
  };

  // Afficher une notification
  const showNotification = (message: string) => {
    setNotification(message);
    setTimeout(() => setNotification(""), 3000);
  };

  // Écran de connexion
  if (isLocked) {
    return <LoginScreen onUnlock={handleUnlock} />;
  }

  // Dashboard principal
  return (
    <div className="app-container">
      {/* Header */}
      <header className="app-header">
        <div className="header-left">
          <h1>🔓 Vault Password Manager</h1>
          <span className="entry-count">{entries.length} entrée(s)</span>
        </div>
        <div className="header-right">
          <button
            className="btn-icon"
            onClick={() => setAutoLockEnabled(!autoLockEnabled)}
            title={
              autoLockEnabled
                ? "Auto-lock activé (5 min)"
                : "Auto-lock désactivé"
            }
          >
            {autoLockEnabled ? "⏱️" : "⏸️"}
          </button>
          <button className="btn-icon" onClick={handleBackup} title="Créer un backup">
            💾
          </button>
          <button className="btn-secondary" onClick={handleLock}>
            🔒 Verrouiller
          </button>
        </div>
      </header>

      {/* Toolbar */}
      <div className="toolbar">
        <SearchBar onSearch={handleSearch} />
        <button className="btn-primary" onClick={handleAdd}>
          ➕ Nouvelle entrée
        </button>
      </div>

      {/* Content */}
      <main className="app-content">
        {filteredEntries.length === 0 ? (
          <div className="empty-state">
            {searchQuery ? (
              <>
                <p className="empty-icon">🔍</p>
                <h3>Aucun résultat</h3>
                <p>Aucune entrée ne correspond à votre recherche</p>
              </>
            ) : (
              <>
                <p className="empty-icon">📭</p>
                <h3>Aucune entrée</h3>
                <p>Commencez par ajouter une nouvelle entrée</p>
                <button className="btn-primary" onClick={handleAdd}>
                  ➕ Ajouter une entrée
                </button>
              </>
            )}
          </div>
        ) : (
          <div className="entries-grid">
            {filteredEntries.map((entry) => (
              <EntryCard
                key={entry.id}
                entry={entry}
                onEdit={handleEdit}
                onDelete={(id) => handleDeleteRequest(id, entry.service_name)}
                onCopyPassword={handleCopyPassword}
              />
            ))}
          </div>
        )}
      </main>

      {/* Modal formulaire */}
      {showEntryForm && (
        <EntryForm
          entry={editingEntry}
          onClose={handleCloseForm}
          onSave={handleSave}
        />
      )}

      {/* Modal de confirmation de suppression */}
      {deleteConfirm.show && (
        <div className="modal-overlay" onClick={handleDeleteCancel}>
          <div className="modal-content delete-confirm" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>⚠️ Confirmer la suppression</h2>
              <button className="btn-close" onClick={handleDeleteCancel}>
                ×
              </button>
            </div>
            <div className="entry-form">
              <p style={{ marginBottom: "1.5rem", fontSize: "1.1rem" }}>
                Êtes-vous sûr de vouloir supprimer <strong>"{deleteConfirm.entryName}"</strong> ?
              </p>
              <p style={{ color: "var(--text-secondary)", marginBottom: "1.5rem" }}>
                Cette action est irréversible.
              </p>
              <div className="modal-actions">
                <button className="btn-secondary" onClick={handleDeleteCancel}>
                  Annuler
                </button>
                <button className="btn-danger" onClick={handleDeleteConfirm}>
                  Supprimer
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Notification */}
      {notification && <div className="notification">{notification}</div>}
    </div>
  );
}

export default App;
