# Mira Security Implementation Plan

## Amaç

Bu belge, güvenlik dokümantasyonunda tanımlanan politikaların teknik uygulamaya geçirilmesi için izlenecek planı tanımlar.

## Uygulama Aşamaları

### 1. HTTPS

- Üretim ortamında HTTPS zorunlu olacak.
- HTTP istekleri HTTPS'e yönlendirilecek.

Durum: Planlandı

---

### 2. Session Management

- Güvenli oturum oluşturma
- Güvenli oturum sonlandırma
- Oturum zaman aşımı

Durum: Planlandı

---

### 3. Passkey

- WebAuthn tabanlı Passkey desteği
- Yeni cihaz kaydı
- Passkey iptal mekanizması

Durum: Planlandı

---

### 4. Account Recovery

- İkinci e-posta doğrulaması
- Yeni cihaz doğrulaması
- Passkey yeniden oluşturma

Durum: Planlandı

---

### 5. Credential Rotation

- Secret yenileme
- Aktif oturumların iptali
- Güvenlik doğrulamalarının yeniden çalıştırılması

Durum: Planlandı

## Tamamlama Kriterleri

Bir uygulama tamamlanmış sayılabilmesi için:

- Kod uygulanmış olmalıdır.
- Testleri başarılı olmalıdır.
- GitHub Actions başarılı olmalıdır.
- Rasterast doğrulaması başarılı olmalıdır.
- Müdebbir onayı alınmalıdır.

## Sonuç

Bu plan, güvenlik dokümantasyonundan üretim seviyesindeki teknik uygulamaya geçiş sürecini tanımlar.


