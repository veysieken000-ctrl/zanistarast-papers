# Mira Müdebbir Yetkilendirme Doğrulaması

## Durum

Doğrulama başarılıdır.

## Doğrulanan bileşenler

- `MIRA_MUDEBBIR_TOKEN` GitHub Repository Secret olarak yapılandırılmıştır.
- Secret değeri kaynak koda yazılmamıştır.
- Secret değeri commit mesajlarında bulunmamaktadır.
- Secret değeri GitHub Actions günlüklerinde gösterilmemiştir.
- Mira API başarıyla başlatılmıştır.
- Tokensız korumalı istek HTTP `401 Unauthorized` ile reddedilmiştir.
- Geçerli Müdebbir tokenı taşıyan istek HTTP `200 OK` ile kabul edilmiştir.
- Manuel `Mira Auth Live` iş akışı başarıyla tamamlanmıştır.

## Güvenlik ilkesi

Gerçek Müdebbir erişim anahtarı:

- kaynak koda eklenmez,
- belgeye yazılmaz,
- commit mesajına yazılmaz,
- ekran görüntüsünde gösterilmez,
- sohbetlerde paylaşılmaz,
- günlük çıktısına yazdırılmaz.

Anahtar yalnızca güvenli secret deposundan ortam değişkeni olarak uygulamaya aktarılır.

## Rasterast değerlendirmesi

### Doğrulananlar

- Kimlik doğrulama olmadan korumalı API erişimi engellenmektedir.
- Doğru kimlik bilgisiyle korumalı API erişimi sağlanmaktadır.
- Anahtarın gerçek değeri doğrulama sırasında açığa çıkmamıştır.
- Müdebbir yetkisi uygulama seviyesinde uygulanmaktadır.

### Riskler

- Token ele geçirilirse yetkisiz erişim mümkün olabilir.
- Uzun süre değiştirilmeyen token güvenlik riskini artırabilir.
- Tokenın istemci tarafındaki açık kaynak koduna yerleştirilmesi yasaktır.

### Zorunlu önlemler

- Token yalnızca güvenli sunucu ortamında kullanılmalıdır.
- Şüpheli durumda token hemen yenilenmelidir.
- Eski token geçersiz kılınmalıdır.
- Canlı sistemde HTTPS zorunlu olmalıdır.
- Başarısız yetkilendirme girişimleri güvenli şekilde kaydedilmelidir.

## Müdebbir kararı

Bu doğrulama, yalnızca teknik kimlik doğrulama temelinin başarılı olduğunu gösterir.

Canlı yayın, kullanıcı oturumu, passkey ve kurtarma sistemi için ayrıca Müdebbir onayı gereklidir.


