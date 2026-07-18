# Mira Emergency Credential Rotation Policy

## Amaç

Bu belge, kimlik bilgilerinin güvenlik riski oluştuğunda güvenli şekilde yenilenmesi için uygulanacak süreci tanımlar.

## Yenileme Gerektiren Durumlar

Kimlik bilgileri aşağıdaki durumlarda derhal yenilenmelidir:

- Secret sızıntısı doğrulanırsa
- Müdebbir hesabının ele geçirildiğinden şüphelenilirse
- GitHub Repository Secret yetkisiz erişime maruz kalırsa
- Passkey güvenliği tehlikeye girerse
- Kurtarma e-postasının güvenliği kaybedilirse

## Yenileme Süreci

1. Risk doğrulanır.
2. İlgili Secret devre dışı bırakılır.
3. Yeni Secret oluşturulur.
4. Repository Secret güncellenir.
5. Eski Secret geçersiz hale getirilir.
6. Tüm aktif oturumlar sonlandırılır.
7. Gerekirse tüm Passkey kayıtları yenilenir.
8. Rasterast doğrulaması çalıştırılır.
9. Müdebbir onayıyla sistem tekrar aktif edilir.

## Doğrulanacak Maddeler

- Yeni Secret aktif
- Eski Secret geçersiz
- Kimlik doğrulama başarılı
- Yetkilendirme başarılı
- GitHub Actions başarılı

## Sonuç

Kimlik bilgisi yenileme işlemleri güvenlik olaylarından sonra mümkün olan en kısa sürede tamamlanmalı ve tüm doğrulamalar başarıyla sonuçlanmalıdır.


