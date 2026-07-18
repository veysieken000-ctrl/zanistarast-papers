# Production Release Checklist

## Güvenlik

- [ ] MIRA_MUDEBBIR_TOKEN yalnızca Repository Secret olarak saklanıyor.
- [ ] Kaynak kodda gizli anahtar bulunmuyor.
- [ ] Log dosyalarında gizli bilgi bulunmuyor.
- [ ] HTTPS etkin.

## Yetkilendirme

- [ ] Tokensız istek → HTTP 401
- [ ] Geçerli Müdebbir tokenı → HTTP 200
- [ ] Geçersiz token → HTTP 401

## GitHub Actions

- [ ] Rust CI başarılı.
- [ ] Mira Auth Live başarılı.
- [ ] Pages Build başarılı.

## Dokümantasyon

- [ ] Güvenlik belgeleri güncel.
- [ ] Canlı doğrulama raporu güncel.
- [ ] Üretim güvenlik yapılandırması güncel.

## Son Karar

Üretim sürümü yalnızca tüm maddeler tamamlandıktan ve Müdebbir onayı alındıktan sonra yayınlanmalıdır.


