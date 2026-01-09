# CoreLab Memory Bank

> Bu dosya tüm geliştirme ilerlemelerini, kararları ve planları takip eder.

---

## 🎯 Proje Özeti

**CoreLab**: Local-first, AI destekli kişisel uygulamalar için modüler framework.  
**İlk Uygulama**: Personal Memory App

---

## 🏗️ Mimari Kararlar

| Karar | Seçim | Tarih |
|-------|-------|-------|
| Core Layer | **Rust** | 2025-12-29 |
| App Layer | **TypeScript + React** | 2025-12-29 |
| Desktop Framework | **Tauri** (Rust + WebView) | 2025-12-29 |
| Database | **SQLite** (rusqlite) | 2025-12-29 |
| AI Provider | **Abstraction Layer** (Local + Cloud) | 2025-12-29 |

---

## 📋 Phase 1 — Core Foundation

### Rust Core
- [ ] Tauri proje yapısı oluştur
- [ ] SQLite database layer (rusqlite)
- [ ] Migration sistemi
- [ ] App Registry
- [ ] Event System
- [ ] AI Interface (trait-based abstraction)
- [ ] IPC commands (Tauri invoke)

### TypeScript App Shell
- [ ] React + Vite frontend
- [ ] UI Shell (Tab navigation)
- [ ] Tauri API bindings
- [ ] Type definitions

---

## 📋 Phase 2 — Personal Memory App

### Kişi Yönetimi
- [ ] Kişi CRUD (Create, Read, Update, Delete)
- [ ] Kişi listesi UI
- [ ] Pasif/Aktif durumu

### Etkileşim Kaydı
- [ ] Görüşme notu ekleme
- [ ] Kişi ile ilişkilendirme
- [ ] Zaman damgası

### Hafıza Yönetimi
- [ ] AI memory extraction
- [ ] Yapılandırılmış saklama
- [ ] Önem derecesi

### AI Önerileri
- [ ] Öneri sistemi
- [ ] Bağlamlı sorgulama

---

## 📋 Phase 3 — Iteration
- [ ] Prompt iyileştirmeleri
- [ ] UI refinements
- [ ] Performans optimizasyonları

---

## 📝 Günlük İlerleme

### 2025-12-29
- ✅ PRD incelendi
- ✅ Mimari kararlar alındı (Rust Core + TS App)
- ✅ Memory bank oluşturuldu
- ✅ **Phase 1 tamamlandı:**
  - Rust Core: database.rs, registry.rs, events.rs, ai.rs, commands.rs
  - TypeScript UI: Shell, TabNav, App.tsx
  - Vite + TypeScript config
- ✅ **Phase 2 başlatıldı:**
  - Personal Memory App UI (MemoryApp, PersonList, PersonDetail)
  - Kişi CRUD ve Görüşme Kaydı hazır
- ⏳ AI provider entegrasyonu bekliyor
- ✅ **Doğrulama Başarılı:**
  - Build & Run: Başarılı (`npm run tauri dev`)
  - Database: `corelab.db` oluşturuldu ve migrations uygulandı.
  - UI: Tarayıcı ve Electron(Tauri) içinde çalıştı.

---

## 🔧 Teknik Notlar

### Proje Yapısı
```
corelab/
├── core/                    # Rust Backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands.rs
│       └── modules/
│           ├── database.rs
│           ├── registry.rs
│           ├── events.rs
│           └── ai.rs
│
├── apps/                    # TypeScript Uygulamalar
│   └── memory/              # Personal Memory App
│       ├── views/
│       └── hooks/
│
├── src/                     # Shared Frontend
│   ├── App.tsx
│   ├── components/
│   └── lib/
│
└── memory_bank.md
```

### Tauri + Rust Avantajları
- Native performans
- Küçük bundle size (~10MB vs Electron ~150MB)
- Memory-safe backend
- Cross-platform (Windows, macOS, Linux)

### AI Abstraction Strategy
```
AIProvider trait
├── OpenAIProvider (cloud)
├── OllamaProvider (local)
└── MockProvider (test)
```

---

## ❓ Açık Sorular
- [ ] Hangi local LLM modeli kullanılacak? (Ollama: llama2, mistral, etc.)
- [ ] Vector DB entegrasyonu gerekli mi? (ileride)
