# PRD — CoreLab & Personal Memory App

## 1. Overview

**CoreLab**, kişisel kullanım odaklı, **local-first**, **AI destekli** uygulamaların üzerine inşa edileceği bir **çekirdek framework**’tür.

Bu PRD:
- CoreLab’in **çekirdek gereksinimlerini**
- CoreLab üzerinde geliştirilecek **ilk referans uygulama** olan **Personal Memory App**’in gereksinimlerini

tanımlar.

---

## 2. Goals & Objectives

### CoreLab için hedefler
- Tekrarlanan altyapı problemlerini ortadan kaldırmak
- Farklı kişisel uygulamalar için ortak zemin sağlamak
- AI entegrasyonunu standartlaştırmak
- Modüler ve genişletilebilir bir mimari kurmak

### Personal Memory App için hedefler
- İnsan etkileşimlerinden anlamlı bilgileri saklamak
- Bilgileri zaman içinde bağlamlı şekilde hatırlamak
- AI yardımıyla sohbet öncesi öneriler üretmek

---

## 3. Non-Goals

- Çoklu kullanıcı desteği
- Cloud senkronizasyon
- Sosyal ağ entegrasyonu
- Ticari dağıtım
- Otomatik karar alma veya davranış yönlendirme

---

## 4. Target User

- Tek kullanıcı
- Teknik altyapıya hakim
- Kişisel verilerini yerel tutmak isteyen
- Deneysel AI sistemleriyle çalışmayı amaçlayan

---

## 5. System Architecture (High Level)

```
CoreLab
│
├── Core Layer
│   ├── Database
│   ├── AI Interface
│   ├── App Registry
│   ├── Event System
│   └── UI Shell
│
├── App Layer
│   └── Personal Memory App
│
└── Storage
    └── SQLite (local)
```

---

## 6. CoreLab Functional Requirements

### 6.1 Database Layer
- SQLite tabanlı olmalıdır
- Migration desteği bulunmalıdır
- Core ve App’ler tarafından ortak kullanılmalıdır

### 6.2 App Registry
- Core’a bağlı uygulamalar kayıt edilebilmelidir
- Uygulamalar:
  - ad
  - versiyon
  - açıklama
  bilgileriyle tanımlanmalıdır

### 6.3 AI Interface
- AI çağrıları için ortak bir API sağlanmalıdır
- Prompt yönetimi Core’dan bağımsız olmalıdır
- AI çıktıları yapılandırılmış formatta (JSON) dönmelidir

### 6.4 Event System
- App’ler Core’a event gönderebilmelidir
- Event’ler loglanabilir olmalıdır
- Core event’lere tepki verebilmelidir

### 6.5 UI Shell
- Tab / App bazlı navigation sağlamalıdır
- App’ler UI alanı talep edebilmelidir
- Core layout kontrolünü elinde tutmalıdır

---

## 7. Personal Memory App Requirements

### 7.1 Core Integration
- App, CoreLab olmadan çalışmamalıdır
- Veriye Core Database üzerinden erişmelidir
- AI çağrılarını Core AI Interface üzerinden yapmalıdır

---

### 7.2 Functional Requirements

#### Kişi Yönetimi
- Yeni kişi eklenebilmelidir
- Kişi bilgileri düzenlenebilmelidir
- Kişi pasif hale getirilebilmelidir

#### Etkileşim Kaydı
- Görüşme notları eklenebilmelidir
- Her kayıt zaman damgası içermelidir
- Kayıtlar kişi ile ilişkilendirilmelidir

#### Hafıza Yönetimi
- AI, görüşmelerden önemli bilgileri çıkarabilmelidir
- Çıkarılan bilgiler yapılandırılmış olarak saklanmalıdır
- Bilgiler önem derecesi içermelidir

#### AI Destekli Öneriler
- Kullanıcı talebiyle öneri üretilmelidir
- Öneriler bağlamlı olmalıdır
- Otomatik aksiyon alınmamalıdır

---

## 8. Data Model (Initial)

### Persons
- id
- name
- notes
- created_at

### Conversations
- id
- person_id
- content
- context
- created_at

### Memories
- id
- person_id
- key
- value
- importance
- created_at

---

## 9. Non-Functional Requirements

### Performance
- Local kullanımda anlık tepki süresi hedeflenmelidir
- AI çağrıları async çalışmalıdır

### Security
- Tüm veriler local saklanmalıdır
- Cloud aktarımı bulunmamalıdır

### Maintainability
- Core ve App kodları ayrık olmalıdır
- Yeni App eklemek mevcut kodu bozmamalıdır

---

## 10. Milestones

### Phase 1 — Core Foundation
- Database layer
- App registry
- Basic UI shell

### Phase 2 — Personal Memory App v1
- Kişi yönetimi
- Etkileşim kaydı
- Temel AI entegrasyonu

### Phase 3 — Iteration
- Prompt iyileştirmeleri
- UI refinements
- Performans optimizasyonları

---

## 11. Success Criteria

- Yeni bir App CoreLab üzerine minimum kodla eklenebiliyor mu?
- Personal Memory App gerçek kullanımda işe yarıyor mu?
- Core değişiklikleri App’leri kırmadan yapılabiliyor mu?

---

## 12. Open Questions

- AI tamamen local mi çalışacak?
- Vector DB entegrasyonu ileride eklenecek mi?
- Plugin sistemi Core’un parçası mı olacak?

---

## 13. Appendix

Bu PRD:
- yaşayan bir dokümandır
- CoreLab evrildikçe güncellenecektir
- nihai değil, başlangıçtır
