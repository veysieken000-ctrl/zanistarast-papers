# Production Security Configuration

## Amaç

Bu belge, Zanistarast üretim ortamında uygulanması gereken güvenlik yapılandırmasını tanımlar.

## Kimlik Doğrulama

- MIRA_MUDEBBIR_TOKEN yalnızca güvenli ortam değişkeni olarak kullanılmalıdır.
- Token hiçbir zaman kaynak koda eklenmemelidir.
- Token hiçbir log dosyasında gösterilmemelidir.
- Token hiçbir istemci tarafı JavaScript koduna gönderilmemelidir.

## İletişim Güvenliği

- HTTPS zorunludur.
- HTTP istekleri HTTPS'e yönlendirilmelidir.
- Güvenli başlıklar (Security Headers) etkin olmalıdır.

## GitHub

- Repository Secret kullanılmalıdır.
- Secret erişimi yalnızca gerekli iş akışlarıyla sınırlandırılmalıdır.
- Başarısız güvenlik testleri üretim dağıtımını durdurmalıdır.

## Doğrulanan Sonuçlar

- Tokensız istek → HTTP 401
- Geçerli token → HTTP 200
- Geçersiz token → HTTP 401

## Sonuç

Üretim ortamı yalnızca tüm güvenlik doğrulamaları başarıyla tamamlandıktan sonra kullanılmalıdır.


